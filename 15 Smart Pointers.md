





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


### 5.2.1 A Use Case for Interior Mutability: Mock Objects



### 5.2.2 


## 5.3 
# 6 Reference Cycles Can Leak Memory