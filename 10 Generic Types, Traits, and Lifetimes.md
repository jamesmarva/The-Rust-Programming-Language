每种编程语言都有自己方式来解决代码重复的问题。在 Rust 中，以后以后总解决方案就是 *泛型(generics)*。泛型(generics)是一种具体类型或者其他属性的抽象的一种替代方式。在写代码的时候，我们可以用编写好一些泛型的行为，以及，泛型与泛型之间的关系，以及不用在编译和运行的时候知道的这些泛型将会取代什么。

函数可以用还不知道具体类型的参数，来运行相同代码段，但是传参的数据类型不同的代码，函数可以用一些 泛型 类型来代替一些具体类型。实际上，我们在第 6 章 的 `Optoin<T>`，在第 8 章中用的是 `Vec<T>`以及 `HashMap<K, V>`，在第9章中用的是 `Result<T, E>`，在本章中，我们将继续探索要如何定义自己的带泛型(generics) 的类型，函数以及方法。

首先，先来看看如何通过提取函数来减少冲U代码，接下来，将用相同的技术来从两仅仅是参数的不同的函数来创建一个通用的函数。同时，我们也会介绍关于如何在 结构体(struct) 和 枚举 (enum，比如 `Result`) 定义中使用泛型。

然后，你学习如何用 特性(trait) 定义一般的行为。你可以将特征与泛型类型结合使用，以泛型 进行类型限制，而不是任何类型,

最后，将会讨论 *生命周期(lifetimes)*，生命周期得就是各种泛型，他们为编译器提供给了引用之间如何关联的信息。生命周期让我们可以在许多的情况下借用值，同时仍使编译器能够检查引用是否有效。

# 0 通过提取函数来说删除重复项 (Removing Duplication by Extracting a Function)
在深入了解泛型语法之前，先来看看如何通过提取函数来删除不涉及泛型类型的重复代码。 然后我们将用这种方法来提取一个泛型函数。同时，你将认识到把重复代码提取到一个函数当中，你会开始认识到，可以用泛型来消除重复代码

下面是一个简短程序，主要用来找数组中的最大数字：
```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    assert_eq!(largest, 100);
}
```
代码 10-1:找出数组中的最大元素
 这段代码保存了一组整型的数组在变量 number_list 中，以及把第一个元素当成这个数组的最大的元素，并且保存进变量 `largest` 中。然后进行遍历数组的所有元素，如果遍历到的元素大于当前的 `largest` 变量，那么就进行赋值给 largest 这个变量。在这个例子中，这最大值就是100。

 如果在两个不同数组中查找最大值，我们可以重复用上面代码。代码10-2 所示
 ```rust
 fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}

 ```
代码 10-2查找两个数组中的最大的元素。

尽管代码是可以跑的，但是重复的代码不仅很繁琐，而且很容易出错。

为了消除这些重复的代码，我可以定义函数来吧这段代码逻辑抽象出来，这个函数的可以对他的操作进行任意的操作。这样会让我们的代码更加的清晰，并且让我们更加抽象地把数组中的最大元素提取出来的作为一种概念存在。
在代码10-3 中，我们把找到数组中的最大的元素的代码提取到函数 `maximum` 中，

```rust
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 6000);
}

```
代码10-3 从两个数组中查找最大元素

函数 `largest` 有个叫做 `list` 的参非，它表示我们可以传递任何一个 `i32` 的数组切片给这个函数。作为结果，这个函数将在这个我们传的特定的值上运行。

总结一下，一下是 代码10-2 重构到 代码10-3 的步骤：
1. 确认重复的代码
2. 把重复的代码提取到函数当中，并且指定好函数的输入和返回值
3. 更新重复代码到这个函数当中。

下面，我们将对泛型(generics) 用相同的步骤来减少在不同的情况下的重复的代码。

比如我有两个函数，一个是在 `i32` 切片中找到最大的元素，一个是在 `char` 的切片中寻找最大的元素。要如何如何消除重复的代码？

# 1 泛型数据类型(Generic Data Types)

用泛型来创建函数签名或者结构体，这样我们就可以用多种不同的具体数据类型了。先来看看如何用泛型来定义函数，结构体，枚举类型，以及方法。然后在讨论泛型是如何影响代码的性能的。

### 1.1 函数定义(In Function Definitions)
当定义泛型函数的时候，需要把泛型放到函数的签名当中，通常用来指定这个函数的参数和返回值的数据类型。这样使用会让我们的代码变得更加灵活并且在给使用者带来更能多的功能的同时也让代码变得更加简洁。

继续上面的 `largest` 函数，代码10-4 就展示了两个来找到两个切片中的最大值。
```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');
}
```
代码 10-4 两个函数的区别在于他们的名字和他们的签名不同。

`largest_i32 `函数是把代码10-3 的代码 提取出来，来寻找切片中的最大的 `i32` 的数据类型的值。`largest_char` 函数就是在切片中找最大的 `char` 类型的值。这两个函数体有相同的代码，让我们用泛型参数来减少重复的代码。

想要在定义的新的函数中进行参数化，就需要对类型参数进行命名，就像在函数中的对值参数进行命名一样。你可以用任何一种标识符来作为类型参数的名字。但是这里将会用 `T`，因为从惯例的角度来说，Rust 中的参数名字普遍很短，通常只有一个字母， Rust 的命名习惯是 驼峰命名法(CamelCase)。`T` 就是 `type`的缩写，所以 `T` 是大多数 Rust 程序员的默认的选择。

当我们在函数体重使用参数的时候，我们必须在签名中声明参数的名称(parameter name)，这样编译器才能知道如何使用参数。类似的，我们也要在使用类型参数的时候在函数的签名中声明变量的名。要定义泛型的 `largest` 函数，就要把类型名称的声明放在函数的名称和参数列表之间的 `尖括号(<>)` 中.

```rust
fn largest<T>(list: &[T]) -> T {
```
这个函数的定义就是：`largest` 函数在类型 `T` 泛型的函数。这个函数的一个参数的名字是 `list`， 类型 `T` 的一个数值的切片， 函数`largest` 会返回和 T类型的最大值。

代码10-5 展示来结合用泛型类型在 `largest` 函数定义的函数签名当中。这个代码给我们展示了如何的使用 `i32` 或者 `char` 的数组切片，注意这个代码有问题，不会被编译通过，后面会进行修改。
```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
代码10-5 用泛型参数的 `largest` 函数，但是有语法错误，无法编译通过

如果你编译上面的代码，那么就会报错：
```shell
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
  = note: `T` might need a bound for `std::cmp::PartialOrd`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
```
这里提及一个对象：`std::cmp::PartialOrd`，这是一种 特性(trait)。在下一节中，我们将会讨论 `特性(trait)`。目前看来，这个错误表示了，不是所有的数据类型都可以被 `largest` 函数体使用的。因为我们需要在函数体里面进行值比较，我们只能用可以用来排序的数据类型。为了可以满足排序的需求，标准库中提供了 `std::cmp::PartialOrd` 特性，可以实现值比较。

### 1.2 结构体的定义(In Struct Definitions)
也可以用泛型参数来定义结构体(struct)，一个或者多个字段都可以是 通用类型(generic type)。
```java
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```
代码 10-6 包含的两个都是T类型的字段 x 和 y

在结构体定义中使用泛型语法和在函数定义中使用泛型语法很相似。首先，需要在结构体之后的尖括号中声明 type 参数的名称。然后，就可以在结构体的定义中原来是用具体的数据类型来声明的位置使用泛型类型。

注意，因为我们只用了一种通用的类型来定义 `Point<T>`这里的定义就是告诉我们`Point<T>` 只有一种 `T` 泛型类型，所以字段 `x` 和 `y` 都是同一种数据类型，不管该类型是什么类型。如果我们创建了一个 `Point<T>` 的实例，且该实例的字段x，y分别是不同的数据类型，那么就会无法编译通过：
```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```
代码10-7 字段x，y必须是同一种数据类型。

在这个例子中，我们把整数值5 分配给变量 `x`，这时候编译知道泛型T的数据类型是整数。然后把字段y指定为4.0，这个时候编译器就会报 `mismatch` 的错误：
```shell
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
```
如果想要定义一个结构体 `Point`，让两个字段 x，y都用泛型，但是拥有不同的数据类型，那么就要用多泛型的参数列表。比如，在代码10-8 中，就改变了`Point` 的定义，这时候字段 `x` 的类型是 `T`， 而字段 `y`的类型是 `U`

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

```
代码 10-8 x，y不同的泛型的 `Point<T,U>`

以上的所有的 `Point` 的实例都是可以被编译通过的。可以根据需求在定义中使用任意多个的泛型类型参数，但是如果泛型太多的话，会让代码难以理解，如果你的代码中需要大量的泛型，那么就表示，你的代码就需要继续少量的重构了。


### 1.3 枚举的定义 (In Enum Definitions)
就像在结构体中定义的那样，也同样可以在 枚举(enum)定义中使用泛型类型。先来看看标准库中挑给你`Option<T>` 的枚举类的代码
```rust
enum Option<T> {
    Some<T>, 
    None,
}
如你所见，`Option<T>` 是一个泛型的枚举类型，并且有两个变量，一个是`Some`，这个具有一个 `T` 类型的值；一个是 `None`，这个变量没有任何值。通过使用`Option<T>` 枚举类，我们可以写出需要可选值的情况，而且因为`Option<T>` 有泛型的特性，所以就可以用这个枚举类型来使用任意类型的。

枚举类型也可以有多类型，比如 `Result<T, E>`
```rust
#![allow(unused_variables)]
fn main() {
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
枚举类`Result`有两种类型，`T` 和 `E`，以及有两个变量：`Ok`，这个包含 `T` 类型的值；`Err` 这个有E类型的值。这个定义让我们在任何地方使用 Result都很方便。
执行成功了会返回`Ok`，失败了会返回 `Err`。这就是我们在代码9-3中使用的逻辑，如果打开文件成功了，那么 `T` 就会被推断出是 `std::fs::File` 类型；如果失败了，那么 `E` 的类型就会被推断为 `std::io::Error`。

### 1.4 方法定义 (In Method Definitions)
我们可以在结构体(struct) 和 枚举类(enum) 中实现方法，也可以用使用的泛型。在代码10-9 展现了如何在结构体 `Point<T>` 实现泛型方法。
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

```
代码 10-9 实现一个名字是 `x` 的方法，在结构体 `Point<T>`，返回一个引用 `T`类型的字段 x

注意，我们这里必须要 `impl` 关键字之后就声明 `T`，这样才可以使用它来指定我们要在 `Point<T>` 类型上实现方法。通过在 `impl` 之后声明泛型类型，Rust可以是识别出 Point 后面对 尖括号的类型是通用类型，而不是具体类型。

比如，我们可以在仅仅在 `Point<f32>` 这个实例实现方法，而不是在 `Point<T>` 对象用泛型类型实现方法。在代码10-10中我们用了具体类型 `f32`，这样在 `impl` 就不用声明任何类型。
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```
代码10-10 用特定的类型得来实现结构体的方法

这里的表示，Point<f32> 类型有个名为 `distance_from_origin` 和 其他的数据类型 `Point<T>` 的对象里是没有这个方法。这个方法是用来计算某个点到 坐标(0.0， 0.0) 的距离。并且仅仅是计算 浮点型(floating) 的情况。

结构体定义中的泛型参数和你的在该结构体方法中的方法签名中的泛型参数不会完全相同。比如在 代码10-11 中，定义了一个泛型方法 `mixup` 在 结构体 `Point<T, U>` 中，这个方法调用了另一个组泛型参数的对象 Point，这个方法创建了一个全新的 `Point<T, w>` 对象得，用新的 `Point<V, W>` 对象和 本身的 `Point<T, U>` 对象。
```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

```
代码 10-11 用不同类型的泛型类型在结构体的定义中。

在 `main` 的我们定义了一个 `Point`对象 p1 ，它的x是`i32`类型，它的y是 `f64`类型。这个p2的也是一个`Point`类型的对象，但是它的x是string切片，它的y是字符类型。`mixup` 调用 `p2` 来得到 同样是 `Point`类型的对象 `p3`。这个对象的x是 `i32` 类型，y是 `char` 类型的。通过宏函数 `println!`可以到看到输出的值。

这个示例仅仅是演示一种情况，一些泛型参数是在 `impl` 关键字后面被定义的，有的则是在方法声明中被定义。这里的泛型参数 `T` 和 `U` 实在 `impl` 后面被定义的，因为它们是和 结构体 一起使用的；而 `v` 和 `w` 是在方法声明中被定义的，因为它们仅仅和方法有关。

### 1.5 使用泛型的代码的性能(Performance of Code Using Generics)
也许你会想知道使用泛型的参数是否会在运行的是产生运行时候的成本。好消息是，Rust 这样的泛型的使用，即使使用泛型，速度也不会变慢。

Rust 通过在编译的时候对使用泛型的代码进行单态化(monomorphization)的处理。*单态话化(Monomorphization)* 是通过在编译时候把具体的类型填充进泛型类型中，将泛型的代码转换为特定的代码的过程。

这个过程中，编译器会执行与创建在代码10-5泛型函数的相反的步骤，编译器会寻找到那些所所有调用泛型的代码的地方，然后为泛型代码的具体类型生成代码。

来看看标准库中的枚举类 `Option<T>`:
```rust
let integer = Some(5);
let float = Some(5.0);
```
Rust 在编译这个代码的时候，它会执行单态化。在这个过程中，编译器会将读取`Option<T>` 类型的被用到实例，然后分别为被用的到的实例创建不同的类型。这里代码中找到是两种 `Option<T>`，然后分别的编译出两种类型。

这个代码 单态化的 效果如下
```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

```
因为Rust会将泛型代码编译成代码中对象的具体类型，所以我们无需再运行的时候付出任何运行成本。代码运行的是，它的性能和我们用手工复制了每个重复代码一样。单态化(monomorphization) 使得rust的泛型运行时候更加高效。

# 2 特性: 共同的行为(Traits: Defining Shared Behavior)
一个特性就会告诉Rust编译器 一个特定的类型有一个特定功能，并且和其他类型共享。可以用特征来定义一种通用的行为。我们可以用 `Traits` 来定义一组有相同的功能的类型。

> 注意：特性(Traits) 和别的语言的 接口(interface) 是有相同的功能的，但是所有一点区别。

### 2.1 定义特性(Defining a Trait)
类型的行为包含了我么可以在改类型上的调用的方法。如果我们可以在不同的类型上定义相同的行为，那么不同的类型就有了相同的行为。特性的定义就是将一种将方法的签名分组归纳在一起以实现某些目的所需要的方式。

有多个结构体(struct)，这种结构体非可以有多个类型变量和多个文本：`NewArticle` 结构体可以持有一种字段，这种字段保存的是新闻的内容。`Tweet` 的这种结构图的字段则保存的一个不超过 280 个字段的字段，并且还要标识出是否为新推文，转发的推文，或者是对另一个推文的回复。

我们想要定义一个媒体的聚合库，这个库可以展示出不同的新闻的数据的摘要。为了完成这个功能，我们需要在每个数据类型(NewArticle、Tweet) 中新增一个摘要方法，用来将从新闻文本，或者从推文中把内容提取出来。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```
↑ 代码 10-12 `Summary` 特性(Trait)，由摘要方法组成的接口

我们用 `trait` 关键字来声明一个 `特征`，并且还声明了这接口的名称 `summary`。在打大括号中，我们声明了这个方法的签名，这些在 特征(trait) 的方法签名决定这特征能够拥有什么方法，这个例子中，这个方法是 `fn summarize(&self) -> String`。

在方法的签名之后，我们用分号来代替了，原来的大括号的方法的实现。每种实现这个特性的类型，都要在这个方法的方法体中增加自己的实现。编译器活强制每个实现了 `Summar` 特性必须要实现 方法 `summarize`。

一个 特征(Trait) 可以有多个方法，具体的格式就是：每行一个方法签名，每行以分号结尾。

### 2.2 在类型中实现特征(Implementing a Trait on a Type)
现在我们已经用了 `Summary` 特征定义了一个共用的方法签名，我们可以在我们的多媒体的聚合库中的类型中实现这些方法了。在 代码 10-13 中的展示了 `Summary` 特性一种的实现。在结构体 `NewsArticle` 中，用字段 headline、 author, location 来创建了一个方法 `summarize` 的返回值。在`Tweet` 结构体中，先假设了tweet的内容I要进限制了280个字符了，然后用 用户名和 内容来作为返回值：
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
↑ 代码 10-13 类型`NewArticle` 和 `Tweet` 实现了特征 `Summary`

在类型上实现特征和实现常规的方法很相似。区别就在于，在 `impl` 关键字之后的，需要加上被实现的 特征(trait)的名称，并且在还要加上 `for` 关键字，然后指定需要实现特征(trait) 的类型的名字。在impl的后面的\
码块中，必须加上trait里面声明的方方法。但是不同的是在原来的特种中的方法声明的后面的分号被代码块所代替。

在实现了这个特征值之后，就可以想调用常规的方法一样。
```rust
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```
输出：`1 new tweet: horse_ebooks: of course, as you probably already know, people`

注意，因为这里我们都在同一个rs的文件中定义了 特征(trait)`Summary` 以及  `NewArticle`类型和`Tweet` 类型都在一个rs的文件中，那么就以为他们在统一作用域。假设想要这个rs文件的内容，当别人想要的这个库的功能的话，或者想要根据自己的需求实现`trait`，那么就要 先引入自己的代码中，通过代码 `use aggregator::Summary`来把这个 特征引入代码中。

需要注意的一点是，类型实现特征(trait) 必须是在我们自己的本地板条箱(crate)中的。比如，我们既可以在类型 `Tweet` 中实现标准库中的 `Display`这个特征(trait)，作为我们的 `aggregator` 的功能。也可以在用`Vect<T>`中来实现 `Summary` ，因为Summary 在我们自己的 `aggregator` 的板条箱中。

但是我们不能在外部的类型(types)中实现所有的外部的特征(trait) (can’t implement external traits on external types)。比如，我们就不能在 `Vec<T>` 中实现 特征 `Display` 在我们的 `aggregator`，这个板条箱中(注意，Summary 这个特性是在我们自己的板条箱中的)。这个限制是程序的一致性，称之为 `一致性(coherence)` 。这个限制是程序的一部分，可以确保别人不会破坏你的代码。如果没有这个规则，尼玛两个板条箱都会实行按一个相同的特征(trait) 在实现同一种类型，这样 Rust 就不知道那个实现是被使用的

```rust

```

### 2.3 默认实现(Default Implementations)
有时候在很多类型里有相同的默认的方法的行为，不需要去在每个类型中都实现一个相同的方法。然后，我们可以再特定的类型上再实现这个特征的时候，可以选择保留这个实现，或者覆盖这个实现。

代码 10-14 展示了如何去定义一个默认的方法实现，而不是仅仅顶一个方法签名
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more ...")
    }
}
```
↑ 代码10-14 一个定义了默认实现的特征

让默认的实现来代替原来的 `NewsArticle`的 `summarize` 的方法实现。`impl` 的代码代码块为空

就算我们接下来都去实现 `NewsArticle` 的 `summarize` 的方法，我们依然还是可用默认的实现。可以在 `NewsArticle` 的对象中调用 `summarize`方法。

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more ...")
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
```
上面这段代码输出 `New article available! Read more ...`。

虽然创建了方法 `summarize` 的默认实现，但是在代码10-13 的 `Tweet` 对象的实现中是不用修改代码，也依然可以得到原来的 tweet 的代码的实现的。其实覆盖原来的特征(trait)的默认的实现和实现默认的方法的语法是一样的。

默认的方法实现是可以调用同一个特征(trait)的其他的方法的，即便那个调用的方法还没有来得及实现。通过这种特性，特征(trait)可以提供很多有用的功能，而且仅仅是通过实现一小部分的代码。比如，我们可以先定义一个非默认方法 `summarize_author`，然后定义一个默认的实现 `summarize` 方法来调用这个`summarize_author`。
```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```
想要使用这个版本的 `Summary`，我们仅仅需要重新实现 `summarize_author` 就行了

```rsut
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```
重新实现了`summarize_author`之后，我们可以用 `Tweet` 结构体调用 `summarize` 方法了，`summarize` 方法会调用 `summarize_author` 方法(当然这个方法是被我们实现过的)。
```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```
上面这点代码输出：`1 new tweet: (Read more from @horse_ebooks...).`

注意，在你覆盖了默认方法实现之后，你不能再调用原来那个默认的方法了，也就是说覆盖的方法和默认的方法你只能留下一个。

### 2.4 参数是特征(Traits as Parameters)
目前为止，我们已经知道了如何去定义，以及实现 特征(trait)，接下来将会探索如何用 特征(trait) 去定义一个方法，以及将不同的参数类型传入方法。

在代码 10-13 中，我们在 `NewsArticle`类型和 `Tweet`类型中实现了 `Summary` 特征。我们可以也可以定义 `notify`函数，用 `Summary` 特征作为他的参数来调用 `summarize` 方法。为了完成这个，就会涉及到一个`impl Trait` 语法，就像下面这个代码：
```rust
pub fn notify(item: &impl Summary) {
    println!("Breakinig news! {}", item.summarize());
}
```
在函数的声明中，我们使用了关键字 `impl`以及特征的名字，而不是直接声明出对象的具体的类型。这样的参数声明是可以接受任何的实现了特征 `Summary`的类型的。在 函数 `notify` 的函数体中，我们可以通过 `item` 对象 来调用所有特征 `Summary` 的特征中有的接口方法，比如 `summarize`。想要调用这个 `notify`，我可以传入像 `NewsArticle` 和 `Tweet` 的对象。如果代码传入了其他类型，比如 `String` 或者 `i32`，会编译不通过的，因为这些类型没有实现 `Summary`。

##### 2.4.1 特征绑定(Trait Bound Syntax)
`impl Trait` 语法看起来使用简单，但是实际上是一个长形式的格式的语法糖，看起来是这样：
```rust
pub fn notify<T: Summary> (item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
这种形式的写法和上面的 `pub fn notify(item: &impl Summary)` 代码是等效的，但是看起来更加的冗长。把特征(trait) 和 泛型的声明绑定在一起，并且放入尖括号中(`<>`) 。

`impl trait` 语法是很方便的，并且在简单的场景下，可以使代码更加的简洁。在其他的更加复杂的场景下，特征绑定语法可以展现出更加更多的复杂性，比如，我们可以有两个实现了 `Summary` 特征的参数比如下面的这样
```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary)
```
如果希望函数允许item1 和 item2 有不同的类型(比如分别传入`NewsArticle` 和 `Tweet`)，是可以用 `impl trait`语法的。如果我们想要两个参数都是相同的类型的话，那么可以用下面的这样的写法：
```rust
pub fn notify<T: Summary> (item1: &T, item2: &T)
```
泛型 `T` 指定了 `item1` 和 `item2`参数的具体的类型，并且进行了类型的约束。这样两个参数是必须是相同的。


##### 2.4.2 用`+`语法来指定多个特征绑定(Specifying Multiple Trait Bounds with the + Syntax)

也可以指定的多个特征和变量绑定起来，假设我们想要 `notify` 方法可以用展示变量`item`的数据，所以我们必须要要在定义中让item实现两个 `Display` 和 `Summary`，可以用 `+` 语法
```rust
pub fn notify(item: &(impl Summary + Display)) {
```
`+` 也可以用在特征绑定的泛型的场景中
```java
pub fn notify<T: Summary + Display> (item: T)
```
随着两个特征比被指定绑定，在notify的函数中就可以调用 `summaize` 和 `{}`来规整他们这个对象的。
##### 2.4.3 用 where 条件语句 进行进一步说明特征绑定 (Clearer Trait Bounds with where Clauses)

用多特征绑定有个缺点。每个泛型都有自己的特征绑定，所以用多个泛型参数作为参数的函数会有很多的特征绑定信息在函数的名称和函数的参数列表之间，这样会导致函数的签名阅读器来很不方便。因此，Rust 提供了备用语法，用于在函数的签名之后去指定特征的绑定信息，比如下面这个代码：
```rust
fn some_function<T: Display + Clone, U: Clone + Debug> (t: &T, u: &U) -> i32
```
用来  `where` 条件语句，就会写成下面这样：
```rust
fn some_function<T, U> (t: &T, u: &U) -> i32
    where T: Display + Clone, 
        U: Clone + Debug 
    {}
```
这样的函数签名读起来就不会很混乱了：函数名称，参数列表，以及返回类型排列在一起，这样读起来就会想没有特征绑定的参数一样。


### 2.5 返回实现了特征的类型(Returning Types that Implement Traits)
也可以在返回的位置用 `impl trait` 来返回实现了某种特征的返回类型的值。
```rust
fn return_summarize() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```
为了通过使用`impl Summary` 来返回类型，我们指定了 `returns_summarizable` h函数的返回类型是实现了 `Summary` 特征的，而不是直接写出类型的名字。在这种场景里，`returns_summarizable` 返回了一个 `Tweet` 类型的对象，但是调用这个函数的代码却不知道里面返回的是一个 `Tweet` 的对象。

返回某个特征的实现的能力在闭包(closures)和迭代器(iterators)的功能中特别有用，这个功能将会在第13中介绍。闭包(closures)和迭代器(iterators)创建的类型只有编译器才知道，或者用长声明的来声明。`impl trait` 语法让你可以简洁的指定实现了 迭代器(Iterator) 类型，而不用非常长的类型声明。

但是，仅当返回单一类型的时候，才可以用 `impl trait` 语法。比如，这个代码返回了 `NewsArticle` 或者 `Tweet` 两种类型，这样的代码会无法运行：
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```
要么返回 `NewsArticle`，要么返回 `Tweet`在使用了 `impl Trait` 语法约束之后是不允许的。我们将会在第17章中“用特征的对象来返回不同类型的值” 来实现有这个功能的函数。

### 2.6 用特征绑定来解决最大的函数的问题 代码10-5 bug (Fixing the largest Function with Trait Bounds)
现在你已经知道了通过特征绑定来指定你想要的函数的行为。让我们在看看 代码10-5 ，本来这个代码是有问题的。上一次我们运行这个代码的时候收到这样一条错误
```shell
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src\main.rs:4:17
  |
4 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest_ele<T: std::cmp::PartialOrd> (list: &[T]) -> T {
  |                 ^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
error: could not compile `listing_10_05`.

To learn more, run the command again with --verbose.
```
在这个 `largest_ele` 函数体中，我们想要比较两个类型 `T` 的值，并且用了大于号(`>`)。因为这个操作符是在标准库特征 `std::cmp::PartialOrd` 定义的，我们需要指出 `PartialOrd`特征类型绑定到`T`类型，这样 `largest` 函数就可以继续类型的比较了。我们是不需要把 `PartialOrd`类型引入代码范围中，因为这个类型是在 `prelude`中，只要更改 `largest` 函数就行：
```rust
fn largest<T: PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

```
运行上面这段代码，就会得到下面这个不同的错误：
```shell
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src\main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
  |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
 --> src\main.rs:3:15
  |
3 |     for &i in list {
  |         --    ^^^^
  |         ||
  |         |data moved here
  |         |move occurs because `i` has type `T`, which does not implement the `Copy` trait
  |         help: consider removing the `&`: `i`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
error: could not compile `listing_10_03_right`.

To learn more, run the command again with --verbose.
```
这个错误的关键的关键行就是 `cannot move out of type [T], a non-copy slice`。在没有带泛型的版本的 `largest` 函数的代码中，我们仅仅能够去找到 `i32` 和 `char` 类型的最大值。就像在第四章中[Stack-Only Data: Copy](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)讨论的那样，想 `i32` 和 `char` 这样的类型，已知道类型的存储的大小，并且存储在栈的空间中，所以它们实现了 `Copy` 特征。但是当我们在 `largest` 函数中增加了泛型功能，在 list 的参数中可能并没有实现 `copy` 特征的类型。所以，我们就不能把值从 `list[0]` 移出来，并且移动进 `largest`变量中，这就是错误的来源。

为了使用必须实现了 `Copy` 特征的类型，我们必须把 `Copy`特征添加到泛型 `T` 的实现的范围中。在代码 10-15 中，我们实现了完整版的代码，这样就可以编译通过了，只要传递给 `largest` 函数的slice的值实现了`Copy` 和 `PartialOrd` 特征，这样就可以使用 `i32` 和 `char`。
```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
↑ 代码10-15 实现了 `PartialOrd` 特征和 `Copy`特征 的泛型类型的 `largest`函数

如果我们不想讲 `largest` 函数限制为 `Copy` 特征的类型，就可以指定 `T` 绑定到特征`Clone`，而不是绑定到特征 `Copy`。这样我们就可以在想要`largest` 函数有所有权的时候，克隆在的 slice 中的每个值了。使用 `clone` 函数，就意味着我们在使用像 `Stirng` 这种类型的时候，就要在堆中开辟更多的空间，如果我们需要处理大量的数据，那么堆的分配就会很慢。

另一种可以实现 `largest` 函数就是从 slice 中返回一个 `T` 类型的值的引用。如果我们吧返回类型更改为 `&T` 而不是 `T`，从而更改了函数的代码体来返回一个引用，我们就不用需要`Clone` 和 `Copy` 的特征绑定了，就可以避免堆的分配和回收了。尝试自己实现这个代码。

### 2.6 用特征绑定来有条件的实现方法。 (Using Trait Bounds to Conditionally Implement Methods)
通过用`impl` 代码块来实现特征绑定，这个方案中有泛型的参数，我们可以为了实现指定的特征，根据条件来实现特征的方法。比如在 代码10-16中，`Pair<T>` 类型永远都实现了 `new` 函数。但是`Pair<T>` 只有在泛型 `T` 同时实现了 `PartialOrd` 特征以及 `Display` 特征的情况下才能实现 `cmp_display` 方法。
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
 ↑ 代码10-16 根据不同的条件来实现泛型中的方法。

对于任何实现了一种特征的类型，我们可以根据不同的条件实现另一种特征。实现一个特征满足特征绑定的的情况称为`blanket implementations`，并且在标准库中已经广泛使用了。比如，标准库中实现了 `ToString` 特征，并且在所有的 `Display` 特征，代码就像下面这样：
```rust
impl<T: Display> ToString for T {
    // --snip--
}
```
因为在标准库中有这种 `blanket Implementation`，所以，我们就可以在任何实现了 `Display` 特征中调用 `ToString`特征才有 `to_string` 方法。比如，我们像下面代码一样，把整数的值转为字符串类型，因为这个额这个整数型的值已经实现了`Display` 特征了。
```rust
let s = 3.to_string();
```
`Blanket implementation` 出现在特征的 文档的 `implements` 章节部分。

特征(Traits) 以及 特征绑定(Trait bound) 让我么可以用泛型参数来减少重复的代码，同时还向编译器指向我么系统通过特征来实现特定的特征行为。编译器可以在根据特征绑定的信息来检查我们提供方法实现是否提供了正确的方法的行为。因为不必在代码的与运行的时候进行类型的检查，所以，所以就可以提升代码的性能了，不必放弃泛型的灵活性了。

另一种泛型已经被使用称之为 `lifetimes`， `lifetimes` 不是确保类型有我们想要的行为，而是确保只要有我们想要引用的时候，那么就是有有效的引用。接下来我们来看看 `lifetimes`是如何做到的。

# 3 用 `LifeTime` 来验证引用的有效(Validating References with Lifetimes) 
在第4章“引用和借”的时候我们讨论过一个细节，每个引用在 Rust中都有一个 *生命期限(lifetime)*，这个就是该引用的有效范围。在多数的时候，生命期限是隐式的(implicit)以及自动推断的(inferred)，就像大多数时候的类型也是自动推断的(inferred)。当代码可能出现多种类型的时候，我们必须要声明数据的类型。同样的，当引用的生命期限有不同的几种联系的时候，那么我们就要声明声明期限(lifetimes)。Rust 需要我们用生命周期参数来注释引用和生命期限的关系。

这里的 声明期限(lifetimes) 的概念和别的语言的lifetime不同，可以说，生命期限就是Rust语言的特色。尽管我们不会在本章中完整的介绍生命期限(lifetimes)概念，但是我们会讨论可能会遇到生命期限的几常见的方式，让你可以更熟悉这个概念。
## 3.1 用生命期来阻止悬挂引用(Preventing Dangling References with Lifetimes)
生命期限的主要功能的就是为了防止悬挂引用的，这些引用使得程序引用不是其想要使用的数据。参考 代码10-17，其中有一个外部域和一个内部域。
```rust
fn main() {
    {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
}
```
代码10-17：尝试使用超出域的引用

> 注意：代码10-17、10-18、10-24中，在声明变量的时候没有赋上初始值，因此变量名称只在外部域中。第一眼看起来，这个似乎和 “Rust不允许有空值” 相矛盾。可是如果我们在它们没有赋值之前尝试使用它们，就会出现编译错误，这就是表示Rust确实不允许空值。

外部域声明了名字为 `r` 的变量，并且没有设置初始值，内部域声明了一个名字为 `x` 的变量，并且设置了初始值为 5.在内部的作用域，我们就尝试把 `x` 值的引用赋值给 `r`。然后这个内部的作用域结束，然后我们就尝试输出 `r`。上面这段代码不能编译，因为这个在是要使用 `r` 之前，`x` 就已经超出范围了，就被释放了。
```shell
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:17
   |
7  |             r = &x;
   |                 ^^ borrowed value does not live long enough
8  |         }
   |         - `x` dropped here while still borrowed
9  | 
10 |         println!("r: {}", r);
   |                           - borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
```
变量的 `x` “活的不够久”。这个原因就是在 `x` 在内部的作用域结束(代码第7行)的时候就已经超出作用域了。但是 `r`在外部作用域的的时候仍然是有效的，因为 `r` 的作用域是更大的，所以我们说它的命更长。如果 Rust 允许这个代码运行，`r` 引用的内存块会在 `x` 超出作用域的时候被回收，所以所有的对于 `r` 的操作都不会有效，那么 Rust 是如何判断代码是否有效的？这个就要使用 借阅查询器(borrow checker)

## 3.2 The Borrow Checker   
Rust编译器的借用查询器(borrow checker)是可以通过比较 作用域(scope) 来确定是否所有的接一共都是有效的。代码10-18 展示了和代码10-17一样的，但是却用注释展示了变量的生命期限：
```rust
fn main() {
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
}
```
代码10-18：注释出 的`r` 和 `x` 的生命期限，称之为 `'a` 和 `'b` 

在这里，我们注释出了 `r` 的声明期限且命名为 `'a`，以及 `x` 的声明期限，命名为 `'b`。就如你所看到的，内部`'b’` 代码块是比起 外部的`'a` 声明期限的代码快要小。在编译的时候，Rust 会比较两个生命期限的长短，这样就发现了一个问题，`r` 的生命期限是 `'a`,但是引用的期限是 `'b`。所以这个程序就被拒绝编译了，因为 `'b` 的生命期限比 `'a`短，因为引用的主题还没有引用活的长(主题都没了，引用还有意义吗？)

代码10-19 解决代码的悬挂引用的问题，所以就可以通过编译了
```rust
fn main() {
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}
```
代码10-19 一个有效的引用，因为数据活的比引用的生命期限还要长。

这里的代码，`x`有生命期限 `'b`, 这里比 `'a`要来的长。这就意味着 `r` 可以引用 `x`了，因为 Rust 知道 `r` 里的引用有效的额时候， `x` 也是有效的。

现在你知道了生命期限是如何进行判断的了，以及 Rust 是如何分析生命期限来保证引用是永远有效的，接下来我们来看看关于函数的参数和返回值的生命期限

## 3.3 函数的生命期限(Generic Lifetimes in Functions)
写一个比返回两个字符串切片(string slices)中更长的切片。这个函数会比较两个字符串切片，然后返回其中更长的一个。之后再来实现`longest`函数，代码10-20 会输出 `The longest string is abcd`
```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```
代码10-20 主函数(main)调用 `longest` 函数来寻找更长的字符串切片。

注意，我们想要这个函数接受字符串切片的引用，因为我们不想要 `longest` 函数获取到参数的所有权(ownership)。有关于为什么我们在代码10-20 中使用的是参数的引用，可以参考第四章 “字符串切片作为参数”
```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
代码10-21 一段无法编译的 `longest` 函数，返回更长的字符串切

我们可以得到下面的错误：
```shell
error[E0106]: missing lifetime specifier
  --> src\main.rs:10:33
   |
10 | fn longest(x: &str, y: &str) -> &str {
   |               ----     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
   |
10 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
   |           ^^^^    ^^^^^^^     ^^^^^^^     ^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0106`.
error: could not compile `listing_10_21`.

To learn more, run the command again with --verbose.
```
这段文本就告诉我们一个，返回类型的需要一个通用的生命期限的参数，因为Rust 不知道返回的引用是 `x` 还是 `y`。实际上，我们也不知道，因为 `if` 的代码块返回的是 `x`，而 `else` 的代码块是返回的 `y`.

定义这个函数的时候，我们不知道将传递给这个函数的具体的值。



## 3.4 生命期限的注释语法(Lifetime Annotation Syntax)



## 3.5 Lifetime Annotations in Function Signatures

## 3.6 Thinking in Terms of Lifetimes


## 3.7 Lifetime Annotations in Struct Definitions


## 3.8 Lifetime Elision



## 3.9 Lifetime Annotations in Method Definitions



## 3.10 The Static Lifetime



# 4 Generic Type Parameters, Trait Bounds, and Lifetimes Together


# 5 Summary















