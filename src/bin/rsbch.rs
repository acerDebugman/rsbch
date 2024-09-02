use clap::{command, Parser};
use flume::Sender;
use reqwest::ClientBuilder;
use std::time::{Duration, Instant};
use tokio::{
    runtime::{Builder, Handle},
    time::sleep,
};

pub async fn init_sinker<T>(
    rt: &tokio::runtime::Runtime,
    para: u32,
    url: &'static str,
    queue: Option<usize>,
    conn_per_para: Option<usize>,
    is_show_resp: bool,
    break_asap: bool,
) -> Sender<T>
where
    T: 'static + Send + Copy,
{
    let (tx, rx) = match queue {
        Some(v) => flume::bounded::<T>(v),
        None => flume::unbounded::<T>(),
    };

    for _ in 0..para {
        let rx = rx.clone();
        rt.spawn(async move {
            let client = match conn_per_para {
                Some(v) => ClientBuilder::new()
                    .pool_max_idle_per_host(v)
                    .pool_idle_timeout(std::time::Duration::from_secs(3600))
                    .build()
                    .unwrap(),
                None => reqwest::Client::new(),
            };
            loop {
                match rx.recv_async().await {
                    Ok(_msg) => {
                        let request = client.get(url).timeout(Duration::from_secs(10));
                        match request.send().await {
                            Ok(_response) => {
                                if is_show_resp {
                                    println!("header: {_response:#?}");
                                    println!("text: {:#?}", _response.text().await);
                                }
                            }
                            Err(e) => {
                                println!("meet error: {e:#?}");
                                if break_asap {
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("rx recv error: {e:?}");
                        let _ = tokio::time::sleep(Duration::from_secs(2)).await;
                    }
                }
            }
        });
    }

    tx
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 500)]
    parallel: u32,

    #[arg(short, long, default_value_t = 31536000)]
    duration: usize,

    #[arg(short, long)]
    qps: usize,

    /// double quotation marks:
    /// "http://localhost:8080/test/10" or "http://localhost:8080/test/10?sleep=1000"
    #[arg(short, long)]
    url: String,

    #[arg(short('c'), long, default_value = None)]
    conn_p: Option<usize>,

    #[arg(long, default_value = None)]
    queue_size: Option<usize>,

    #[arg(long, default_value_t = false)]
    show_resp: bool,

    #[arg(long, default_value_t = false)]
    break_asap: bool,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let p = args.parallel;
    let qps = args.qps;
    let duration = args.duration;
    let url: &'static str = Box::leak(Box::new(args.url));

    let rt = Builder::new_multi_thread()
        .worker_threads(32)
        .enable_all()
        .build()
        .expect("create Multi-Thread Scheduler failed！");

    let tx = init_sinker::<usize>(
        &rt,
        p,
        &url,
        args.queue_size,
        args.conn_p,
        args.show_resp,
        args.break_asap,
    )
    .await;

    let mut jds = vec![];

    let tx = tx.clone();
    let jd = rt.spawn(async move {
        let handle = Handle::current();
        for i in 1..duration + 1 {
            let tx = tx.clone();
            let curr = Instant::now();
            let jd = handle.spawn(async move {
                let handle = Handle::current();

                let jds = (0..qps)
                    .map(|q| {
                        let tx = tx.clone();
                        handle.spawn(async move {
                            match tx.send_async(i + q).await {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("all conn closed error: {:?}", e);
                                    std::process::exit(0);
                                }
                            }
                        })
                    })
                    .collect::<Vec<_>>();

                for jd in jds {
                    let _ = jd.await;
                }
            });
            sleep(Duration::from_secs(1)).await;
            let _ = jd.await;
            println!(
                "secs: {}, qps: {}, time: {:.2?}s",
                i,
                qps,
                (curr.elapsed() - Duration::from_secs(1)).as_secs_f32()
            );
        }
    });

    jds.push(jd);

    for jd in jds {
        jd.await.unwrap();
    }
    println!("done");
    sleep(Duration::from_secs(30)).await;
    rt.shutdown_background();
    Ok(())
}
