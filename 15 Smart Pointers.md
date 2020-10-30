














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









