Rust’s commitment to reliability extends to error handling. Errors are a fact of life in software, so Rust has a number of features for handling situations in which something goes wrong. In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile. This requirement makes your program more robust by ensuring that you’ll discover errors and handle them appropriately before you’ve deployed your code to production!

Rust 的稳定性的体现也在错误处理上也得到了不错的体现。错误(Errors) 在整个软件生命周期中是必然存在的。

Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error. This chapter covers calling panic! first and then talks about returning Result<T, E> values. Additionally, we’ll explore considerations when deciding whether to try to recover from an error or to stop execution.


# 1 Unrecoverable Errors with panic!
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

1 可以API文档(https://doc.rust-lang.org/std/index.html)

2 可以问编译器

如何问编译器，只要随便给 `f` 声明一个变量，并且尝试编译，编译器就是会告诉我们返回类型不匹配。错误信息会告诉我们函数 `File::open` 的函数的真正返回类型。因为我们没提前知道了 `File::open` 的返回类型肯定不是 `u32`，所以就有下面这个代码：
```rust
use std::fs::File;

fn main() {
    let f: u32 = File::open("hello.txt");
}
```
尝试编译之后就有以下的错误信息：
```rust
error[E0308]: mismatched types
 --> src\main.rs:5:18
  |
5 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `std::result::Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `std::result::Result<std::fs::File, std::io::Error>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `listing_09_03`.

To learn more, run the command again with --verbose.
```
这个信息告诉我们函数的 `File::open` 就是 `Result<T, E>`，泛型的类型的参数`T`在此处就是成功的调用后的返回类型：`std::fs::File`。如果调用中出现立刻错误，那么就会返回 `std::io::Error`类型。

这个返回类型就表示在如果 `File::open` 调用成功了，那么就会返回文件的句柄，你可以用这个句柄对文件进行读取和写入操作。当然，也许函数也会调用失败，比如文件也许不存在，或者我们没有权限使用文件。`File::open` 函数需要一种方途径告诉我们到底这个函数调用成功还是失败了，以及同时给我们成功后的文件句柄，或者告诉我们出现错误的信息。这就是枚举类型 (enum) 的用武之地了。

如果成功，那么就返回就是Ok的实例，这个对象就包含句柄了。
如果失败，那么就包含 Err 的实例，这里就有所有的错误信息。

我们需要把处理不同的情况的值添加到 代码 9-3 中，整体代码如下：
```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```
↑ 代码 9-4：用 `match` 表达式来处理 `result` 变量

注意，就像 `Option` 结构体一样，结构体 `Result` 是被纳入了作用域中的(其实就死活不用引入才能使用的意思)，所以我们不用得在使用 `Ok` 和 `Err` 变量之前指出 `Result::`.

这里，我们告诉 `Rust`，如果结果为 `Ok`，那就就返回`Ok` 里的值，并且，把文件的句柄分配给变量 `f`， `match`之后，我们就可以用变量 `f` 来对文件进行读取和写入。

另一部分的代码就是处理 `File::open` 出现的错误的情况，在这个例子当中，我们用了宏函数 `panic!` 进行处理错误的情况。如果这个时候，你的目录下面没有 `hello.txt` 这个文件，那么就会报错。
```rust
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/error-handling`
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```
这个输出告诉了我们错误的原因是什么。


### 2.1 匹配不同的错误类型 (Matching on Different Errors)
不管为什么 `File::open` 因为什么出现错误，程序都会出现用宏函数 `panic!` 来报错。此时，我们想要根据不同错误来进行不同操作。比如，如果返回的是文件没找到的错误，那么就创建文件，并且重新返回，如果是别的错误，那么就调用函数 `panic!` 来返回。

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
↑ 代码 9-5 根据不同的错误类型分配不同的错误方式。
`File::open` 会返回在变量 `Err` 中的数据类型是 `io::Error`，这个是标准库提供的结构体。我们可以用这个结构体的 `kind` 方法来的到错误得 `io:ErrorKind` 类型的值。结构体 `io:ErrorKind` 是标准库提供的，有一个变量来表示不同的因为 `io` 操作导致的错误类型。在这里使用的类型是 `ErrorKind::NotFound`，这类型的错误就表示我们尝试打开的文件不存在的。

检查内部匹配项的条件就是，函数 `error.kind()` 返回值是否就是的结构体 `ErrorKind` 的变量 `NotFound`。如果正好匹配上了，那么就表示没有文件，那么我们就要创建文件。可是因为 `File::create` 也有可能失败，我们需要在 match 匹配的表达式的内部再次使用一次 match 表达式。当无法创建文件袋饿时候，就会输出这个错误信息。外部那个match的不变，所以程序会因为没有文件而报错。

此处有很多 match 表达式，Match是非常有用的，同时用起来也比较繁琐。在第13章中的将会学到关于闭包(closures) ；`result<T, E>` 中有很多接受一个闭包对象，并且用 `match` 表达式来实现。用这些方法可以让你的代码变得更加简洁。很多经验丰富的 Rustacean　会编写更加简洁的代码　代码 9-5 ：
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```
尽管以上的这代码的是实现的功能和 代码 9-5 相同个，但是他它不包含任何的匹配表达式，并且更加容易阅读。这里例子在阅读完 第13章之后的。在处理错误的时候，有很多方法可以用来去除 `match` 的匹配表达式。

### 2.2 简单的使用 Panic 来处理错误(Shortcuts for Panic on Error: unwrap and expect) 
使用 match 表达式使用起来效果很好，但是有点冗长，并且不一定可以很好的表达需要传达的意图. `Result<T, E>` 需要在其定义之上定义许多辅助方法来执行各种任务。其中一个方法就是 `unwrap`， 一种快捷方法来实现和 `match` 表达式一样的功能。如果result的结果是 `Ok` 变量，upwrap 方法就会返回 Ok 变量里的值。如果出现了错误，那么就会调用 宏函数 `panic!` 。
```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
如果在运行程序的时候没有 *hello.txt* 文件的话，就会看到调用 `panic!` 函数出现的错误信息。
```rust
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```
另一个方法 `expect`，这个和 `upwarp` 的使用方式有点像，但是它可以让开发者指定的调用 `panic!` 错误的信息。使用 `expect` 方法 而不是用 `unwrap` ，就是因为 `expect` 可以提供更好的错误信息以帮助我可以更简单的跟做错误的信息，使用方式如下。
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

```
传递给 `expect` 方法的参数会变成我们指定的错误信息，而不是原来的 默认的错误信息。
```rust
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```
得益于我们指定的错误信息，所以我们可以更快的定位到错位的位置。如果我们使用 `unwrap` 的话，那么就花很多时间在定位错误上。因为所有的 `unwrap` 会返回同样的 错误。

### 2.3 错误的传播 (Propagating Errors)
当我们的调用一个函数的时候，这个被调用的函数是有可能会出现错误的情况的，所以我们有可能会得到一个错误的返回值，开发者可以根据错误来进行相应的操作（是捕获然后日志记录，还是继续往上层抛）而不是一概让函数自己的吧错误给处理了。这种函数抛出错误的行为被称为 “错误的传播(Propagating Errors)”，这让代码有了更多的控制权。在这里有更多信息传递或者提示错误的逻辑处理方式，而不是仅仅在函数中处理错误。

举个例子，在代码9-6 中，就展示啦一个函数，这个函数实现了读取一个文件中的用户名。如果这个文件不存在或者程序没有读取文件的权限，那么调用这个函数的代码就会得到一个错误。
```rust
#![allow(unused_variables)]
fn main() {
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```
代码 9-6 用match表达式返回不同的错误

上面的这个代码可以有更加简短的表达方式，但是我们为了更加明白错误处理(error handling)还是用多代码的形式来表达，在后面，有更加简短的表达。首先，来看看这个函数的返回值：`Result<String, io::Error>`。这个就表示了这个函数的返回值的数据类型是`Result<T, E>`，`T` 的具体数据类型是 `String`，而 `E` 的具体数据类型是 `io::Error`。如果这个函数没有发生任何的问题，就会成功返回用 `Ok` 包裹String 的对象。如果程序发生了任何的问题，那么就会出现问题，就会返回用 `Err` ，其包裹了 `io::Error` 类型的对象，这个对象中包含了函数的错误的信息。之所以选择类型 `io::Error`，是因为不管 `File::open` 操作还是 `read_to_String` 操作的错误的类型都是 `io::Error`。

这个函数在最开始时候调用了一个 `File::open` 函数。接着我们用了和 代码 9-4中的同一种方式：`match` 表达式，来处理了这函数的返回的 `Result` 类型的值，只不过在9-4 中是把错误输出。在 `Err` case 中，此函数会把从函数 `File::open` 得到错误返回给调用方。如果调用函数 `File::open` 成功执行了，那么就把文件句柄保存在变量 `f` 继续下面的步骤。

然后创建一个新的 `String(字符串)` 类型的变量 `s`以及在文件句柄 `f` 中调用 `read_to_string` 方法，然后读取文件的内容到变量 `s`中。方法 `read_to_string ` 是有可能会出现错误情况的，所以会返回 `Result` 类型的值。所以就需要另一个 `match` 的表达式来处理这个 `Result` 类型的值：如果成功返回数据，那么就会返回一个用 `Ok` 包裹的s的对象；如果出现错误的情况，那么就会返回用 `Err` 包裹的错误信息。不过在最后的这一行不用关键字 `return` 来修饰，这是这个函数的最后一个表达式。

调用上面这个函数 `read_username_from_file` 的最后会得到一个 `Reuslt` 类型的值，在成功执行到了函数的情况下，就会得到包含的用户名 `Ok` 对象；如果出现错误了，那么就返回用`Err` 包裹的 `io::Error` 的值。因为不知道到底调用代码会出现什么的错误，如果出现错误那么，就会返回就会中断程序，如果正确执行的话，那么就会返回相应的值(用户名)。因为没有足够的信息来确定调用代码是正确还是错误，所以我们把不管是成功还是错误的结果都向上传播。

##### 2.3.1 一个简短版的传播错误的代码: `?` 操作符 (A Shortcut for Propagating Errors: the ? Operator) 
代码 9-7 展示了和函数 `read_username_from_file` 在代码 9-6 中一样的功能的代码，只不过有部分的功能是用 `?` 操作符。
```rust
#![allow(unused_variables)]
fn main() {
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
}
```
代码 9-7: 用 `? `操作符来返回错误的情况

The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in Listing 9-6. If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

There is a difference between what the match expression from Listing 9-6 does and what the ? operator does: error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another. When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as each error type implements the from function to define how to convert itself to the returned error type, the ? operator takes care of the conversion automatically.

In the context of Listing 9-7, the ? at the end of the File::open call will return the value inside an Ok to the variable f. If an error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code. The same thing applies to the ? at the end of the read_to_string call.

The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the ?, as shown in Listing 9-8.





##### 2.3.2 The ? Operator Can Be Used in Functions That Return Result


# 3 To panic! or Not to panic!

### 3.1 Examples, Prototype Code, and Tests


### 3.2 Cases in Which You Have More Information Than the Compiler


### 3.3 Guidelines for Error Handling



### 3.4 Creating Custom Types for Validation


# 4 Summary


