





解引用强制多态如何与可变性交互


# 3 用 Drop Trait 来清理代码 （Running Code on Cleanup with the Drop Trait）

```
struct CustomSmartPointer { 
    data: String, 
} 
 
impl Drop for CustomSmartPointer { 
    fn drop(&mut self) { 
        println!("Dropping CustomSmartPointer with data `{}`!", self.data); 
    } 
} 
 
fn main() { 
    let c = CustomSmartPointer { data: String::from("my stuff") }; 
    let d = CustomSmartPointer { data: String::from("other stuff") }; 
    println!("CustomSmartPointers created."); 
} 
```
代码 15-14 

## 3.1 用 `std::mem::drop` 提前丢弃值

```rust
fn main() { 
    let c = CustomSmartPointer { data: String::from("some data") }; 
    println!("CustomSmartPointer created."); 
    c.drop(); 
    println!("CustomSmartPointer dropped before the end of main."); 
} 
```
代码 15-15 尝试调用 `Drop` trait 中的 drop 方法来手动提前是释放

编译上面这段代码，会输出一下的错误信息
```
$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
error[E0040]: explicit use of destructor method
  --> src/main.rs:16:7
   |
16 |     c.drop();
   |       ^^^^ explicit destructor calls not allowed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0040`.
error: could not compile `drop-example`.

To learn more, run the command again with --verbose.
```





```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```
代码15-16 



# 4 `Rc<T>`可以计数引用的智能指针(Rc<T>, the Reference Counted Smart Pointer)



## 4.1 使用 Rc<T> 共享数据(Using Rc<T> to Share Data)

## 4.2 克隆 Rc<T> 会增加引用计数(Cloning an Rc<T> Increases the Reference Count)

```
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```
代码15-19 输出引用计数

```
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/cons-list`
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```
可以看见，在变量 `a` 中的 `Rc<List>` 有个初始的引用计数为1，接下来的每次 `clone`，这个技术都是加一。当 `c` 离开作用域的时候，这个计数就会减一。我们不用特地调用一个方法来减少引用，就像我们调用`Rc::clone` 来增加引用次数一样：实现 `Drop` trait 会使 `Rc<T>` 离开作用域的时候会自动地减少引用次数。

# 5 `RefCell<T>` 和 内部可变性 (RefCell<T> and the Interior Mutability Pattern)
*内部可变性(Interior mutability)* 是Rust的一种设计模式，他允许你在即使在有不可变引用的时候也能修改数字，这个操作是借用规则（borrowing rules）所不允许的。为了改变数据，这个模式在数据结构中用 `unsafe` 代码来模糊 Rust 的可变操作和借用规则。因为我们还没涉及 不安全代码；我们将会在第19章学习。当我们可以确保程序会在运行的时候遵守借用规则，即使在编译器不能保证安全的情况下，我们仍然可以使用**那些使用内部可变的模式** 的类型。`不安全(unsafe)` 代码将会被封装进安全的API里，但是仍然是不可变的。

让我们来看看 `RefCall<T>` 类型，这个使用了内部可变的模式的数据类型。

## 5.1 用 `ReCell<T>` 在运行的时候执行检查借用规则（Enforcing Borrowing Rules at Runtime with RefCell<T>）
和 `Rc<T>` 不同，`RefCell<T>` 类型代表了它持有了数据的唯一的所有权。所以，是什么让 `RefCell<T>` 和 `Box<T>` 不同。回顾一下在第四章学习的借用规则（borrowing rules）
- 
- 


## 5.2 内部可变性：一个不可变值的可变引用(Interior Mutability: A Mutable Borrow to an Immutable Value)
借用规则有个推论是：当一个值是不可变的，不能进行可变借用。如下的代码就是错误的：
```
fn main() {
    let x = 5;
    let y = &mut x;s
}
```
尝试编译这段代码，就会得到一下的错误信息：
```
$ cargo run
   Compiling borrowing v0.1.0 (file:///projects/borrowing)
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
 --> src/main.rs:3:13
  |
2 |     let x = 5;
  |         - help: consider changing this to be mutable: `mut x`
3 |     let y = &mut x;
  |             ^^^^^^ cannot borrow as mutable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
error: could not compile `borrowing`.

To learn more, run the command again with --verbose.
```
可是，在某些情况下，一个值可以在它的方法里修改它的值，但是在别的代码中，它还是不变的，这种特性还是很有用的。在这个值的方法之外的代码是无法改变其值的。`RefCell<T>` 是一个可以获取内部可变的特性的方法。 但是 `RefCell<T>` 还是没有完全绕来借用规则：编译器中的借用检查器还是可以允许通过这些内内部可变性的，并且在运行的时候检查这种借用规则。如果编写的代码违反了这些规则，那么就会报 `panic!` 而不是编译错误。

让我们通过实际操作来看看什么何处可以用内部可变这个特性是有用的，以及为什么有用。

### 5.2.1 内部可变的用武之地：mock 对象(A Use Case for Interior Mutability: Mock Objects)
*测试替身(test double)* 是一个通用的编程理念，就是表示在测试的时候代替某个类型以用来测试。 *mock 对象（Mock objects）* 就是就类型的替身，它们可以用来记录测试过程中发生了什么，以及你可以断言哪些操作是对的。

```rust
#![allow(unused)]
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```
代码 15-20 


```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
      let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```
代码15-22 用 `RefCell<T>` 来修改一个不可变的引用的内部值

现在的字段 `sent_message` 字段已经是 `RefCell<Vec<String>>` 类型而不是 `Vec<String>` 类型了。在函数 `new` 中，我用空 vector 来创建了一个 `RefCell<Vec<String>>` 对象。

`send` 方法的实现代码中，第一个参数 `self` 仍然是一个不可变的借用，这样是符合 trait 定义的。我们可以调用 `borrow_mut()` 方法来获取 `self.sent_message` 的 `RefCell` 的值 `Vec<String>`。然后就可以在可变的引用中调用 `push` 方法来记录发送的消息。

最后就是新增一个断言：为了看看内部的vector有多少的数据，要调用 `RefCell` 的 `borrow()` 来获取不可变的引用。

现在你已经知道了如何使用 `RefCell<T>` 了，下一步来深入研究一下是如何工作的。

### 5.2.2 用 `RefCell<T>` 在运行时跟踪借用(Keeping Track of Borrows at Runtime with RefCell<T>)


```rust
impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        let mut first_borrow = self.sent_message.borrow_mut();
        let mut second_borrow = self.sent_message.borrow_mut();
        first_borrow.push(String::from(msg));
        second_borrow.push(String::from(msg));
    }
}
```
代码15-23 在同一个作用域中创建两个可变的引用，会引发 `panic`

## 5.3 用 `Rc<T>` `RefCell<T>` 来使可变数据有多个持有者(Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>)
```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```
代码15-24 用 `Rc<RefCell<i32>>` 来创建一个我们可以修改的 `List` 对象


# 6 循环引用导致的内存泄漏(Reference Cycles Can Leak Memory)

## 6.1 写一个循环引用(Creating a Reference Cycle)

## 6.2 