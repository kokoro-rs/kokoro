# Kokoro
关于 Kokoro 这个名字的由来，一开始是为了致敬 JavaScript 库 cordis。

但是 Kokoro 发展至今，已经脱离了 cordis 的思想与概念。

所以我们更加希望你将 Kokoro 作为“心”来解读。

我们希望 Kokoro 能够作为高解耦的应用程序的心脏，而不单纯是一个发布订阅模式框架。

<br/>

## 关于定义
在上一部分已经提到了，Kokoro 是为了构建高解耦的应用程序而存在的，那么我们对他的定义即使如此。

Kokoro 自身的设计在于抽象，其更像是一个编程范式，而不是一个库。

在 Rust 中，我们可以为几乎一切的类型去实现自己的特征。在 Kokoro 中，我们利用这个特性，将 `Context` 定义为可扩展的上下文。

> 在 `Context` 上实现方法，意味着扩展当前上下文的功能。

你可以将自己的类型实例使用 `Context` 实例包装，进而利用 `Context` 上独有的方法。并且 `Context` 是可在线程间转移且可复制的。

这意味着你的类型需要 `Arc` 包装

当 `Context` 解引用时，他会变为被包装类型实例的引用，所以在被 `Context` 扩展的同时，实例本身也是可访问的。

```rust
struct MyStruct;
impl MyStruct {
    fn hello_kokoro() {
        println!("Kokoro yyds");
    }
}
let context = context.with(Arc::new(Scope::create(Arc::new(MyStruct))));
context.hello_kokoro();
```

由此我们可以看出，`Context` 作为一个扩展的携带者，为你自己的类型进行了扩展，这也是一种理解的方式。

同样 `Context` 不止作为扩展者扩展其他类型，其自身也具有一个类型实例伴随其终生。

```rust
struct Global;
struct MyStruct;
let scope = Scope::create(Arc::new(MyStruct));
let context = Context::create(Arc::new(scope), Arc::new(Global));
```

当 `Context` 当中的 `Global` 用于携带 `Context` 自身扩展中所需要的类型实力。

就像 `kokoro-flume-channel` 中的 `MPSC` 存储了 `sender`，`receiver` 和 `runner` 一样。

在泛型中，我们也将 `Global` 称为 `Mode` 正因为其代表着 `Context` 在何种模式下工作。

## 关于发布订阅
一开始 Kokoro 是作为发布订阅模式框架而出现的，我们对 kokoro 定义及实现上的改变并不会影响其作为发布订阅模式框架来使用。

我们有官方包 `kokoro-flume-channel` 来为 Context 实现 Mode MPSC，同时 MPSC 也是可扩展的且遵循 `Context` 的设计原则。

## 定位问题
可以看出，`kokoro-core` 本身极小甚至缺少默认实现，如果需要用到，则按照相同范式再次编写一个即可。

你说的没错，`kokoro-core` 的存在感极低。并且好像其存在没什么意义，甚至会有人说过度抽象会增加开发成本。

也是正确的，我不否定单独使用 `kokoro-core` 会徒增开发成本。并且会认可你的看法非常到位。

所以让我们回到问题的本身，`kokoro-core` 的定位是什么，或者说其意义在于？

关于这个问题我有两个回答：

1. 推广这个范式或模式。
2. 为我自己定下一个规范。

这个范式主要是为了可扩展，当没有统一的类型时，可扩展这个目标或许就并不好实现了。

并且我们一直讨论的是 `kokoro-core`，当你组合自己喜欢的库来合成自己需要的 Kokoro 时，你就会发现其用法的多样性。

比如将 `kokoro-flume-channel` 与 `kokoro-default-impl::thread` 组合，你就会得到类似 ros 的效果。

比如你不想要自己实现动态插件机制或与插件间沟通的能力，你就可以组合 `kokoro-dynamic-plugin` 与 `kokoro-flume-channel` 与 `kokoro-default-impl::plugin` 实现一个可动态扩展的应用程序，甚至比肩 vscode 这种使用动态语言编写的应用程序的扩展能力。

我们不止希望如此，我们未来还会推出更多扩展实现，让 Kokoro 成为愿景中的 **元框架**。

当然，你也可以编写自己的扩展实现，同时也可以以插件的形式编写静态/动态链接库。
