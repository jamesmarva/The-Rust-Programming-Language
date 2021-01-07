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
在第四章 “悬挂引用” 的部分，我们提到，编译器要保证没给给引用都是有效的。Unsafe Rust 有两个新的类型被称之为 裸指针（Raw Pointer），这个和引用（Reference）很像。和引用一样，裸指针可以是可变（mutable）的或者不可变的（immutable），分别写作`*const T`和 `*mut T`。这个星号（asterisk） 不是解引用的操作；它是类型名字的一部分。在 裸指针（raw pointer）的使用中，不可变就意味着在解引用之后，指针就不能被重新的赋值。
引用（Reference）与智能指针（smart points）和 裸指针 三者的不同点：
- 允许忽略借用规则，可以同时有可变的和不可变的指针，或者多个指针同时指向一个位置
- 不保证指向一个有效的内存
- 允许裸指针是 null
- 不会自动被清理

通过放弃Rust的以上的强制规范，你可以放弃安全来换取更好的性能（greater performance），或者使用别的语言的接口或者硬件的接口。

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}
```
19-1 如何从一个引用创建可变的裸指针和不可变的裸指针，用as关键字

注意，这里我们并没有用 `unsafe` 关键字，我们可以在 safe 代码里创建裸指针；只不过我们不能在unsafe代码之外解裸指针（dereference raw pointer）。


接下来，我们将会创建一个无法确其有效性的裸指针。在代码19-2展示了如何创建指向任意位置的裸指针。因为是尝试去使用一个未定义的随机的内存：这个有可能是有数据的，也有可能是没有数据的。编译器优化代码之后，这样很可能访问不到数据，或者程序会因为分段出现错误。通常来说，不会有充分的理由来支撑我们写这样的代码，但是现实中也的确有可能出现这样的代码。
```rust
let address = 0x12345usize;
let r = address as *const i32;
```
19-2 创建一个指向随机内存地址的裸指针

arbitrary：任意的
validity ：有效期



```rust
fn main() {
    let mut n = 5;

    let r1 = &n as *const i32;
    let r2 = &mut n as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
```
19-3 在 `unsafe`代码块中解裸指针（raw pointer）

注意19-1会19-3的代码，我们创建一个 `*const i32` 类型和 `*mut i32`类型的裸指针，这两个裸指针都值指向了同一个内存位置，`num`的值存放的内存而为之。如果不是用裸指针，而是创建一个可变的引用和不可变的引用来指向数据，那么会编译不通过的。我们创建了可变的和不可变的裸指针指向一个内存位置，然后可以通过可变的裸指针来修改数据，但是也会潜在 造成数据竞争。

既然有危险，为什么依然要使用裸指针（raw pointers）？其中一个最大的作用的情境就是要调用 c 的代码。下一小节 “调用一个unsafe 函数或者方法”。另一个场景就是创建一个借用检查器无法理解的安全抽象。

## 1.3 调用 Unsafe 函数或者方法 （Calling an Unsafe Function or Method）
unsafe 函数和方法本身看起来和常规的函数和方法没啥区别，只不过多一个额外的关键字声明 `unsfe`。在 `unsafe`就表示在代码所处的语境中，我们需要自己保证函数的安全需求，而Rust不保证我们会按照他们的要求来保持安全。用了关键字`unsafe` 来调用代码，就表示我们已经知道了这个函数功能，并且已经知道了函数的所需的等等。保证我们已经知道了调用函数的不可靠性。



### 1.3.1 Creating a Safe Abstraction over Unsafe Code
如果仅仅是因为函数里面有unsafe的代码，那么不至于把整个函数都是用 unsafe 关键字来声明。事实上，在一个安全的函数里包裹不安全的代码是个很常见的抽象的概念。举个例子，来看看标注包里的`split_at_mut`,这个函数就有用到一些不安全的代码，今天让我们来看看如何实现。
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