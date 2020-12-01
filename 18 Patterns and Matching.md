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


# 3 所有的模式匹配语法


## 3.1 匹配字面量（）

### 3.5.2 解构枚举（Destructuring Enums）


### 3.5.3 解构嵌套的结构体和枚举

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Change the color to red {}, green {}, and blue {}", r, g, b),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Change the color to hue {}, saturation {}, and value {}", h, s, v),
        _ => (),
    }
}
```
18-16 匹配内嵌的枚举


### 3.5.4 解构结构体和元组（Destructuring Structs and Tuples） 


```
let ((feet, inches), Point{x, y}) = ((3, 10), Point{x:3, y:-10});
```

## 3.6 忽略在Pattern中的某些值

### 3.6.1 Ignoring an Entire Value with `_`

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y),
}

fn main() {
    foo(3, 4);
}
```
18-17 在函数的签名里用 `_`

### 3.6.2 Ignoring Parts of a Value with a Nested `_`
```rust
let mut setting _value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}
println!("setting is {:?}", setting_value);
```
18-18 在patterns中用一个下划线来匹配一些不需要`Some`的 内部值的情形

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _ fifth) => {
        println!(")
    }
}
```
18-19 忽略元组中的多个值

### 3.6.3 Ignoring an Unused Variable by Starting Its Name with  `_`

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```
18-20 在没有使用到的变量的开始加上下划线`_`，来避免未使用变量的警告。

```rust
fn main() {

    let s = Some(String::from("Hello！"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}
```
18-21 

```rust
fn main() {
    let s = Some(String::from("hello!"));

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);
}
```
18-22 用下划线就不会绑定值


### 3.6.4 Ignoring Remaining Parts of a Value with  ..
```rust
fn main() {
    struct Point {
        x: i32, 
        y: i32, 
        z: i32,
    }

    let origin = Point{x: 0, y: 0, z: 0};

    match origin {
        Point{x, ..} => println!("x is {}", x),
    }
}
```
18-23 用 `..`来忽略在 `Point` 里的除了 `x` 以外的字段


```rust
fn main() {
    let numbers = {2, 4, 8, 16, 32};

    match number {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    match number {
        (first, .., last) => println!("Some numbers: {}, {}", first, last),
    }
}
```
18-24 仅仅匹配元组里的第一个值和最后一个值，然后忽略其他的值。

```rust


```
### 3.6.5 