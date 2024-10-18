# bitcoin-webui

为`bitcoin`节点（`bitcoind`）提供一个简单的`webui`，展示节点相关的一些信息。

# 架构

采用前后端分离架构。

`frontend`为前端工程，使用`egui`图形框架，使用`ehttp`调用后端的`API`接口周期获取数据并展示。

`backend`为后端工程，一方面作为`web`服务器`host`前端工程和静态资源，另外一方面作为`proxy`调用`bitcoind`节点的`rpc`接口获取节点信息，并对信息进行重新封装提供给前端。


