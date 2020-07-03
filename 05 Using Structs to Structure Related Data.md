结构体，或者称为结构，是一种自定义的数据类型让你可以自己命名多个相关的值以及将他们打包在一起，以及组成一个有意义的数据组。
如果你熟悉面相对象语言的话，那么结构体就像对象的数据属性一样。在这个章节里，我们将会比较元组和结构体，以及讨论如何定义方法以及将一个结构体和一个函数的行为绑定起来。
结构题和枚举是创建模块为了创造新的类型在你的程度的整型中，对于Rust的编译时候的类型确认给予了非常大的帮助。

# 1 定义以及初始化结构体 Defining and Instantiating Structs


### Using the Field Init Shorthand when Variables and Fields Have the Same Name


### Creating Instances From Other Instances With Struct Update Syntax

### 使用元祖结构体创建不同类型的数据在无命名字段情况下 Using Tuple Structs without Named Fields to Create Different Types


### Unit-Like Structs Without Any Fields

你也可以定义一个没有任何字段的结构体，这种结构体被称为 *类单元结构体(unit-like structs)*，因为他们的行为和 `()`，一个中单元类型。类单元结构体对于那些需要在某种类型上实现某种特征但是又不想这个类型本身存储任何数据的情境下非常有用。我们将在第10章中详细讨论

> ### 类结构数据的所有权 Ownership of Struct Data
> 
> 

# 2 一个关于使用结构体的样例 An Example Program Using Structs

It doesn’t matter if we mix up width and height for the area calculation, but if we want to draw the rectangle on the screen, it would matter! We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1. If someone else worked on this code, they would have to figure this out and keep it in mind as well. It would be easy to forget or mix up these values and cause errors, because we haven’t conveyed the meaning of our data in our code.

### 2.1 Refactoring with Tuples

### 2.2 Refactoring with Structs: Adding More Meaning

### 2.3 Adding Useful Functionality with Derived Traits


# 3 方法语法 Method Syntax
方法(`method`)的语法和 函数(`function`)的语法很像：声明它们的时候用的是相同的关键字 `fn`以及其的名称，他们都可以有 参数和返回值，并且他们都包含代码而且会被别的位置代码所调用。但是，方法()和函数的不同之处就在于，方法是在结构体`struct`(或者一个枚举，或者一个特征对象，我们会在第六章和第17张中介绍)的上下文的中被定义的，以及他们的第一个参数永远都是 `self`，它表示拥有这方法的这个结构体`struct`的实例。

### 3.1 定义方法 (Defining Methods)
让我们来改变 `area` 函数的实现，本来这个函数是用一个 `Rectangle` 实例来作为参数的，现在我们来用一个带着 `area` 方法的结构体来代替这种写法, 代码清单5-13:
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
↑ 代码清单 5-13: 在结构体 `Rectangle` 中定义 `area` 方法

为了在 `Rectangle` 的上下文中定义一个函数，我们先定义一个 `impl` 的代码块。然后我们把 `area` 函数的代码搬到 `impl` 关键字后面的大括号中，并且修改函数的签名中的第一个参数为 `self`。在 `main` 函数中，我们调用了 `area` 函数，并且把 `rect1` 作为参数，可以用方法语法来代替函数使用，用`Rectangle` 实例来调用 `area` 方法。方法是在一个实例之后：我们在方法名称，括号和所有的参数后面添加一个点。

在这个方法的签名中，我们用 `&self` 来代替 参数 `rectangle: &Rectangle`，因为根据该方法在 `impl Rectangle` 的上下文中，所以Rust知道 `self` 的数据类型就是 `Rectangle`。要注意，我们仍然需要在 `self` 前面增加一个符号 `&`，就像我们之前的 `&Rectangle` 那种做法一样。方法可以取得 `self` 的所有权，借用不可变的 `self` ，或者借用可变的 `self`，就像我们使用别的参数那样。很少有方法只使用 `self` 作为第一参数，并且获取它的所有权的；这种做法通常是方法为了吧 `self` 转换为其他形式，并且你想要防止调用者在转换之后重复使用这这个原始的对象(这段不通顺)。

在这个方法的签名中，我们用 &self 来代替 参数 `rectangle: &Rectangle`，因为根据该方法在 `impl Rectangle` 的上下文中，所以Rust知道 `self` 的数据类型就是 `Rectangle`。要注意，我们仍然需要在 `self` 前面增加一个符号 `&`，就像我们之前的 `&Rectangle` 那种做法一样。方法可以取得 `self` 的所有权，借用不可变的 `self` ，或者借用可变的 `self`，就像我们使用别的参数那样。很少有方法只使用 `self` 作为第一参数，并且获取它的所有权的；这种做法通常是方法为了吧 `self` 转换为其他形式，并且你想要防止调用者在转换之后重复使用这这个原始的对象(这段不通顺)。

除了使用方法的时候不用在每个方法签名中重复声明 `self` 的数据类型之外，用方法来代替函数的最主要的好处对是代码的整体的结构而言。我们将把这类的所有的功能代码都放在同一个 `impl` 代码块中，这样开发者就不同去别我们提供的库中的各个位置搜索 `Rectangle`的代码了
> `->` 去哪了？ (Where’s the `->` Operator?)
> 在 `C` 和 `C++` 中，有两种不同的操作符来表示调用方法：
> - 1. 如果你想直接通过一个对象来调用方法的话，那么你就会用到 `.`
> - 2. 如果你想通过指针来调用方法，那么你就会用到 `->`，同时你还需要先取消引用该指针(完了关于C的知识全忘了)
>
> 换句话说，如果 `object` 是一个指针，那么 `object->something()` 和 `(*object).something` 是相同的。
> Rust 没有 `->` 这种操作符，同时，Rust有一个称之为自动引用和自动取消引用的语言特性。调用方法是Rust少数的使用这种特性的情形之一。
> 它的这样运行的: 当你使用 `object.something()`，Rust会自动添加 `&`，`&mut`或者 `*`从而使对象与方法签名匹配。换句话说，下面的两种写法功能都是相同的：
> ```rust
  p1.distance(&p2);
  (&p1).distance(&p2);
> ```


### 3.2 Methods with More Parameters
```rust
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```
↑ Listing 5-14: Using the as-yet-unwritten can_hold method

### 3.3 Associated Functions
### 3.4 Multiple `impl` Blocks
### 3.5 Summary

