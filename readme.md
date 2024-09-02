# usage
```
rsbch$ ./target/release/rsbch -help
Usage: rsbch [OPTIONS] --qps <QPS> --url <URL>

Options:
  -p, --parallel <PARALLEL>      [default: 500]
  -d, --duration <DURATION>      [default: 31536000]
  -q, --qps <QPS>                
  -u, --url <URL>                double quotation marks: "http://localhost:8888/test/10" or "http://localhost:8888/test/10?sleep=1000"
  -c, --conn-p <CONN_P>          
      --queue-size <QUEUE_SIZE>  
      --show-resp                
      --break-asap               
  -h, --help                     Print help
  -V, --version                  Print version
```

可以启动rangeweb做测试:
```
# 默认开启8080
./target/release/rangeweb --port 8888  
```


# 问题
1. 参数组合校验  
2. reqwest的连接池管理能力较弱, 需要优化
