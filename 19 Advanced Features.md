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

## 1.2 解裸指针（Dereferencing a Raw Pointer）
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

在这个例子中，我们用一个可变的slice 来指向一个存放`i32` 类型数字的数组，`as_mut_ptr` 方法会返回一个裸指针，这个指针是个 `*mut i32` 类型的裸指针，我们将这个裸指针存在变量 `ptr`中。
注意：这里裸指针的类型 `*mut i32` 的i32是因为slice指向的是数组的存放的值的类型是i32，`*mut` 是因为 `as_mut_ptr`的原因。

我们先确定mid是小于slice的长度。然后我们调用一个非安全的代码，`slice::from_raw_parts_mut`函数有两个参数，一个是裸指针，以及一个 `length`，返回一个`slice`。用这个方法可以创建一个从 `ptr` 指针开始的，`mid` 长度的 `slice`。接着我们调用 `add` 方法来返回一个新的值裸指针，然后在用 `len - mid` 的值来返回一个slice。

`slice::from_mut_parts_mut` 函数是`unsafe`的，因为这个函数调用了一个裸指针，而且必须确保这个裸指针是个有效的裸指针，但是这个指针本身是否有效这个不是Rust来保证的。同理，`add` 方法也是非安全的，因为确保add的参数是有效的，就是要保证在偏移之后的值是有效内存地址。通过增加一个`mid<len`的断言，我们可以确保在偏移之后的内存位置也是在slice之内的。确保正确是 `unsafe` 的一个可以接受以及适当的用法。

注意，这里我们不想要把`split_at_mut`声明为 `unsafe`，在safe Rust的代码里，我们可以调用这个方法。我们创建了一个 safe的函数，这个函数是调用了 `unsafe` 在 safe的模式下，因为我们确保了裸指针只想到内存位置都是有效的。

相比之下，下面这段代码在使用slice就有一定的概率出现错误，我们不能确保这个裸指针指向的位置是有效的。这个指针是指向的一个随意的值的地址。所以很可能有无效的地址出现。

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
静态变量，在Rust中也称之为 全局变量，用关键字`static`关键字来声明的变量。
作用：被多个根部的函数访问value或者修改。
How？
- 如果仅仅是访问的话，那么问题不大，不需要用unsafe关键字进行包裹，就好像下面这个19-9 的这个例子一样。
- 但是如果涉及修改的话，那么情况就复杂了，首先是声明，如果涉及有的函数需要修改变量的话，那么就需要在关键字`static` 后面加上`mut`，这样的静态变量就是个可以被多个函数修改的变量了，而不是仅仅用来只读。但是相应的，当然我们想要修改或者访问这个变量的时候就必须在unsafe的代码块里来使用这个变量了。

为什么要用unsafe关键字?
因为会触发数据竞争（data race）的问题，简单来说就是静态变量被改动导致了结果出现问题，那么Rust编译器不背锅，这就是你自己非要这么写导致的，你自己要保证它是安全的就行。


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



## 1.5 实现非安全的Trait（Implementing an Unsafe Trait）
A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify
当至少有一个方法有一些不变量是编译器无法验证的时候，trait是不安全的

## 1.6 Accessing Fields of a Union

## 1.7  When to Use Unsafe Code

# 2 高级 Trait （Advanced Traits）
## 2.1用关联类型指定一个占位符类型（Specifying Placeholder Types in Trait Definitions with Associated Types）

相比较这章的其他的内容而言，关联类型(accociated type)的出现的场景反而更多些。

## 2.2 默认泛型类型和操作符重载（Default Generic Type Parameters and Operator Overloading）
在使用泛型类型参数这个功能的时候，我们可以指定一个默认的泛型参数。

使用默认泛型参数这个功能，最好的的例子就是操作符的重载（operator overloading）。Rust是不允许你创建自己的操作符的，而且你也不能自己随意重载一个操作符。你可以重载的操作符是有限制的，开发者仅仅能重载`std::ops` 中所列出的特性（trait）。比如，在代码19-14 中，我们重载了 Add（+）操作符，而且把两个 Point 实例相加到一起。
```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```
`Add` 特性有个关联类型的变量叫做 `Output`，这个变量是用来明确 add 方法返回的类型的。

在 Add trait的默认泛型类型的定义
```rust
#[doc(alias = "+")]
pub trait Add<Rhs = Self> {
    /// The resulting type after applying the `+` operator.
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    /// Performs the `+` operation.
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

上面这段代码里里面比较模式的就是 `Rhs=Self`了，这种句法被称之为默认类型变量。`Rhs` 这个泛型类型参数（简称右边`Right hand side`）定义了 `Rhs` 参数的类型。如果在实现 Add trait 的时候没有给Rhs 指定一个具体的类型，那么Rhs也就默认是 Self 的类型，也就是实现 Add的类型。

之前用Point 实现 Add trait 的时候，就没有指定一个具体的类型，因为两个相同的类型，不存在结果用什么类型来保存的问题。那么如果是两个不同的类型进行相加，那么就会有结果到底用什么类型来保存的问题了。比如，假设有两个变量分别是米和毫米，两个相加，最后要保存为以毫米为单位的结果，那么要怎么实现？
```rust
use std::ops:Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, o: Meters) -> Millimeters {
        Millimeter(self.0 + o.0 * 1000)
    }
}

impl Add<Millimeters> for Meters {
    type Output = Millimeters;

    fn add(self, o: Millimeters) -> Millimeters {
        Millimeters(self.0 * 1000 + o.0)
    }
}
```
在两个编码的实现中可能你会用到默认类型参数：
- 在不破坏现有的代码的情况下，扩展一个类型。
- 为了可以在某些特别的场景满足定义类型，但是在大部分的场景都用默认的类型就够了。

标准库里的 `Add` 就是第二种的实现，在大部分的场景中，都是两个相近的类型进行相加，但是在此之上，也依然提供了自定义的能力来完成两个不同类型的相加

第一种的目的看起来和第二中的很相似，但是相反：如果你想要给一个已经存在的trait新增一个类型参数，这样就可以在不破坏已有的代码的前提下完成扩展trait的功能的目的了。

## 2.3 完全限定句法来消除歧义：用相同的名字来调用方法（Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name）

限定语法
why？
如果实现的Trait的方法和结构体的方法出现了相同签名方法，那么当你去用一般的方法去调用方法的时候，默认会调用结构里的方法，比如下面的代码，
```rust
fn main() {
    let h = Human;
    h.fly();
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

// struct 
struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
```
那么如果想要的调用别的trait的方法，。在方法的前面指定trait的名称就可以让Rust知道到底要调用结构体中的哪个放了

可是关联函数也是trait语法的一部分，并且没有默认的self的方法。没了 self 作为参数，那么rust就不知道是调用结构体里的哪个方法了，这个时候就要用到完全限定句法（Fully Qualified Syntax），比如下面这个例子：
```rust
fn main() {
    println!("{}", PetDog::name());
}

trait Animal {
    fn name() ->  String;
}

struct PetDog;

impl PetDog {
    fn name() -> String {
        String::from("moon（明月）")
    }
}

impl Animal for PetDog {
    fn name() -> String{
        String::from("Alaskan Malamute（阿拉斯加雪橇犬）")
    }
}
```
一只宠物狗一般来说有主人命名的名字，比如 `moon（明月）`，身为动物的他们也有一个作为动物的种类名字，比如 `Alaskan Malamute（阿拉斯加雪橇犬）`）。按照上面的使用 `Pet_Doy::name()`的代码，我们只能用结构体的实现的方法，但是如何去使用trait的实现的方法？完全限定句法就是答案。
```rust
fn main() {
    println!("{}", <PetDog as Animal>::name())
}

trait Animal {
    fn name() ->  String;
}

struct PetDog;

impl PetDog {
    fn name() -> String {
        String::from("moon（明月）")
    }
}

impl Animal for PetDog {
    fn name() -> String{
        String::from("Alaskan Malamute（阿拉斯加雪橇犬）")
    }
}
```
`<PetDog as Animal>::name()`这行代码就可以实现调用实现trait的那个实现方法。

总结一下，fully qualified syntax 的具体格式是下面这样的：
```rust
<Type as  Trait>::function(recevier_if_method, next_arg, ...)
```


## 2.4 Using Supertraits to Require One Trait’s Functionality Within Another Trait
当有个trait想用别的trait的未实现的trait的时候，语法应该是怎样的？
比如有个想要将输出格式变换一下，比如有个方法想要打印：
```shell
***************
*             *
* (1111, 222) *
*             *
***************
```
这种是依赖Display的 `to_string` 方法的。那么应该如何使用别的trait的里的方法？
```rust
use std::fmt::{Display, Formatter, Result};

trait OnlinePrint: Display {
    fn online_print(&self) {
        let str = self.to_string();
        let len = str.len();
        println!("{}", "*".repeat(len + 4)); 
        println!("*{}*", " ".repeat(len + 2)); 
        println!("* {} *", str);
        println!("*{}*", " ".repeat(len + 2)); 
        println!("{}", "*".repeat(len + 4)); 
    }
}

struct Point {
    x: u32, 
    y: u32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Position {
    longitude: f32,
    latitude: f32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}

impl OnlinePrint for Point {}

fn main() {
    let p = Point{
        x: 1111,
        y:222,
    };
    
    p.online_print()
}
```
Listing 19-22: Implementing the  OutlinePrint  trait that requires the functionality from  Display


## 2.5 用 `Newtype` 来实现外部的type 实现 trait （Using the Newtype Pattern to Implement External Traits on External Types）




# 4 Advanced Functions and Closures
接下来，我们来学些更高级的关于函数和闭包的特征，包括函数指针（function pointers）和返回闭包（closures）

## 4.1 函数指针 （Function Pointers）
We’ve talked about how to pass closures to functions; you can also pass regular functions to functions! This technique is useful when you want to pass a function you’ve already defined rather than defining a new closure. Doing this with function pointers will allow you to use functions as arguments to other functions. Functions coerce to the type fn (with a lowercase f), not to be confused with the Fn closure trait. The fn type is called a function pointer. The syntax for specifying that a parameter is a function pointer is similar to that of closures, as shown in Listing 19-27.

我们已经谈论过了如何将一个闭包传递个函数；你也可以传一个常规的函数给一个函数。这个技术当你想要传一个已经定义好的函数作为参数给另一个函数的时候是很有用的，因为没有这个技术的话，那么想要作为参数传给函数只能去重新定义一个闭包了。这个技术可以让你用将一个函数作为参数传递给另一个函数。Function是表现为类型 `fn`（注意这是个小写的 f），不要和 `Fn` 闭包 trait 混淆。`fn` 类型被称之为函数指针（function pointer）。指定函数指针作为参数的语法和指定闭包（closure）的语法相似，就像代码19-27 里面展示的那样：
```rust
fn main() {
    let a = do_twice(add_one, 5);
    println!("{}", a);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
```
19-27 

不像一个闭包，fn 是一个类型而不是一个 trait，所以我们直接指定 fn 作为参数而不是指定一个泛型类型参数然后用一个 `Fn` 然后进行trait绑定。

函数指针实现了所有闭包trait（`Fn`， `FnMut`， `FnOnce`），所以你可以将函数指针传递给一个想要闭包的函数参数来使用。写代码的最好的方式就是用泛型和一个闭包trait，这样函数既可以接受一个函数作为参数也可以接受一个闭包作为参数。

举个只能接受 `fn` 而不是闭包的例子，就是在调用外部代码，而这个代码是无法使用闭包这个特性的：就比如 `C` 这就是没有闭包这个特性的语言。

举个你可以选择定义一个内联闭包的（其实就是感觉匿名闭包）或者选择一个已经定义好的函数来作为参数的例子，下面是用闭包的例子
```rust
let l_of_num = vec![1, 2, 3];
let l_of_string: Vec<String>  = l_of_num.iter()
    .map(|x| x.to_string())
    .collect();
for s in l_of_string.iter() {
    println!("{}", s);
}
```
还有声明一个函数来代替这个闭包
```rust
let l_of_num = vec![1, 2, 3];
let l_of_string: Vec<String>  = l_of_num.iter()
    .map(ToString::to_string)
    .collect();
for s in l_of_string.iter() {
    println!("{}", s);
}
```
注意，这里我们就要用到我们之前“高级 traits”提过的，完全限定语法了，因为有很多函数都被命名为 `to_string`，而我们要用的则是在 `ToString` 中的 `to_string`，在标准库（standard library）中，所有的实现了 `Display` 的类型都会实现这个trait。

## 4.2 返回闭包（Returning Closures）





