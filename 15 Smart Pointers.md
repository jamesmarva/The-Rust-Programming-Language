














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


