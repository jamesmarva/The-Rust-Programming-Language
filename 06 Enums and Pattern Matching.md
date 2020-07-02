在这个章节我们会看到枚举类型，同时简称为枚举。枚举允许你通过举例它的可能的变量来定义一个类型。首先，我们会定义以及使用枚举来展示枚举是如何将含义和数据一起编码的。其次，我们会探索一个非常有用的枚举类 Option，这个枚举类表示一个值可以代表某些东西，也可以什么都没有。然后我们会看到在匹配模式里面，match 表达式是如何针对枚举的不同值来轻松运行不同的代码的（有点像Java switch？）。最后，我们将介绍`if let` 结构是如何为你提供了另一个方便和简洁的习惯用法来处理代码中枚举类。
枚举是非常多的编程语言都有的特性，但是每种语言的表示都有所不同。 Rust 的枚举和代数数据类型(algebraic data types) 在函数式语言中，比如F#, OCaml, and Haskell.

# 1. 定义一个枚举 Defining an Enum
Let’s look at a situation we might want to express in code and see why enums are useful and more appropriate than structs in this case. Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six. These are the only possibilities for an IP address that our program will come across: we can enumerate all possible variants, which is where enumeration gets its name.
先来看一个我们想要用代码来表达的一种情形案例，接着就会明白为什么这种情形下里这里枚举比结构体更加有用以及更加的合适。假设我们我们的程序中需要用到IP地址。当前，IP地址主要是分成两个版本的，IPv4 和 IPv6。就目前的情况下，我们在实际情况中只会遇到这种两种IP协议，这两种协议已经包含了我们能遇到的所有可能的情况，我们可以在代码中枚举所有的IP协议版本，这就是枚举的名称的由来。
Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate, because enum values can only be one of its variants. Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.
任何一个IP地址都是第四版和第六版中的一个，但是不能同时两个都是。因为IP地址这个条件的限制使得这里使用枚举更加的妥当。因为枚举只能是他的变量的之一。
We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be, V4 and V6. These are the variants of the enum



```rust
enum IpAddrKind {
    V4,
    V6,
}
```
`IpAddrKind` 现在是一个自定义的数据类型了，我们可以在别处的代码中使用了。
### 1.1 枚举值 Enum Values
We can create instances of each of the two variants of IpAddrKind like this:


```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
```


```rust
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```



### 1.2 `Option`枚举类型以及它相对于空值的高级特性 (The Option Enum and Its Advantages Over Null Values)

Null 的发明者 `Tony Hoare` 在他的2009年演讲“Null References”：一个十亿美元的错误中提到：
> 我称之为我的十亿美元错误。当时，我正在设计第一个全面的类型系统，以面向对象的语言进行引用。我的目标是确保所有引用的使用都绝对安全，并由编译器自动执行检查。但是我忍不住要插入空引用的诱惑，仅仅是因为它很容易实现。这导致了无数的错误，漏洞和系统崩溃，在最近四十年中可能造成十亿美元的痛苦和破坏。
### 

### 



# 2. The match Control Flow Operator
Rust has an extremely powerful control flow operator called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things; Chapter 18 covers all the different kinds of patterns and what they do. The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into. In the same way, values go through each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution.

Because we just mentioned coins, let’s use them as an example using match! We can write a function that can take an unknown United States coin and, in a similar way as the counting machine, determine which coin it is and return its value in cents, as shown here in Listing 6-3.

# 3. Concise Control Flow with if let

The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest. Consider the program in Listing 6-6 that matches on an Option<u8> value but only wants to execute code if the value is 3.



We want to do something with the Some(3) match but do nothing with any other Some<u8> value or the None value. To satisfy the match expression, we have to add _ => () after processing just one variant, which is a lot of boilerplate code to add.

Instead, we could write this in a shorter way using if let. The following code behaves the same as the match in Listing 6-6: