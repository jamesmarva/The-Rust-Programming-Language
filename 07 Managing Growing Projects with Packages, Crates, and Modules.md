当你写大型的程序的时候，如果去组织你的代码会变的非常重要，因为你不可能能单纯靠着脑子去跟踪你的代码。通过吧相关的功能都组织起来，以及吧单独的功能给拆分开，找到的你一个特别的功能可以就变得很清晰，很容易了，

就目前为止的我们所写的代码都是在一个文件里的一个模块，随着项目的开发，你可以通过把代码拆分成不同的模块然后分成不同的文件来组织你的代码。一个包可以包含多个bin crate，也可以仅仅包含一个lib crate。随着包的开发，你可以把不同的模块的代码提取到不同过的 crate中，接下来变成外部依赖。这章涉及所有的这些技术。

通过新增给功能的归组，封装实现细节会让你在更高的级别重用代码：一旦你实现了一个操作，其他的代码可以通过调用 public 方法来使用，而不用知道内部的实现的细节。

In addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level: once you’ve implemented an operation, other code can call that code via the code’s public interface without knowing how the implementation works. The way you write code defines which parts are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head.

A related concept is scope: the nested context in which code is written has a set of names that are defined as “in scope.” When reading, writing, and compiling code, programmers and compilers need to know whether a particular name at a particular spot refers to a variable, function, struct, enum, module, constant, or other item and what that item means. You can create scopes and change which names are in or out of scope. You can’t have two items with the same name in the same scope; tools are available to resolve name conflicts.

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:
- Packages（包）: A Cargo feature that lets you build, test, and share crates
- Crates（板条箱）: A tree of modules that produces a library or executable
- **Modules** and **use** （模块以及use）: Let you control the organization, scope, and privacy of paths
- Paths（路径）: A way of naming an item, such as a struct, function, or module

在这章中，我们将会涉及所有这些功能，讨论如何将他们联系起来
In this chapter, we’ll cover all these features, discuss how they interact, and explain how to use them to manage scope. By the end, you should have a solid understanding of the module system and be able to work with scopes like a pro!

# 1 包 和 箱 Packages and Crates
我们介绍的模块系统的第一部分是 包(packages) 和 箱(crates)，一个 箱（crate） 可以是一个 二进制文件(binary crate) 或者 库(libiary crate)。箱(crate)的 *root* 是一个是 Rust编译器开始运行的代码文件，以及组成了箱的组成模块。一个 *包（package）* 是一个或者多个的提供功能的箱（crate）。一个包(package)包含一个 `Cargo.toml` 文件，这个文件描述了如何构建这些箱。

有几个规则规定了一个包(package)里可以包含什么。一个包（package） *必须* 包含0个 或者 1个 library crate ，并且不能有超过一个。包（package）可以包含任意数量的binary crate，但是必须包含至少一个 crate。

让我们来看看我们创建一个代码包的时候，我们会发生什么？首先，我们输入一个命令：`cargo new`：
```shell
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```
输入命令之后，`Cargo` 创建了一个文件 `Carog.toml`，以及创建了一个代码包(package)。先来看看这个 `toml` 的文件里的内容，在文件里并没有提及 `src/mian.rs` 文件，这是因为 `Cargo` 遵循一个规则，就是 `src/main.sr` 是与包（package）同名的二进制箱（binary）的根(crate root)。这样， Car就知道的如果 包的文件夹下面包含 文件 `src/lib.rs`，这个包含一个就包含了一个同名箱的根。`Cargo` 将箱的文件传递给 `rustc`，用于创建 文件库或者二进制文件。
此时，我们只有一个仅仅包含了 `src/main.rs`的包，这个就意味着它仅仅包含了一个命名为 `my-project` 的二进制箱，如果一个包含 `src/main.rs` 和 `src/lib.rs`，那么它就有了两个箱：一个库和一个二进制文件，两个的箱的名称和包的名称相同。通过吧文件放在 `src/bin` 的文件夹下面，1个软件包就可以具有多个二进制的文件，每个文件都是一个单独的文件箱。

箱会把一个作用域里的所用的相关的功能组合在一起，因此该功能就可以在不同的项目之间共享。通过把箱导入我们项目范围，比如，我们在第二章中用的 `rand` 的箱提供可以可以生成随机数能给你。通过把包导入倒我们的项目中，就可以通过 `rand` 的箱来访问 `rand` 的箱提供的功能了。
把 箱的功能保持在自己的范围之内，这样就可以分清楚这是在的我们的箱中还是在 `rand` 箱中的定义了特定的功能，并且防止了潜在的冲突。比如，`rand` 箱提供了一个 命名为 `Rng` 的功能。我们也可以顶一个命名为 `Rng` 的结构体，在我们的自己箱中。因为一个箱的功能是在他自己的范围内命名的，因此当我们把 `rand`  添加为依赖项的时候，编译器是不会对 `Rng` 的所指的名称感到困惑。在我们的代码箱中，它值的我们定义的 `Rng`的结构体。我们将从 `rand` 箱中以 `rand::Rng` 访问 `Rng` 的功能。
接下来我们继续讨论模块系统。
# 2 Defining Modules to Control Scope and Privacy
在这一节，我们将讨论模块以及模块的其他部分， 用关键字 `use` 来把想一个路径引入我们的项目，用 `pub` 的关键字来把代码的功能公开。我们也会讨论的 关键字 `as` 外部包，以及 `glob` 操作符。现在让我们先来关注 『modules(模块)』
模块让我们可以将 箱(crate)中的代码分组，以及提高代码的可读性以及重用性。模块也可以控制项目的隐私，就是这个代码是可以被外部使用的(public)的，还是仅仅是内部的实现细节，不被外部使用(private)。

我们提供了一个例子，要先写一个提供 『餐厅』 功能的 代码库箱(library crate)。我们要会顶一个方法的签名，但是要会留着他妈的方法体空白以专注于代码的组织，而不是在去实现餐厅的代码。
In the restaurant industry, some parts of a restaurant are referred to as front of house and others as back of house. Front of house is where customers are; this is where hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.
在餐饮行业中，餐厅的某些部分被成为房屋前部，而其他部分称为房屋的后。屋前就是顾客的信任所在。店主在这里招待客人，服务员在这里开单子以及付款。屋后是厨师工作的地方，以及洗碟子，以及管理店的地方。


To structure our crate in the same way that a real restaurant works, we can organize the functions into nested modules. Create a new library named restaurant by running cargo new --lib restaurant; then put the code in Listing 7-1 into src/lib.rs to define some modules and function signatures.

为了以实际餐厅的模型来构造 箱(crate)，我们可以功能到嵌套模块中。创建一个新的代码库命名为 `restaurant `，通过运行命令：`cargo new --lib restaurant`；同时把代码清单的 7-1的代码写到 `src/lib.rs` 文件中为了定义模块以及方法签名。
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
↑ 代码清单：7 - 1：`front_of_house ` 包含其他的模块，以及功能。
定义一个模块从关键字 `mod` 开始，以及指名这个模块的名字(在这个案例里，名字叫 `front_of_house`)，然后用大括号把模块的代码体包裹起来。在模块中，我们可以有其他的模块，在这个案例里，里面的模块就是 `hosting` 和 `serving`。模块也可以定义别结构的代码，比如结构体，枚举型，常量以及功能特性，还有函数。

通过使用模块，我们就能根据代码的功能相关性进行分组了。开发者就可以这个花更短的时间根据代码的所代表的功能来查找代码，因为开发者可以根据代码的分组导航到代码，不用阅读所有的代码。这样添加新的代码的时候就可以知道把代码放到何处以保持程序的组织性。

在前面，有提到关于 *src/main.rs* 和 *src/lib.rs* 是箱的根，是因为这个文件中的任何一个内容在箱的模块的根目录形成了一个名为 crate  的模块。
Listing 7-2 shows the module tree for the structure in Listing 7-1.
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
↑ 代码清单 7-2: 代码清单 7-1 的模块代码树

注意，这个模块树是在隐含的模块 `crate` 下面的

# 3 引用项目的路径 Paths for Referring to an Item in the Module Tree

A path can take two forms:
路径有两种格式
- An absolute path starts from a crate root by using a crate name or a literal crate.
- 绝对路径是通过使用箱的名称或者字符串 `crate` 从箱的根目录开始的。
- A relative path starts from the current module and uses self, super, or an identifier in the current module.


Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).
绝对路径和相对路径吊样能够都是用 两个冒号来进行调用的 `::`

以下的代码有问题，无法编译通过。
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
↑ 代码7-3:根据绝对路径和相对路径来调用 `add_to_waitlist` 函数
### 3.1 使用关键字 `pub` 来公开路径 Exposing Paths with the pub Keyword


### 3.2 利用`super`来使用 相对路径Starting Relative Paths with super


### 3.3 把结构体和枚举的访问权限修改为 `Public` Making Structs and Enums Public

```java
#![allow(unused_variables)]
fn main() {
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }

    pub fn eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
    }
}
```
代码清单：7-9 一个有public的字段和private字段的结构体

试着把注释那行代码取消注释，来看看会发生什么错误信息。


另外，这里要注意，由于`back_of_house::Breakfast`有个私有字段

枚举里如有 `pub` 来修饰 `enum` 那么里面的所有的枚举类型都是公开的(`public`)的权限的。

这里还没有涉及`pub`另一种情况，

# 4 用关键字`use`把模块路径引入作用域 Bringing Paths into Scope with the  use  Keyword



In Listing 7-11, we bring the `crate::front_of_house::hosting` module into the scope of the `eat_at_restaurant` function so we only have to specify `hosting::add_to_waitlist` to call the `add_to_waitlist` function in `eat_at_restaurant`.


```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {}
```
↑ 代码 7-11: Bringing a module into scope with use



```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {}
```
↑ 代码 7-12: 用关键字 `use`和相对路径导入包

### 4.1 创建一个惯用的 `use` 路径 (Creating Idiomatic use Paths)
也许你会疑惑，在代码7-11 中为什么我们要指定 `use crate::front_of_house::hosting` 这个路径，然后在 `eat_at_restaurant` 调用 `hosting::add_to_waitlist` 来使用方法，而不是直接指定 全路径直到 `add_to_waitlist` 来达到相同的效果，就像代码7-13中展示的：
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

fn main() {}
```
↑ 代码 7-13: 把函数 `add_to_waitlist` 通过关键字 `use` 引入作用域 ，但这个不是习惯的用法
代码 7-11 和代码 7-13 是可以完成相同功能的，但是代码清单7-11是一个惯用的通过 `use` 关键字把函数引入作用域的。通过引入函数的父模块，我们就不得不在调用函数的带出函数的父模块，以便知道这个函数到底是不是本地定义的，同时又让完整路径重复覆盖达到了最小。代码 7-13 中，我们不知道函数 `add_to_waitlist` 实在哪里定义的。
另一方面，当引入结构体，枚举类型还有其他项的时候，习惯上制定完整的路径。代码7-14就显示了将标准库 `HashMap` 引入二进制的代码箱的习惯方式。
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

```
↑ 代码 7-14: 用惯用的方式引入 HashMap 
为什么一定要这样使用，其实并没有非常有说服力的理由，只能说这个就是给习惯的用法，人们已经习惯了这样阅读 Rust 代码。
这个惯例的例外情况是，如果我们将两个相同名称的类型放入作用域中，rust是不允许这么使用的。代码 7-15 显示了，如何将两个有相同名称但是不同父模块的不同的 `Result` 引入作用域中，以及如何去引用他它们：
```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```
↑ 代码 7-15: 通过使用他们的父模块引入两个相同名称的类型到作用域中
就像你看到的，利用父模块来曲风这两个 `Rusult` 类型。如果我们用 `use std::fmt::Result` 和 `use std::io::Result` 来引入这两个类型，那么在编译的时候，Rust 就不知道在代码中，我们使用的 `Result` 指的是哪一个类型。

### 4.2 使用关键字`as` 来指定一个新的名称 (Providing New Names with the `as` Keyword) 

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```
↑ 代码 7-16: 用关键字 `as` 来给引入的类型重新命名
在第二个 `use` 语句中，给 `std::io::Result` 类型用了一个新的名字 `IoResult`，这样就不会和 `std::fmt` 的Result 的冲突了。不管是 代码 7-15 还是 代码 7-16 都是惯用的方式，至于用哪个，完全取决于你。

### 4.3 Re-exporting Names with `pub use`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {}
```
↑ 代码 7-17: Making a name available for any code to use from a new scope with pub use

By using pub use, external code can now call the add_to_waitlist function using hosting::add_to_waitlist. If we hadn’t specified pub use, the eat_at_restaurant function could call hosting::add_to_waitlist in its scope, but external code couldn’t take advantage of this new path.
通过使用 `pub use`，外部的代码就可以在调用 `add_to_waitlist` 的同时使用 `hosting::add_to_waitlist` 了。如果我们不用 `pub use`，那么 `eat_at_restaurant` 可以调用 `hosting::add_to_waitlist` 在它的作用域里，但是外部的代码不能利用这个这个新的路径了。
Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. With pub use, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library.
`重导入` 是一个非常有用的功能，
### 4.4  用第三方的包 (Using External Packages)

In Chapter 2, we programmed a guessing game project that used an external package called rand to get random numbers. To use rand in our project, we added this line to Cargo.toml:
```toml
[dependencies]
rand = "0.5.5"
```

Then, to bring rand definitions into the scope of our package, we added a use line starting with the name of the package, rand, and listed the items we wanted to bring into scope. Recall that in the “Generating a Random Number” section in Chapter 2, we brought the Rng trait into scope and called the rand::thread_rng function:

### 4.5 Using Nested Paths to Clean Up Large use Lists


### 4.6 The Glob Operator


# 5 Separating Modules into Different Files



# 6 Summary