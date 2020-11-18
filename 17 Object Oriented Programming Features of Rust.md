



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


