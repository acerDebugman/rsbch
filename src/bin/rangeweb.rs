use actix_web::{get, web, App, HttpServer, Responder};
use clap::{arg, command, value_parser};
use rsbch::utils;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
struct QueryParams {
    sleep: Option<u64>,
    show: Option<bool>,
}

impl Display for QueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "QueryParams {{ sleep: {:?}, show: {:?}}}",
            self.sleep, self.show
        ))
    }
}

#[get("/test/{n}")]
async fn greet(n: web::Path<u32>, query: web::Query<QueryParams>) -> impl Responder {
    if query.show.is_some() && query.show.unwrap() == true {
        println!("path n: {}, query params: {}", n, query);
    }
    if query.sleep.is_some() {
        let _ = tokio::time::sleep(std::time::Duration::from_millis(query.sleep.unwrap())).await;
    }
    utils::text::random_n_ascii_chars(*n.as_ref())
}

// #[actix_web::main] or #[tokio::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let matches = command!()
        .arg(
            arg!(
                -p --port <PORT> "port number"
            )
            .required(false)
            .value_parser(value_parser!(u16)),
        )
        .get_matches();

    let port: u16 = match matches.get_one::<u16>("port") {
        Some(v) => *v,
        None => 8080,
    };

    HttpServer::new(|| App::new().service(greet))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}