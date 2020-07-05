As you write large programs, organizing your code will be important because keeping track of your entire program in your head will become impossible. By grouping related functionality and separating code with distinct features, you’ll clarify where to find code that implements a particular feature and where to go to change how a feature works.
当你写大型的程序的时候，如果去组织你的代码会变的非常重要，因为你不可能能单纯靠着脑子去跟踪你的代码。通过吧相关的功能都组织起来，以及吧单独的功能给拆分开，找到的你一个特别的功能可以就变得很清晰，很容易了，
The programs we’ve written so far have been in one module in one file. As a project grows, you can organize code by splitting it into multiple modules and then multiple files. A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies. This chapter covers all these techniques. For very large projects of a set of interrelated packages that evolve together, Cargo provides workspaces, which we’ll cover in the “Cargo Workspaces” section in Chapter 14.

In addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level: once you’ve implemented an operation, other code can call that code via the code’s public interface without knowing how the implementation works. The way you write code defines which parts are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head.

A related concept is scope: the nested context in which code is written has a set of names that are defined as “in scope.” When reading, writing, and compiling code, programmers and compilers need to know whether a particular name at a particular spot refers to a variable, function, struct, enum, module, constant, or other item and what that item means. You can create scopes and change which names are in or out of scope. You can’t have two items with the same name in the same scope; tools are available to resolve name conflicts.

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:
- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

In this chapter, we’ll cover all these features, discuss how they interact, and explain how to use them to manage scope. By the end, you should have a solid understanding of the module system and be able to work with scopes like a pro!


# 1 包 和 箱 Packages and Crates
我们介绍的模块系统的第一部分是 包(packages) 和 箱(crates)，一个 箱 是一个 二进制文件(binary file) 或者 库(libiary)。箱(crate)的根目录是一个是 Rust编译器开始运行的代码文件，以及组成了箱的组成模块。一个包是一个或者多个的提供功能的箱。一个包(package)包含一个 `Cargo.toml` 文件，这个文件描述了如何构建这些箱。
有几个规则规定了一个包(package)里可以包含什么。一个包必须包含0个 或者 1个的代码库，并且不能包含更多的代码库了，他可以包含任意数量的二进制箱子，但是他必须包含至少一个箱。
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
输入命令之后，`Cargo` 创建了一个文件 `Carog.toml`，以及创建了一个代码包(package)。先来看看这个 `toml` 的文件里的内容，在文件里并没有提及 `src/mian.rs` 文件，这是因为 `Cargo` 遵循一个规则，就是 `src/main.sr` 是与包同名的二进制箱的根(crate root)。这样， Car就知道的如果 包的文件夹下面包含 文件 `src/lib.rs`，这个包含一个就包含了一个同名箱的根。`Cargo` 将箱的文件传递给 `rustc`，用于创建 文件库或者二进制文件。
此时，我们只有一个仅仅包含了 `src/main.rs`的包，这个就意味着它仅仅包含了一个命名为 `my-project` 的二进制箱，如果一个包含 `src/main.rs` 和 `src/lib.rs`，那么它就有了两个箱：一个库和一个二进制文件，两个的箱的名称和包的名称相同。通过吧文件放在 `src/bin` 的文件夹下面，1个软件包就可以具有多个二进制的文件，每个文件都是一个单独的文件箱。

箱会把一个作用域里的所用的相关的功能组合在一起，因此该功能就可以在不同的项目之间共享。通过把箱导入我们项目范围，比如，我们在第二章中用的 `rand` 的箱提供可以可以生成随机数能给你。通过把包导入倒我们的项目中，就可以通过 `rand` 的箱来访问 `rand` 的箱提供的功能了。

Keeping a crate’s functionality in its own scope clarifies whether particular functionality is defined in our crate or the rand crate and prevents potential conflicts. For example, the rand crate provides a trait named Rng. We can also define a struct named Rng in our own crate. Because a crate’s functionality is namespaced in its own scope, when we add rand as a dependency, the compiler isn’t confused about what the name Rng refers to. In our crate, it refers to the struct Rng that we defined. We would access the Rng trait from the rand crate as rand::Rng.

Let’s move on and talk about the module system!

# Defining Modules to Control Scope and Privacy
在这一节，我们将讨论模块以及模块的其他部分， 用关键字 `use` 来把想一个路径引入我们的项目，用 `pub` 的关键字来把代码的功能公开。我们也会讨论的 关键字 `as` 外部包，以及 `glob` 操作符。现在让我们先来关注 『modules(模块)』

Modules let us organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

As an example, let’s write a library crate that provides the functionality of a restaurant. We’ll define the signatures of functions but leave their bodies empty to concentrate on the organization of the code, rather than actually implement a restaurant in code.

In the restaurant industry, some parts of a restaurant are referred to as front of house and others as back of house. Front of house is where customers are; this is where hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.

To structure our crate in the same way that a real restaurant works, we can organize the functions into nested modules. Create a new library named restaurant by running cargo new --lib restaurant; then put the code in Listing 7-1 into src/lib.rs to define some modules and function signatures.