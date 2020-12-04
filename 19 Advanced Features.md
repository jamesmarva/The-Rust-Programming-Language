- UnSafe Rust:
- Advanced traits:
- Advanced types:
- Advanced function and closures:
- Macros:

# 1 不安全Rust （Unsafe Rust）
到目前为止，我们所有涉及的代码都是在编译期就强制保证了内存安全的。

## 1.1 Unsafe Superpowers

- Dereference a raw pointer
- 调用不安全的函数或者方法
- 读取或者修改一个可变的静态变量
- 实现一个不全的trait
- 读取一个 `union` 的字段（field）

## 1.2 Dereferencing a Raw Pointer

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



## 1.4 Accessing or Modifying a Mutable Static Variable

## 1.5 Implementing an Unsafe Trait

## 1.6 Accessing Fields of a Union

## 1.7  When to Use Unsafe Code

# 2 高级 Trait （Advanced Traits）

## 2.1 Specifying Placeholder Types in Trait Definitions with Associated Types