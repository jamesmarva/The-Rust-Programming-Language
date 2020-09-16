1972年， Edsger W. Dijkstra 的 文章“谦虚的程序员”(The Humble Programmer)里头说：“程序测试是发现bug的一种非常有效的方式，但是还是很难发现程序所存在的问题(发现bug还是有难度)”但是这并不意味着我们不要尽可能多地测试。

程序的正确性是我们的代码按照我们预期运行的程度。Rust 的设计高度关注程序的正确性，但是正确性是很复杂而且很难证明的。Rust的类型系统肩负很大的负担，但是类型系统不能捕获每种错误。Rust支持用编写自动化测试。

举个例子，比如我们编写了一个名字是 `add_two` 的函数，这个函数的功能就是给传入的参数增加2。这个函数签名有个整型的参数，然后返回一个整型的结果。当我们实现和编译这个函数的时候，Rust会继续所有的类型检查，借用检查，以确保我们不会传入各种类型不对的值，比如`String`；以及无效的引用。但是Rust无法检查这个函数是否可以准确的达到我们的预期，比如说返回的是加2，而不是返回加10，或者减50！这个就是需要我们去测试的地方。


# 1 如何写测试(How to Write Tests)
测试就是验证是否非测试的函数是否的按照预期的方式来工作。测试函数的主体通常有三个操作：
1. 设置所有的所需要的数据和状态
2. 运行你要测试的代码
3. 设置断言来判断结果是否是你所期望的。

让我们来看看 Rust 装门为编写测试代码所提供的功能，这些功能包括 `test` 属性，一些宏函数，以及 `should_panic` 属性。
## 1.1 测试功能的剖析(The Anatomy of a Test Function)
简而言之，测试用例在Rust是一种特别的，带着`test`属性注释的函数。属性是Rust代码段的元数据 ，一个例子就是我们在第5章将`derive`属性和结构体一起使用。要转换一个函数变成一个测试函数，就要在关键字 `fn` 的上一行新增一个注释 `#[test]`。当使用 `cargo test` 命令运行代码测试的时候，Rust就会构建一个二进制的测试运行文件，这个文件可以运行带有测试属性注释的函数，并且报告每个测试函数是否通过或者失败。

当我们用 Cargo 创建一个新的 library 项目的时候，测试模块就会被自动生成。这个模块会帮助我们编写测试用例，所以你就不必每次启动新的项目时候都去测试构建测试的模块了。你可以根据你的需要添加任意数量的其他测试函数和测试模块。

我们用一些测试的模板来生成一些测试代码进行试验来探索测试的功能。然后我们将会编写一些真实的测试例子，这些例子调用我们写的某些代码，然后断言这些代码的行为是否正确。

```shell
$ cargo new adder --lib
    create library `adder` project
$ cd adder
```
*src/lib.rs* 的文件的内容看起就像代码11-1 一样
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```
代码11-1 用 `Cargo new` 自动生成的代码

现在，让我先忽略前两行，然后专注在函数的代码上，先看看函数是如何工作的。注意，在函数的签名的上一行的`#[test]`的注释，这个注释就表明了这个是个测试函数，以及，测试的运行程序会知道这个函数是个测试用例。我们也可以在测试模块有非测试的函数的代码，来帮助我们创建常规的操作，所以我们才需要用 `#[test]` 属性来帮助我们指示谁才是测试函数。

这个函数代码体用宏函数 `assert_eq!` 来确定是否 2 + 2 等于4。这个断言是用来作为典型的测试模本的示例。让我们来运行一下看看是否这个例子可以正常与西宁。

```shell
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.57s
     Running target/debug/deps/adder-92948b65e88960b4

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


```






# 2 控制测试用例如何运行(Controlling How Tests Are Run)




# 3 测试的组织结构(Test Organization)


# 4 总结 (Summary)
