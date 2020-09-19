这个章节就是对前面你所学的技能的一次回顾，以及对标准库的特性的一次探索。


# 1 获取命令行的参数(Accepting Command Line Arguments)
让我们用命令 `cargo new` 创建一个新的项目。我们把我们的项目称之为`minigrep`，以便来区分 你的系统里的`grep` 工具。
```shell
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```
第一个任务就是让项目 `minigrep` 接受两个命令行的参数：一个文件名和用于搜索的字符串。也就是说，我希望在运行的时候，用`cargo run`，一个文件名，一个字符串用于搜索，如下所示：
```rust
$ cargo run searchstring example-filename.txt
```
目前，由 `cargo new` 生成的代程序不能处理我们提供的参数。`cargo.io` 上某些现有的库可以帮助我们处理这些命令行的参数，但是因为你只是学习这个概念，所以请你自己实现这些功能。

## 1.1  读取参数(Reading the Argument Values)
为了能让 `minigrep` 可以读取传递给他们命令行的参数，我们需要Rust 标准库的提供的函数，就是 `std::env::args`。这个函数将会返回我们传递给 `minigrep` 命令行参数所组成的迭代器。我们将会在第13章中全面介绍迭代器。目前，你只要知道两个有关于迭代器(iterator)的两个细节：迭代器会产生一系列的值，我们可以在迭代器上调用 `collect` 方法来将其转为一个集合(collection)，比如 vector，其中就包含迭代器生成的所有的元素。

代码12-1 中的代码，程序就可以读取传递给 `minigrep` 的所有的命令行的元素了，而且可以把这些元素收集到 vector中。

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```
代码12-1 收集所有的命令行参数到vector，然后输出他们

首先，我们用 `use` 语句把 `std::env` 模块引入作用域中，以便可以用 `args` 函数。注意，这里的 `std::env::args` 函数是嵌套在两个层级的模块中。就像我们在第7章中所讨论的那样，如果所需的函数是在嵌套在多个层级的模块中的额，那么通常是将其的父模块引入作用域中而不是直接把函数引入作用域中。这样，我们就可以轻松的使用 `std::env` 中的其他的函数了。比起 仅仅使用`args` 相比，使用`env::args`不会那么模凌两可，因为直接调用 `args`函数会让人误以为是当前模块里定义的函数。

> # The args Function and Invalid Unicode
> 注意，这个 `std::env::args` 在参数的包含非法 unicode 的时候会报错。如果你的程序需要传入一些非法字符作为参数，请用 `std::env::args_os` 来代替。

在 main 函数的第一行，我们用 `collect` 来将 迭代器的值转为 vector，里面都是迭代器里的值。我们可以用`collect`来创建多种类型的集合，因此我们需要显示注释 `args` 的类型，以指定我们想要一个字符的vector。尽管我们很少需要在Rust中去注释类型， `collect` 是一个你需要经常需要注释的函数，因为 Rust 无法推断出你想要的集合的类型。

最后，我们调用格式化程序，`:?`来打印 vector。让我们来先运行无参，然后在运行有参的情况：
```shell
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/minigrep`
["target/debug/minigrep"]
```

```shell
$ cargo run needle haystack
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/minigrep needle haystack`
["target/debug/minigrep", "needle", "haystack"]
```
请注意，vector的第一个元素是 `target/debug/minigrep`，这个是我们而精致的额文件的名称。 这个和 c 语言的参数列表是一致。


## 1.2 用变量来保存参数 (Saving the Argument Values in Variables)
打印vector的参数说明这个程序的可以访问命令行传给程序的参数。现在我们需要把两个参数保存在程序的变量中，以便程序的剩下的部分可以使用这两个值。
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```
代码12-2 创建用来保存参数的变量

正如我们在打印 vecotr的时候看到的，程序的名称占用了 vector的第一个值 `args[0]`，所以我们的要从 索引的 1 开始。 `minigrep` 的第一个参数就是我们需要搜索的字符串，因此我们吧第一个参数的引用放入变量 `query`。第二个参数是个文件的名字，所有我们将文件的名字的引用放到一个变量中。

我们临时打印这些变量的值，以证明代码是按照预期工作的。让我们用参数`test`和 `sample.txt` 来运行程序。
```rust
$ cargo run test sample.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep test sample.txt`
Searching for test
In file sample.txt
```
程序跑起来了。我们需要吧参数值保存到正确的变量中，稍后我们将会添加一些错误来处理一些潜在错误，比如用户在不提供的参数的时候，会出错。不过现在我们将会忽略这种情况，而改为继续讨论读取文件的功能。
# 2 读取文件(Reading a File)
现在我们将会去读取指定命令行中的文件名的文件。

```rust
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us - don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```
代码12-3：一首来自Emily Dickinson的诗

