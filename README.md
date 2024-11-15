<div align="center" alt="Kokoro">
  <a href="https://www.kokoro-rs.dev"><img src="https://github.com/BERADQ/kokoro-rs/assets/78293733/57a6178e-186f-4526-8ff9-52dd88712daa"></img></a>

  [![docs.rs](https://img.shields.io/docsrs/kokoro)](https://docs.rs/kokoro/latest/kokoro/)
  [![Crates.io Version](https://img.shields.io/crates/v/kokoro)](https://crates.io/crates/kokoro)
  ![Crates.io License](https://img.shields.io/crates/l/kokoro)

  <p>Kokoro is a Rust-based pluggable-framework that prioritizes memory safety, performance, and stability to empower the creation of highly decoupled applications.</p>

</div>

<br/>

## 定义

kokoro 项目经历了多次演进，从基于插件的模式设计框架到元框架，最后又被修改为插件模式库(~~云原生单例~~)。

它的主要目标是为 Rust 应用程序提供插件模式的支持，使得各种功能之间可以基本解耦，实现模块化的编程模型。

最开始我就选择遵循 KISS 原则，我的软件仅仅只做单一的且它该做的事。


- simple: 仅用于支持插件模式设计。
- stupid: 不去做任何 front 该做的事情，只做好基础的 framework。不定义接口，不固定特性。

## 选型

市面上有多种插件模式的技术解决方案，

例如嵌入式脚本、动态链接库、基于标准输入输出的应用程序子进程、基于 IPC 的…等等。

这些方案都有其优缺点，kokoro 项目的目标是选择一种简单且合适的解决方案。

kokoro 的选型则为混合方案，该方案选用了以下几个名词用于理解：

1. Primary: 主程序。
2. Planet: 直接由主程序都调用的 wasm component model 插件。
3. Satellite: 由插件启动的子进程通过 IPC 进行数据交。

### Primary

Primary 是主程序，其主要功能是管理插件。

难点：

1. 接口版本管理。
2. 配置管理。
3. 依赖管理。

Kokoro 是 KISS 原则下的产物，我们不会管理配置和接口版本。

但会提供最令人头疼的依赖关系(版本除外)管理解决方案。

并且接口声明推荐使用 [wit](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md)。

### Planet

Planet 是插件，遵循单例模式，主要通过接口供 Primary 调用。

格式: wasm component model

接口声明: wit

难点:

- 插件与插件间的通信。

### Satellite

Satellite 是插件的子进程，主要通过 IPC 进行数据交换。

Satellite 的工作方式较为特殊且低效，对于数据流的形象的描述如下：

Planet -> Primary -> Satellite -> Planet -> Primary

在这个过程中会顺序进行：

wasm host 调用 -> IPC 通信 -> wasm host 调用返回 -> wasm host 调用

~~数据流格式则为了性能考量选用 Bincode。~~

wit 定义的数据格式满足以下条件：

- 透明
- 无依赖
- 是独立的，没有堆，没有指向外部源的指针
- 有统一且稳定的内存表示
- 不使用指针来管理内部结构

很明显，Satellite 被作为可选特性。

主程序可以选择支持与否，如若主程序支持，则插件可选使用 Satellite 用于密集计算任务/系统原生操作。

基本的逻辑和网络操作文件操作均可通过 wasi 进行。

Satellite 主要是为副作用而生的，所以不鼓励使用。

### 依赖管理

依赖分为两种：

1. 可选依赖。
2. 必选依赖。

在插件启动时，会检查依赖是否满足，不满足则推迟载插件，满足则直接加载插件。

选用可选依赖的插件也会推迟加载，但依赖不存在不会终止加载。

插件与依赖间通信的方式则为插件间的通信。

### 插件与插件间的通信

wasm host 调用 -> wasm component model 调用 -> wasm host 调用返回

使用 wit 声明方法/函数签名。
