在这个章节我们会看到枚举类型，同时简称为枚举。枚举允许你通过举例它的可能的变量来定义一个类型。首先，我们会定义以及使用枚举来展示枚举是如何将含义和数据一起编码的。其次，我们会探索一个非常有用的枚举类 Option，这个枚举类表示一个值可以代表某些东西，也可以什么都没有。然后我们会看到在匹配模式里面，match 表达式是如何针对枚举的不同值来轻松运行不同的代码的（有点像Java switch？）。最后，我们将介绍`if let` 结构是如何为你提供了另一个方便和简洁的习惯用法来处理代码中枚举类。
枚举是非常多的编程语言都有的特性，但是每种语言的表示都有所不同。 Rust 的枚举和代数数据类型(algebraic data types) 在函数式语言中，比如F#, OCaml, and Haskell.

# 1. 定义一个枚举 Defining an Enum

```rust
enum IpAddrKind {
    V4,
    V6,
}
```
`IpAddrKind` 现在是一个自定义的数据类型了，我们可以在别处的代码中使用了。
### 1.1 枚举值 Enum Values

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



### 1.2 The Option Enum and Its Advantages Over Null Values

### 

### 



# 2. The match Control Flow Operator


# 3. Concise Control Flow with if let

