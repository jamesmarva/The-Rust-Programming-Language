安全而且高效地完成*并发编程（concurrent programming）*是RUst 另一个主目标。*并发编程（Concurrent programming）*，让不同的不同部分的程序相互独立运行，*并行编程（Parallel programing）*代码的不同部分同时执行，这两个概念随着计算在多处理器上利用越来越搞笑变得越来越重要。可惜，从古到今，并发和并行依然有困难以及时常有错误，Rust想改变这一点。


1 如何创建线程来同时运行不同的代码段
2 


# 1 使用线程同时运行代码（Using Threads to Run Code Simultaneously）
## 1.1 使用 spawn 创建新线程（Creating a New Thread with spawn）
```rust

```

## 1.2 Waiting for All Threads to Finish Using join Handles


## 1.3 Using move Closures with Threads
```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

```
$ cargo run
   Compiling threads v0.1.0 (file:///projects/threads)
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
 --> src/main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {:?}", v);
  |                                           - `v` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:6:18
  |
6 |       let handle = thread::spawn(|| {
  |  __________________^
7 | |         println!("Here's a vector: {:?}", v);
8 | |     });
  | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0373`.
error: could not compile `threads`.

To learn more, run the command again with --verbose.
```


```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}

```

# 2 用消息在线程指尖传递消息（Using Message Passing to Transfer Data Between Threads）
有个越来越流行的用来保证并发安全的方式：*消息传递(message passing)*，线程或者使用者(actors)通过发送带有数据的消息来进行交流沟通。这个思想是来自 “Go 语言编程文档” 的slogan，不要通过共享内存来交换数据，用传递消息的是让来交换数据（Do not communicate by sharing memory; instead, share memory by communicating.）

Rust 实现消息传递的主要工具是 *通道（channel）*。





