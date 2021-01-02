- UnSafe Rust:
- Advanced traits:
- Advanced types:
- Advanced function and closures:
- Macros:

# 1 不安全Rust （Unsafe Rust）
到目前为止，我们所有涉及的代码都是在编译期就强制保证了内存安全的。

## 1.1 unsafe 的力量 （Unsafe Superpowers）
如果你想写出Rust的不安全的代码，那么就要用关键字 `unsafe`，然后用一个新的块来放置不安全的代码。在 unsafe Rust中，你可以执行以下的五个操作，这些都操作被称之为 *unsafe superpower*，当然这些操作在 safe Rust 中是不能通过编译的。
- 解一个裸指针（Dereference a raw pointer）
- 调用不安全的函数或者方法（Call an unsafe function or method）
- 读取或者修改一个可变的静态变量（Access or modify a mutable static variable）
- 实现一个不全的trait（Implement an unsafe trait）
- 读取一个 `union` 的字段（field）（Access fields of `unions`）

如果你在 `unsafe` 的代码区域里使用了一个引用的话，这个引用仍然会被借用检查器检查。
你要知道的是，`unsafe` 关键字仅仅是让你使用以上五种特性的时候不会被编译器检查是否内存安全。在unsafe的代码块里你会仍然会获得一定程度的 safety。


## 1.2 Dereferencing a Raw Pointer
在第四章 “悬挂引用” 的部分，我们提到，编译器要保证没给给引用都是有效的。Unsafe Rust 有两个新的类型被称之为 裸指针（Raw Pointer），这个和引用（Reference）很像。和引用一样，裸指针额可以是可变（mutable）的或者不可变（immutable）的。
不同于引用（Reference）和 智能指针（smart points），裸指针：
- 允许忽略借用元祖，可以同时有可变的和不可变的指针，或者多个指针同时指向一个位置
- 不保证指向一个有效的内存
- 允许裸指针是 null
- 不会自动清理

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as &mut i32;
}
```
19-1 


## 1.3 Calling an Unsafe Function or Method


### 1.3.1 Creating a Safe Abstraction over Unsafe Code
```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
```
19-4 使用安全的函数 `split_at_mut`



```rust
fn split_at_mut(slice: mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    
    assert!(mid <= len);

    (&mut slice[..mid],
    &mut slice[mid..])
}
```
19-5 尝试使用 safe 的代码来实现 `split_at_mut`

当我们编译 19-5 的代码的时候，会得到下面的这个错误：
```shell
$ cargo run
   Compiling unsafe-example v0.1.0 (file:///projects/unsafe-example)
error[E0499]: cannot borrow `*slice` as mutable more than once at a time
 --> src/main.rs:6:30
  |
1 | fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  |                        - let's call the lifetime of this reference `'1`
...
6 |     (&mut slice[..mid], &mut slice[mid..])
  |     -------------------------^^^^^--------
  |     |     |                  |
  |     |     |                  second mutable borrow occurs here
  |     |     first mutable borrow occurs here
  |     returning this value requires that `*slice` is borrowed for `'1`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.
error: could not compile `unsafe-example`.

To learn more, run the command again with --verbose.
```


```rust
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    
    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}
```
19-6 在 split_at_mut 中使用不安全的代码

上面的代码，在这种情况下，因为有个保存数据类型是 `i32` 的可变的slice，所以，`as_mut_ptr` 会返回一个类型是 `*mut i32` 的裸指针（row pointer），存储在 `ptr` 中。


```rust
use std::slice;

let address = 0x01234usize;

let r = address as *mut i32;

let slice: &[i32] = unsafe {
    slice::from_raw_parts_mut(r, 1000);
}
```
19-7 通过任意内存地址来创建slice


### 1.3.2 用 `extern` 函数来调用外部的代码（Using `extern` Functions to Call External Code）


```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
      println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```
19-8 



## 1.4 访问或者修改一个可变的静态变量（Accessing or Modifying a Mutable Static Variable）
```rust
static HELLO_WORLD: &str = "hello world!";
fn main() {
    println!("name is: {}", HEllO_WORLD);
}
```
19-9 


```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER:{}", COUNTER);
    }
}
```
19-10 Reading from or writing to a mutable static variable is unsafe.



## 1.5 Implementing an Unsafe Trait

## 1.6 Accessing Fields of a Union

## 1.7  When to Use Unsafe Code

# 2 高级 Trait （Advanced Traits）

## 2.1 Specifying Placeholder Types in Trait Definitions with Associated Types