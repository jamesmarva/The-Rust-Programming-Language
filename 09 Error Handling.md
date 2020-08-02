Rust’s commitment to reliability extends to error handling. Errors are a fact of life in software, so Rust has a number of features for handling situations in which something goes wrong. In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile. This requirement makes your program more robust by ensuring that you’ll discover errors and handle them appropriately before you’ve deployed your code to production!
Rust 的稳定性的体现也在错误处理上也得到了不错的体现。错误(Errors) 在整个软件生命周期中是必然存在的。
Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error. This chapter covers calling panic! first and then talks about returning Result<T, E> values. Additionally, we’ll explore considerations when deciding whether to try to recover from an error or to stop execution.


# 1 Unrecoverable Errors with panic!

Sometimes, bad things happen in your code, and there’s nothing you can do about it. In these cases, Rust has the panic! macro. When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit. This most commonly occurs when a bug of some kind has been detected and it’s not clear to the programmer how to handle the error.
有的时候，代码里面会发生一些你意想不到的错误，但是你面对这些错误却无能为力。在这些场景下，Rust 有一个洪函数 `panic!` 。在这个函数执行之后，程序中就会输出失败的消息，并且释放栈和堆的内存，然后退出程序。最常见的情况就是的遇到某个错误，但是开发者不知道要如何处理这个错误。


> ### 退出堆栈，或者 终止 `panic` 的结果
> By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. But this walking back and cleanup is a lot of work. The alternative is to immediately abort, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding` panic = 'abort'` to the appropriate [profile] sections in your Cargo.toml file. For example, if you want to abort on panic in release mode, add this:
> ```rust
> [profile.release]
> panic = 'abort'
> ```

Let’s try calling panic! in a simple program:
### 1.1 Using a panic! Backtrace

```rust
fn main() {
    panic!("crash and burn");
}

```
When you run the program, you’ll see something like this:

当你运行了上的代码，你会在命令行里看到下面这段：
```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```


# 2 Recoverable Errors with Result
大多数的错误是没有充分的理由让整个程序停止运行的。有时候一个函数调用的失败，你是可以很快找到原因，并且解决的。比如，你想要开启一个库文件，但是操作因为文件不存在而失败，那么你需要的是创建文件而不是停止整个程序。
在第2章中提到的“用 `result` 类型来处理是可能出现的失败”，里面粗出现的结构体 `result` 有两个变量。
```rust
#![allow(unused_variables)]
fn main() {
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
```
`T` 和 `E` 是泛型的参数类型：在第10章中，会有更多的讨论 泛型(generic type)的内容。现在你只要知道的就是，`T`代表的数据类型的值会在的随着变量 `OK`返回。`E` 代表的数据类型值会随着的变量 `Err` 在运行失败的情况下返回。由于 `Result` 有这些泛型参数，我们可以用 `Result` 类型和标准库中的函数在很多不同的错误的情况下处理错误。

当调用函数的时候，需要考虑到失败的情况，所以需要返回 `Result`。在代码9-3 的功能就是尝试打开文件。
```
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
}
```
↑ 代码 9-6 打开文件
要如何知道 `File::open` 返回一个 `Result` 对象？







### 2.1 Matching on Different Errors
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```
Listing 9-5: Handling different kinds of errors in different ways

### 2.2 Shortcuts for Panic on Error: unwrap and expect



### 2.3 Propagating Errors



##### 2.3.1 A Shortcut for Propagating Errors: the ? Operator





##### 2.3.2 The ? Operator Can Be Used in Functions That Return Result


# 3 To panic! or Not to panic!

### 3.1 Examples, Prototype Code, and Tests


### 3.2 Cases in Which You Have More Information Than the Compiler


### 3.3 Guidelines for Error Handling



### 3.4 Creating Custom Types for Validation


# 4 Summary


