# usage
## rsbch 压测工具
```
$ ./target/release/rsbch -help
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
-p 是并行度，在服务端能力够的情况下，如果qps打量上不来，建议提高平行度，默认是500;  
-d 是运行多长时间，单位是秒


使用例子，配合下面的web端工具：
打100qps:
```
./rsbch -q 100 -u "http://localhost:8888/test/10"
```
打10s的100qps:
```
./rsbch -d 10 -q 100 -u "http://localhost:8888/test/10"
```
打10s的100qps,且每次服务相应请求休息200ms
```
./rsbch -d 10 -q 100 -u "http://localhost:8888/test/10?sleep=200"
```

## web测试端
可以启动内置的rangeweb做测试:
```
# 默认开启8080
./target/release/rangeweb --port 8888  
```

```
$ ./target/release/rangeweb --help
Usage: rangeweb [OPTIONS]

Options:
  -p, --port <PORT>  port number
  -h, --help         Print help
  -V, --version      Print version
```

该web项目内置提供 /test/$n API接口:
1. $n 表示返回的response的字节数，
2. 该接口支持 sleep query参数, sleep是让服务请求休息多少毫秒(ms)，设置时间不建议超过1s
使用如下：
```
# 返回10字节
http://localhost:8888/test/10
# 返回1024字节
http://localhost:8888/test/1024
# 返回100000字节
http://localhost:8888/test/100000

# 返回1024字节, 且每次服务相应请求休息 10ms
http://localhost:8888/test/1024?sleep=10
# 返回1024字节, 且每次服务相应请求休息 200ms
http://localhost:8888/test/1024?sleep=200
```



# TODO
1. 参数组合校验  
2. reqwest的连接池管理能力较弱, 需要优化

