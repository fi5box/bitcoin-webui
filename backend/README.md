# Bitcoin webui backend

* router /          前端工程和静态资源 
* router /bitcoin   获取bitcoin节点信息

# 配置

配置文件例子参见`example/config.toml`

* port                           后端监听端口，会绑定到所有的网络接口
* request_timeout                请求超时时间，单位是秒
* static_dir                     前端工程和静态资源目录
* rpc_url                        bitcoind节点的rpc url
* rpc_username                   bitcoind节点的rpc username
* rpc_password                   bitcoind节点的rpc password
* update_interval                更新bitcoin节点信息的间隔，单位是秒

# 用法

```
$ RUST_LOG=tower_http=debug,info ./target/debug/backend run -c example/config.toml
[2024-09-27T12:15:28Z INFO  backend] start server ...
[2024-09-27T12:16:19Z DEBUG tower_http::trace::make_span] request; method=GET uri=/index.html version=HTTP/1.1
[2024-09-27T12:16:19Z DEBUG tower_http::trace::on_request] started processing request
[2024-09-27T12:16:19Z DEBUG tower_http::trace::on_response] finished processing request latency=0 ms status=200
[2024-09-27T12:16:28Z DEBUG tower_http::trace::make_span] request; method=GET uri=/bitcoin version=HTTP/1.1
[2024-09-27T12:16:28Z DEBUG tower_http::trace::on_request] started processing request
[2024-09-27T12:16:28Z DEBUG tower_http::trace::on_response] finished processing request latency=0 ms status=200
```

```
$ curl localhost:8000/bitcoin
{"latest_blocks":863068,"difficulty":"88.40 TH/s","synchronized":"100.00%","disk_usage":"4.86 GB","prune_mode":"Yes","connections":10,"connections_in":0,"connections_out":10,"mempool":"279.49 MB","hash_rate":"559.01 EH/s"}

$ curl localhost:8000/index.html
<!DOCTYPE html>
<html>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
```
