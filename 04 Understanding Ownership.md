所有权(Ownership) 是Rust最独特功能，它使得Rust无需进行垃圾收集器就可以保证内存的安全。因此，了解所有权（OwnerShip）在Rust中的工作方式非常重要。在本章中，我们将会讨论关于所有权的几个功能：借用(borrow)，切片（slice），以及Rust在内存中的是如何布置数据的。
 
# 1. 什么是所有权 (What Is Ownership?)
Rust 一个主要的特性就是所有权(ownership)。尽管这个特性看起来容易理解，但是实际上不是，并且他对于这门语言的功能有着非常重要的影响。

所有的程序都必须在运行程序的时候管理他们所用的内存。某些语言有垃圾回收的功能，这种垃圾回收机制会不断的寻找不再被使用的内存(比如Java)。还有别的语言，开发者必须要显式的分配和释放内存。Rust 用了一个第三种的途径: 通过拥有一整套在编译的时候检查的规则的 **所有权系统** 来管理内存。

因为所有权(ownership)对许多程序员的新概念，所以它确实需要一些时间来习惯：这对于开发者来说是一个好消息，也是一个坏消息。坏消息是因为所有权对于大多数的开发者来说是个全新的概念，所以需要花一些时间才能习惯；而好消息就是，一旦你适应了这个**所有权系统**，你就能开发出安全高效的代码。

当你彻底明白了所有权(OwenerShip)之后，你会对 Rust 独树一帜的功能的有更深层次的理解。在本章中，你会通过一些字符串的示例来学习 **所有权**。

> ### 栈与堆 The Stack and the Heap
> 在很多编程语言中，你不用过多的考虑关于堆和栈是如何运行的。但是像 Rust 这个一个系统编程语言中，一个值在堆中还是在栈中会对这个值接下来的行为以及做出正确抉择产生巨大的影响。所有权(ownership)的各个部分的知识会在本章的后面的堆和栈中介绍，这里的只是做一个简要的说明。
> 
> 堆(heap) 和 栈(stack) 都是程序在运行时会使用到的内存的部分，但是他们的组成结构不同。栈(stack) 是存、取数的顺序是后进先出的，也就是说，先入栈的数据反而后面才能取到。想象有一堆盘子是堆叠放置的，如果你想要堆更多的盘子，那么后来的盘子只能放在已有的盘子的顶部，不断的累积；如果你想要取盘子，你肯定不会选择这一叠中最底部的去取，你会取最上面的那个盘子，这就是典型的栈的结构。添加数据被称为 *入栈(pushing onto the stack)*，取出数据被称为 *出栈(popping off the stack)*
> 
> 所有保存在栈(stack) 的数据必须要有已知固定的大小。相反，如果在编译期还无法确定大小，或者大小会根据程序的运行而改变的数据就要保存在堆(heap)。堆不善于利用空间： 当你需要把数据放在堆(heap) 的时候，你必然需要一定量的内存空间。内存分配器会在堆中寻找一个足够大的空白区域，然后将其标记为正在被使用，然后返回一个 `指针(pointer)`，这个指针指向的就是内存的地址。这个过程称之为 *在堆上分配分配(allocating on the heap)* ，有时候也简称为 *分配(allocating)*。把值压入栈(stack) 的过程不视为分配。因为指针是大小已知的，所以就可以吧指针存在的栈上，但是当你想要实际数据的时候，必须要用指针进行操作。
> 
> 把数据压入栈的效率要比在堆上分配数据的效率高，因为栈的分配器不需要去搜索用来存储数据的空白的空间。把数据存在栈上，这个数据始终都直接在栈的顶部，而且大小已知，相比之下，在堆上分配空间则需要更多的操作，堆分配器必须要先寻找一个足够大空间以备容纳数据，然后把位置信息记录下来，以备下次分配空间用。
> 
> 访问堆中的数据的速度也会比访问栈上的数据的速度慢，因为你必须要通过获取指针指向的地址，然后才能到堆中的相应的位置取得数据。处理器如果在内存中访问的越少，那么速度就会越快。以此类推，你想象有个服务生要在多张桌子上处理单子。最有效的做法就是，处理完所有A桌子的订单，再去B桌子的订单，减少走动；如果是从A桌子接了一个单子，再去B桌子接了一个单子，然后再去A桌子接一个单子，这样效无疑非常的低(时间都浪费在走路上了)。
> 
> 当你调用函数的时候，传递给函数的值和函数的局部变量都会被压入栈中，在函数调用之后，这些参数会被出栈
> 
> 所有权要解决的问题就是，跟踪那些代码的数据是保存在堆里的，最大程度上减少堆的重复的数据，并且清理对上的未使用的的数据，以防止内存空间用尽。一旦你理解了所有权(ownership)，你就不必经常考虑堆和栈了，但是知道如何管理堆的数据可以帮助你理解所有权如何运作的。


## 1.1 所有权的规则 (Ownership Rules)
首先，先来看看所有权的规则。在阅读下面的示例的时候，请把这些规则记在脑子里。

1. 每个在Rust 的value 都有一个变量(variable)，这个变量称为它的所有者。
2. 在同一时间只能有一个所有者
3. 当所有者离开空间的时候，这个值就被删除

## 1.2 变量的作用域 (Variable Scope)
作为所有权(ownership)的第一个代码示例，先来看看一些变量的 *域(scope)*。 域(scope)是指程序的中的元素的有效范围。来看看下面的这个变量:
```rust
let s = "hello";
```
这个变量 `s` 表示的是一个字符串，这个字符串的值是被硬编码到我们的程序的文本中的。这个变量在声明的地方开始，到当前的 *域(scope)* 结束都一直有效的。代码4-1用注释来解释了这个域的有效范围
```rust
fn main() {
    {                      // s 在这里是无效的，因为还未声明
        let s = "hello";   // 从里开始是有效的

        // 对 s 进行一系列操作
    }                      // 此时 域 结束了，变量 s 就不再有效了
}
```
代码 4-1：一个变量以及域的有效范围

换句话说，这里有两个重要的时间点：
- 当 `s` 出现在 *域(scope)* 的时候，他是有效的
- 一直有有效到它离开域

在这一点上，域和域之间的关系，以及变量的有效的时间点，这两点就和别的语言里的这两点有的相似。

## 1.3 字符串类型(The String Type)
我们需要一个比较复杂的数据类型来说明所有域(ownership)的规则，这种数据结构要比之前的第3章“数据类型” 一节中的数据类型更加复杂。前面的介绍的那些数据类型都是保存在栈中的，我们更想知道数据是如何保存在堆中的，并且弄明白 Rust 是何时，通过何种方式清理这些在堆里的数据的。

我们这里以 `String` 这个数据类型作为示例，并且集中讨论和所有域相关的字符串的部分的内容。当然讨论这些概念也同样适用于别的复杂的数据类型，不管它们是标准库提供的还是你自己创建的数据类型。我们将会在第8章深入讨论 `String`

在上一个小节中，我们已经看到了字符串文字(string literals)类型了，那个字符串文字(string literals)是被硬编码到我们的程序中的。字符串的文字是很方便，但是并不适用于我们可能要使用字符串的情况的。原因之一就是他们是不可变的。另一个原因就是编码的时候，并不是每个字符串的值，我们都是在编码阶段就知道的。比如，我们想要获取用户输入的字符串，以及存储这个值，这时候要咋办？对于这些情况，Rust 提供第二字符串类型， `String`。这种类型是在堆上分配的，因此可以存储在编译的时候我们不知道大小的文本数据。用字符串文字和`from` 函数来创建一个`String` 类型的变量

```rust
fn main() {
    let s = String::from("hello");
}
```
双冒号(::)是一种运算符，允许我们可以使用 `String` 类的特定函数 `from` 函数，而不是 `string_from` 之类的名称。我们将会讨论更多讨论这个语法在在第5章中。

这种字符串类型是可变的：
```rust
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
}
```
所以，这里的两个字符串的类型有什么区别？为什么 `String` 可变，而 文字类型(literal) 不可变？区别就在于这两种类型是如何使用内存的。

## 1.4 内存以及分配(Memory and Allocation)
对于字符串的文字类型(string literal)，我们在编译的是就知道什么内容，因此文本内容是直接被硬编码到最终执行文件中。这就是为什么字符文字(string literal)类型运行快速以及高效的原因。但是，这些优点仅仅是来自字符串的不可变的性。但是对于在编译的是不知道具体的大小的，并且在运行程序的大小可变的字符串，就无法在二进制文件中进行编码。

对于 `String` 类型，为了可以支持可变的，可增长的文本，我们需要在堆上分配一个在编译器为知的内存量来容纳内容。这就意味着：
- 必须在运行的时候向内存分配器来获取内存。
- 在完成 `String` 的操作之后，我们需要一种把内存返回给分配器的方法。

第一部分是由我们来完成：当我们调用 `String::from` 函数的时候，它就会实现获取所需的内存。这个点几乎在编程的语言里几乎都是共通的。

但是第二部分不同。在有*垃圾回收器(garbage collector)* 的语言中，垃圾回收器会跟踪并且清理不在使用的内存，因此就不用考虑它了。在没有垃圾回收器的语言下，我们就需要确定合适不再使用内存了，并且调用代码来显式返回这个内存，就像用代码去请求这个内存的时候一样。从以往的经验来看，正确执行这个操作一直是个编程难题。如果忘记了释放内存，那么就会造成浪费内存。如果释放内存太早了，那么就会出现无效变量的情况。如果释放两次，那么也是一个错误。编程的时候，要达到正确的话，那么就必须一次 *申请(allocate)* 和一次 *释放(free)* 要一一对应。

Rust 用的是另一种方式：当持有内存的地址的引用离开域的情况下，就会释放内存。下面是代码4-1的另一个版本：
```rust
fn main() {
    {
        let s = String::from("hello"); // 此时开始s是有效的

        // 执行对于 s 的操作
    }   // 域结束，s 不再有效
}
```
程序可以很自然的把 `String` 的内存返回给 分配器(allocator):当 `s` 离开域的时候。当一个变量离开域，Rust 会调用一个特定函数 `drop`，这是 `String` 的作者可以放置代码来返回内存的函数。Rust 会在大括号的结束的地方自动调用 `drop`。

这样的编程模式对于Rust的代码编写有          深远的影响，现在看似很简单，但是如果有多个变量在堆上继续分配的时候，复杂的情况下的代码也许会出乎意料。

### 1.4.1 变量和数据交互的方式: 移动 (Ways Variables and Data Interact: Move)
多个变量可以用多种方式来和同一个数据进行交互。来看看代码4-2的例子：
```rust
fn main() {
    let x = 5;
    let y = x;
}
```
代码4-2 将变量 `x` 赋值给变量 `y`

我们可以大概猜到这段代码做了什么事情：把 值 `5` 和 `x` 绑定，然后创建一个在 `x` 里的复制的值，然后和`y`绑定”。我们现在有两个变量，`x` `y` 都是等于 `5`。事实就是发生了上面这个事情，因为整数是有已知大小的类型，然后这两个 `5` 的值都被压入了栈。

现在来看看 `String` 类型的版本：
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
}
```
这段代码看起来和上一段的代码有点像，所以我们先假设他们的工作的原理是原理谁相同的：就是，在第二行的代码中，创建了一个在 `s1` 的复制的值，然后和 `s2` 变量绑定。但是这个不可能发生的事情。

图4-1 ，看看 `String` 幕后发生了什么。字符串是由三个部分组成的，如左图所示，

![在这里插入图片描述](pic/ch04/trpl04-01.svg)
图4-1 绑定内存里表示字符串“hello”到变量`s1`

length就是当前的 `String` 的内容所使用的字节的数量。容量（capacity）就是从内存分配器分到的总的字节数量。容量 和 长度的差别是个很重要的内容，但是此时此地不是非常重要，现在忽略二者从差别是允许的。

当我们把 `s1` 赋值给 `s2` 的时候，字符串被复制，但是这个仅仅意味着我们复制了指针，长度以及栈的容量。不没有复制指针所指向的堆上的数据。内存的数据如图4-2所示：

![在这里插入图片描述](pic/ch04/trpl04-02.svg)
图4-2 在内存里表示变量 `s2` 的复制了 `s1` 指针，长度，以及容量。

这种表示并不像 图4-3 那样，如果Rust 选择了复制堆上的数据，那么表示也许会想 图4-3 那样。但是如何 Rust 这样做的话，那么堆上的数据如如果很大的话，那么运行时候的赋值操作就会很昂贵。
 ![在这里插入图片描述](pic/ch04/trpl04-03.svg)
图4-3 另一种可能的表示，如果Rust选择复制数据，

之前，曾经提到过，如果一个变量超出了 作用域(scope)，那么，Rust就会自动调用 `drop` 函数以及清楚堆上的数据。但是如果按照 图4-2所指示的那样的话，那么就会产生一个问题，如果s2 和 s1 都超出了 作用域(scope)，那么他们都会超过释放同一内存空间，那么情况被成为 *double free error(双重释放错误)* 是我们之前提到的内存安全性错误之一。释放同一片重复两次会导致内存损坏，从而导致安全漏洞。

为了确保内存安全，还有一个细节是值得一说的。与单纯的复制不同，当 `s1` 赋值给 `s2` 的时候，那么Rust就认为 `s1` 不在有效，所以当 `s1` 超出作用域的时候，Rust是不会释放任何东西的。当我们在创建了`s2`之后，再次使用 `s1` 的话，那么就会出现错误：
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```
你会得到一个错误，就像下面一样，因为 Rust 会阻止你使用一个无效的引用。
```shell
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 | 
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership`.

To learn more, run the command again with --verbose.
```
如果你在使用其他语言的时候听说过 浅复制() 和 深复制()，那么复制 指针、长度和容量而不复制数据的这种情况，听起来有点像浅复制。但是由于 Rust 会让第一个变量无效(比如，s1 赋值给 s2后，s1就无效了)，这种情况也不能称之为 浅复制。所以被称为 *“搬迁(move)”*。在这个例子中，我们称之为 `s1` 搬迁到 `s2`。就像图 4-4 的所示的那样：

 ![在这里插入图片描述](pic/ch04/trpl04-04.svg)
图4-4 `s1` 无效之后在内存中的表现形式

这样解决上面说的那种释放两次的问题，因为这样之后，只有 `s2` 有效，当 `s2` 超出作用域的时候，s2 是单独释放内存的。

另外，这这种设计也就意味着：Rust 永远不会自动创建一个深度拷贝，所以在运行时候的任何自动复制都是廉价的

### 1.4.2 变量和数据的交互方式：克隆(Ways Variables and Data Interact: Clone)
如果我们想要深度拷贝堆里面数据，而不是仅仅是栈的数据，则可以使用一种通用的方法 `clone`。我们将会在第五章里讨论这个方法，由于这个方法是许多编程语言的常用方法，所以也许你已经见过它们了。

这里就有一个 `clone` 方法的代码例子：
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```
这样工作的就产生了图 4-3 的行为，并且确实在堆里面复制了这些数据。

当你看到调用克隆 `clone`的时候，就要知道他正在执行某些代码，并且这个操作是昂贵的。

### 1.4.3 在栈上的数据：复制(Stack-Only Data: Copy)
还有一种情况我们没有讨论到。就是当代码里面用到整型的数字时候，下面的代码展现了这种情况：
```rust
fn main() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
```
这段代码似乎和我们所学的是相冲突的，我们并没有调用 `clone` 方法，但是变量 `x` 仍是有效的，值并没有搬迁到 变量`y`中。

不会出错的愿意就在于，在编译的时候已知大小的数据类型是完全存储在栈上的，因此可以快速创建实际值的副本。这就意味着在创建了变量 `y` 之后，我们没有理由在让变量 `x` 失效了。换句话说，在这里的深拷贝和浅拷贝没有差别。

Rust 有个特殊的声明，称之为 `Copy` 特征，我们可以把它想整数类型一样，存储在栈上(将在第10章中讨论)。如果一个类型有 `Copy` 特征的话，那么在被赋值给别的变量之后，仍然是有效的。Rust 不会让我们声明一个类型是同时有 `Drop` 特征和 `Copy` 特征。如果某个类型需要在超出作用域(scope)的时候执行某些特殊的操作话，那么此时给这个变量增加 `Copy` 特征声明是会报错的。要了解如何添加 `Copy` 特征到你的类型中，请看附录C的 [“可衍生特征(Derivable Traits)”](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)。

所以，到底那些类型有`Copy`特性？你可以通过文档来确定，但是有个毕竟大概的规则，简单的 `scalar` 值是就有 `Copy` 特性，以及一些不需内存分配器的资源就有 `Copy` 特征。下面有些确定是有 `Copy` 特征的类型：

- 所有的整数类型，比如 `u32`
- 布尔类型，比如 `bool`
- 所有的浮点类型，比如 `f64`
- 字符类型，`char`
- 元组类型，前提是元组里保存的类型有 `copy` 特征。比如 `(i32, i32)`就有 `copy`特征，但是 `(i32, String)` 就没有。

## 1.5 所有权和函数 (Ownership and Functions)
把值传给函数的语法和给变量赋值的语义很相似。就像赋值一样，把变量传给一个函数的时候，数值会发生复制(copy)，或者搬迁。在代码4-3 中有一个带有注释的例子，其中显示了变量进入和离开作用域(scope)的位置。
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function..
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```
↑ 代码 4-3：带有注释的函数所有权和作用域的代码

如果我们在调用了 `takes_ownership` 函数之后再去调用变量 `s`，Rust 会抛出一个编译时的错误。这些静态检查可以防止我们放错误。尝试添加 使用 `s` 和 `x` 的代码到main函数中，以查看他们在何处使用他们，以及在何处所有权的规则会阻止你使用这些变量。

## 1.6 返回 值和作用域 (Return Values and Scope)
返回值的时候同样也会把所有权返回。代码清单 4 - 4 是和 4-3 一样的带有注释的示例。
```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```
↑ 代码清单 4-4：转移返回值的所有权

变量的所有权 每时每刻都遵循相同的模式的：把一个值赋值给另一个变量就会发生搬迁(move)。当指向堆中的数据的变量超出了作用域的时候，那么就会被 `drop` 方法清除在内存中数据了了，除非该数据已经被转移给另一个变量。

每个函数都获取所有权然后返回所有是非常繁琐的操作。如果我们仅仅是让函数使用变量的值而不是所有权怎么办？令人十分烦恼的是，我们如果想在再次使用这个变量，并且让函数的代码在执行中不产生任何额外的数据，那么我们只能返回它。

可以用元组来返回多个值，请看代码清单 4-5：
```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```
↑ 代码 4-5：返回参数的所有权

对于这样一个普遍的功能来说，这样的入参和返回有太多的不方便了，比如每次都要用元组把所有权和别的返回值一起返回，这样明显不方便。幸好Rust有一个解决的方法，那就是“Reference” 


# 2. 引用和借用 References and Borrowing
Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:
获取一个对象的引用，不是获取这个变量的所有权，也就是得到这个对象的值？

```java
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

We call having references as function parameters borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.

So what happens if we try to modify something we’re borrowing? Try the code in Listing 4-6. Spoiler alert: it doesn’t work!

### 2.1 可变引用 Mutable References
我们只需要稍稍修改就可以解决 代码清单4-6 中的错误了:
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
首先，我们必须修改 变量 `s` 为可变的 `mut`，

### 2.2 悬挂引用 Dangling References

# 3 切片类型 (The Slice Type)
另一种没有所有权的数据类型是 *切片*。切片让你可以引用到集合中部分连续的元素序列，而不是整个集合。
先来看看一个编程的小问题，：编写一个函数，传入一个字符串变量，然后返回一个在字符串中找到的第一个单词。如果这个函数没有在这个字符串中找到一个空格，那么整个字符串就被看作是一个单词，那么就要返回整个字符串。

这里有小问题：编写一个函数，这个函数必须要使用一个字符串并起返回这个在这个字符串中找到的第一个单词。如果这个函数没有在这个字符串中找到一个空格，那么整个字符串就被看作是一个单词，那么就要返回整个字符串。
先来看看这个函数的签名：
```rust
fn first_word(s : &String) -> ?
```
这个函数`first_word`，有一个 `&String` 类型的参数。我们不想要这个参数的所有权，同时也可以使用这个变量的数据，所以这样做很符合我们的需求。但是我们要返回什么？我们现在还没办法返回字符串的一个*部分*(因为还没学)。但是我们可以返回这个单词的最后一个字母所在的索引。就像 代码4-7 里面写的那样。
```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn main() {}
```
Listing 4-7: 这个方法返回字符串的的首个单词的最后的一个索引

因为我们需要逐个检查 `String` 的元素，并且同时检查是否为空格，所以要需要先用 `as_bytes` 把字符串`String` 转换为一个字节的数组
```rust
let bytes = s.as_bytes();
```
接着，我们用 `iter` 方法 创建了一个字节数组的迭代器：
```rust
for (i, &item) in bytes.iter().enumerate() {
```
我们会在的第十三章讨论更多关于 迭代器(iterators)的细节。现在，只要知道迭代器会返回集合里每个元素，并且每个元素都是用枚举来包装的，并且是作为一个元组来返回的。第一个在元组里的整型是 索引(index()，第二个是元素的引用(reference)。这个比自己去计算索引要方便一些。
因为 `enumerate`方法返回的是个元组，所以，我们可以用一些模式来破坏这个元祖。所以在 `for` 循环中，我们用了一个方法方式来使用的 `i` 的来通过索引的方式来使用数组的元素，以及用 `&item` 来使用元组中的单个字节数据，因为我们用 `.iter().enumerate()` 得到了元素的引用，所以我们可以用 `&` 在模式中。

在`for`循环中的，我们不断通过字节文字语法(the byte literal syntax)来判断一个字符是否是空格。如果找到了空格，我们就要返回这位置。否则我们就会用 `s.len()` 来返回字符串的长度。
```rust
    if item == b' ' {
        return i;
    }
}
s.len()
```
我们现在有了找到字符串里的第一个单词末尾的索引的方法，但是依然存在问题。我们返回了一个 `usize` 类型的值，这个值只是在有 `&String`的上下文中有意义的。换句话说，如果脱离了 `String` 类型，那么它就变的毫无意义了。比如在代码4-8里面的代码写的那样：
```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```
Listing 4-8: Storing the result from calling the first_word function and then changing the String contents

上面这段代码可以通过编译，就算是我们在 调用了 `s.clear()` 之后继续使用 变量 `word`，也依然不会报错，因为这个数字和 `s` 的状态无关的，`word` 依然是保存着 `5`。我们可以用 这个数字来提取出第一个单词，但是如果字符串的内容发生了改变的话，那么，我们依然用 `5` 这个值来提取首个单词就会产生bug了。
担心 `word` 中的索引会和 `s` 中的数据同步问题是很无聊，而且很容易出错的。如果我们再写一个函数来找出第二个单词，那么管理这些索引的难度就会更大，注意，这里我们的目的是从字符串中获取到单词，所以才去管理这些单词的索引位置的，如果用索引的思路去解决获取字符串中的第二个单词的问题，那么函数的前面就会像下面那样：
```rust
fn second_word(s: &String) -> (usize, usize) {
```
现在我们还必须要实时的跟踪单词的开始索引和结束索引，并且这些值还是在字符串某个状态下面产生的，和字符串的状态没有同步关系。这样一来，我们为了获取字符串里的第一和第二个单词就需要去手动同步是哪个变量，非常麻烦。
幸好，Rust 有个不错的方案来解决这个问题：字符串切片(string slices)。

### 3.1 字符串切片(String Slices)
字符串切片(Stringb slice) 是对字符串的部分数据引用，使用如下：
```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
```
这种情况有点像，我们引用了整个字符串，但是只提取出了[0...5]位(bit)。但是这不是引用了整个字符串，而是对字符长度部分的引用。
通过中括号的中的指定的范围 `[starting_index ... ending_index]`，我们可以创建一个切，`
starting_index` 是切片的第一个位置，而 `ending_index` 要比切片的最后一个位置还要大 1。在内部，切片的数据结构会保存这个切片的在字符串的起始位置，以及这个切片的长度，这长度的数据是根据 `ending_index` 减去 `starting_index` 来得到的结果。因此在使用 `let word = &s[6..11]`后，world将会是一个切片，其中包含了一个指向了s的第7个字节的的指针，而且这个切片的长度为 5。

图 4-6 展现了这种状态：
![切片的引用情况](pic/ch04/trpl04-06.svg)
↑ 图 4-6 字符串切片是对字符串的部分的引用

如果你使用范围语法，如果你想要从0这个索引的位置开始，那么你可以把 0 省略，下面这两种使用的到的值都是相同的。
```rust
#![allow(unused_variables)]
fn main() {
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
}
```
同理，如果是你的切片中的后一个范围是字符的最后一个字节，那么你也同样可以省略这个尾巴的数字。
```rust
#![allow(unused_variables)]
fn main() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
}
```
当然，如果你想有整个字符串切片，那么你可以把头尾两个范围值都省略了。
```rust
#![allow(unused_variables)]
fn main() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
}
```
> 注意：这里的字符串的切片的索引必须都是 UTF-8 字符的边界处。如果在多字节字符的中间创建字符串的片段的话，程序将退出，并且显示错误。为了引入字符串切片，我们都假定本小节中的都是ASCII。在第八章中的“[用字符串来保存UTF-8编码格式的文本](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)”，我们将会详细讨论对 UTF-8 的处理。

通过以上的这些信息，我们就用切片来重写上面的这个寻找第一个单词的程序了。表示 “字符串切片” 的类型的标识为 `&str`：
```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn main() {}
```
We get the index for the end of the word in the same way as we did in Listing 4-7, by looking for the first occurrence of a space. When we find a space, we return a string slice using the start of the string and the index of the space as the starting and ending indices.


Now when we call first_word, we get back a single value that is tied to the underlying data. The value is made up of a reference to the starting point of the slice and the number of elements in the slice.

Returning a slice would also work for a second_word function:
```rust
fn second_word(s: &String) -> &str {
```
We now have a straightforward API that’s much harder to mess up, because the compiler will ensure the references into the String remain valid. Remember the bug in the program in Listing 4-8, when we got the index to the end of the first word but then cleared the string so our index was invalid? That code was logically incorrect but didn’t show any immediate errors. The problems would show up later if we kept trying to use the first word index with an emptied string. Slices make this bug impossible and let us know we have a problem with our code much sooner. Using the slice version of first_word will throw a compile-time error:
###### String Literals Are Slices

### String Slices as Parameters


## Other Slices


# 4 Summary

