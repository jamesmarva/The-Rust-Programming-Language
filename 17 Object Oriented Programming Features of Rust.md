



# 1 面向对象语言的特点（Characteristics of Object-Oriented Languages）

```rust

```
## 1.1 对象包含数据以及行为（Objects Contain Data and Behavior）


## 1.2 封装隐藏了实现细节(Encapsulation that Hides Implementation Details)

```rust
pub struct AveragedCollection { 
    list: Vec<i32>, 
    average: f64, 
} 
```
代码17-1 在 `AveragedCollection` 结构体维护了一个整型的列表和集合里的平均值

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```
代码 17-2


## 1.3 Inheritance as a Type System and as Code Sharing

> 多态(polymorphism)
>
>
> 
> 

# 2 为不同的值而设计的 Trait 对象（Using Trait Objects That Allow for Values of Different Types）

## 2.1 为通用的行为定义一个Trait（Defining a Trait for Common Behavior）
```rust
pub trait Draw {
    fn draw(&self);
}
```
代码17-3 定义一个 `Draw` trait



```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```
代码 17-4 定义


```rust
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```
代码 17-5 

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```
代码17-6 一种 `Screen` 的替代实现，用 trait 以及 trait bound

上面这样的实现，限制了Screen 的实例必须全是 `Button` 或者 全都是 `TextFiled`类型的组件列表。如果你只要一个只有相同类型的集合，用泛型还有 trait bound 是可取的，因为定义在编译的时候会用根据具体的类型进行单态化（monomorphized）

另一方面，在使用 trati object 的方案下，一个 `Screen` 实例可以持有同时持有`Box<Button>` 以及 `Box<TextFiled>`。来看看这个方案是如何实现的，以及运行时候的性能影响。


## 2.2 实现 Trait （Implementing the Trait）
```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```
17-7 



```rust
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
```
17-8 


```rust
use gui::{Button, Screen};
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}
```
17-9 

```rust
use gui::Screen;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(String::from("hi")),
        ],
    };
}
```
17-10 尝试使用一种没有实现 `Draw` 的对象。

```
$ cargo run
   Compiling gui v0.1.0 (file:///projects/gui)
error[E0277]: the trait bound `std::string::String: gui::Draw` is not satisfied
 --> src/main.rs:5:26
  |
5 |         components: vec![Box::new(String::from("Hi"))],
  |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `gui::Draw` is not implemented for `std::string::String`
  |
  = note: required for the cast to the object type `dyn gui::Draw`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `gui`.

To learn more, run the command again with --verbose.
```
## 2.3 Trait Objects 实现的动态分发(Trait Objects Perform Dynamic Dispatch)


## 2.4 使用 Trait Objects 需要 对象安全（Object Safety）Object Safety Is Required for Trait Objects

# 3 Implementing an Object-Oriented Design Pattern


## 3.1 Defining Post and Creating a New Instance in the Draft State


## 3.2 Storing the Text of the Post Content


## 3.3 Ensuring the Content of a Draft Post Is Empty


## 3.4 Adding the approve Method that Changes the Behavior of content



```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // --snip--
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // --snip--
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // --snip--
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

```

```rust
impl Post {
    // --- snip ---
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self);
    }
    // --- snip ---
}
```
17-17 委托给 `State` 的 `content` 来更新调用在第更新 `Post` 的 `content`

我们的目标是保持在结构体 `State` 里的实现，然后调用字段 `state` 的 `content` 方法来，并且传 Post 实例作为参数，然后会返回字段 `state` 的`content` 方法作为返回值。

这里调用 `Option` 的 `as_ref` 来返回一个 `Option` 中的值的引用，而不是返回其的值。因为 `state` 是个 `Option<Box<dyn State>>` 的类型，所以调用 `as_ref` 方法会返回一个 `Option<&Box<dyn State>>` 类型。如果不调用的话就会报错，因为不能从一个 `&self` 中把 state 给 move 出来。
> 注: 上面的这段意思可以理解为，`self.as_ref().unwrap().content(self)`，可以看成下面的代码，先创建一个临时的变量 `tmp` 类型为 `&Box<dyn State>`，用来存放一个临时的值，代码如下：
> ```
> pub fn content(&self) -> &str {
>     let tmp: &Box<dyn State> = self.state.as_ref().unwrap();// tmp is borrower
>     tmp.content(self)
> }
> ```
> 1 post 取完 content之后还是要可以取的，所以这里只能是 `&self`
> 2 既然是借用，那么就不能从 `self` 里面 `move` 出 `state` 变量。
> 3 所以这里的只能借用 `state` 变量，所以 `tmp` 是个 `&Box<dyn State>` 类型，是个 borrower。
> 4 所以必须要用 `as_ref` 方法来得到一个 `borrower`
>
> 比如下面这种代码，编译器就会报错的：
> ```
> pub fn content(&self) -> &str {
>     let tmp: Box<dyn State> = self.state.unwrap();
>     tmp.content(self)      
> }
> ```
> 不能把state的move出来，只能是用as_ref 的得到借用的对象

```rust
trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```
 17-18 给 `State` 的trait新增 `content` 方法

这里在 `State` trait 新增了一个默认的实现的方法 `content`，这个方法返回一个空的字符串切片(string slice)。这就意味着我们不需要在 `Draft` 和 `PendingReview` 的结构体中实现 `content` 方法了。`Published` 结构体实现的 `content` 方法会覆盖原来的默认的实现，并且会返回 `post.content` 的值。

## 3.5 状态模式的取舍（Trade-offs of the State Pattern）


用状态模式实现的代码很有利于扩展，为了更好的理解这个模式下维护代码的简洁性，请试试下面的功能实现：
- 1 新增一个 `reject` 方法，把状态`PendingReview`转变为 `Draft`
- 2 在转换为 `Published` 之前需要调用两次 `approve` 方法
- 3 只允许用户在状态 `Draft` 的新增内容，提示：让状态对象负责可以发生什么改变，但是不负责改变 `Post`



## 3.6 把行为和状态编码为类型（Encoding States and Behavior as Types）






## 3.7 Implementing Transitions as Transformations into Different Types


# 4 Summary












