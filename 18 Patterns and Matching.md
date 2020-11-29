模式（Patterns）是 Rust 中用来匹配 the structure of types 的特殊的语法，不管这个type是简单还是复杂的。 `match` 连着其他的结构体一起使用的 patterns 方法下，你可以更好的控制程序的流程。模式由下面的列表中的几种元素的组合形成：
- 字面值（Literals）
- 对于数组，枚举，结构体或者元组的解构
- 变量（Variables）
- 通配符（Wildcards）
- 占位符（Placeholders） 

这些组件（components）描述了我们正在使用的数据的是什么，然后我可以通过匹配值来决定我们的程序要如何根据确定的数据来执行特定的代码。

想要使用一个 pattern，我们就要将它和其他的值进行比较。



# 什么位置可以用 Patterns （All the Places Patterns Can Be Used）

Patterns 出现在Rust的很多地方，在你不经意间，你就使用了很多模式了，这个部分就是对所有的有效的位置的一个参考。

## 1.1 `match` Arms



## 1.2 `if let` 条件表达式（Conditional if let Expressions）

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;

    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    
    } else if is_tuesday {

    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orangle as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```
18-1 结合 `if let`， `else if `，`else if let`, `else`


## 1.3 `while let` 条件循环
```rust
use std::vec::Vec;

fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```
18-2 只要 `stack.pop()` 返回 `Some`，就用 `while let` 来循环输出 stack 的值。


## 1.4 `for` 循环

```rust
fn main() {
    let v: Vec<char> = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        prilntln!("{} is at index {}", value, index);
    }
}
```
18-3 在for的循环中来解构一个元组



## 1.5 let Statements

```rust
let x = 5;
```

```rust
let (x, y, z) = (1, 2, 3);
```
18-4


```rust
let (x, y) = (1, 2, 3);
```
18-5

```
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error[E0308]: mismatched types
 --> src/main.rs:2:9
  |
2 |     let (x, y) = (1, 2, 3);
  |         ^^^^^^ expected a tuple with 3 elements, found one with 2 elements
  |
  = note: expected tuple `({integer}, {integer}, {integer})`
             found tuple `(_, _)`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `patterns`.

To learn more, run the command again with --verbose.
```



## 1.6 方法参数（Function Parameters）




