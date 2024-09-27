# Bitcoin WebUI frontend

use [eframe_template](https://github.com/emilk/eframe_template/tree/main) a template repo for [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui/).

use [ehttp](https://github.com/emilk/ehttp) query data from backend.

### Testing locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install trunk`.
3. Run `trunk serve --address 0.0.0.0` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

### Web Deploy
1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website.
3. Upload the `dist` directory to backend project folder and run backend.

# 坑

### 加载图片

按照文档增加相关的代码后，不能加载`png`的图片，必须要引入`image`依赖，还得打开它的`png features`。

### 图标被缩小

图标和标题放在`TopBottomPanel`里，一开始想着这两项水平排列，于是用`ui.horizontal`，结果图标被缩小到和标题文字同样的高度。

一开始还以为要设置图片的缩放，或者是设置`TopBottomPanel`的高度。

折腾半天发现用`ui.horizontal_centered`就可以了，水平排列并在垂直方向居中。


### 调试

`web`平台无法打印日志，只能把想看的信息放到一个`label`里显示出来。

`app`会把应用数据保存在浏览器的本地存储中，如果数据结构有修改，记得先清除一下本地存储。

应用数据中如果有`json`字符串，保存到浏览器的本地存储中会被转移，再次读出来之后就解析不了了。如果有这种情况，建议注释掉`app`中的`save`函数。

`trunk`经常崩溃。。。

### 代理服务器

开发时如果要调用别的`api server`会遇到跨域问题。

可以用代理服务器解决，`trunk`包含这个功能，还可以`rewrite`路径，添加`header`。

如果调用的`api server`有`basci auth`，也可以写在代理服务器配置里，避免写死在前端代码中。

以上用法可以参见本项目的`Trunk.toml`。

### 定时刷新

一开始想后台起个线程定时更新数据，后来发现`wasm`里可以用`std::thread`，但是在`web`平台运行不了。

后来发现用`request_repaint_after`就可以了。

鼠标一动就刷新的特别快。

### 文字无法复制

打开`accesskit`也不行。

可能是用`label`的问题？不知道换其他组件是否可以解决？

但是官方的例子中也是用的`label`，但是它那里就可以复制，不知道为什么？

### 布局

* 图标和标题现在是紧贴左上角的，想要前面有点空白，还没找到简洁的方法。

* 想把图标和标题放到`TopBottomPanel`中间也没找到办法，官方`layout`例子可以把标题居中，但是想把图标和标题水平排列一起居中没找到办法。

* `CentralPanel`中`horizontal_wrapped`似乎不起作用，`wrapped`按照文档说法应该是能自动折行的，但是我调窄屏幕分辨率还是会被截断。

* 不能自适应布局，屏幕换成手机屏幕分辨率，组件并没有自动缩小。


