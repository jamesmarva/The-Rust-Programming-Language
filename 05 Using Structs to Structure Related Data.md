结构体，或者称为结构，是一种自定义的数据类型让你可以自己命名多个相关的值以及将他们打包在一起，以及组成一个有意义的数据组。
如果你熟悉面相对象语言的话，那么结构体就像对象的数据属性一样。在这个章节里，我们将会比较元组和结构体，以及讨论如何定义方法以及将一个结构体和一个函数的行为绑定起来。
结构题和枚举是创建模块为了创造新的类型在你的程度的整型中，对于Rust的编译时候的类型确认给予了非常大的帮助。

# 1 定义以及初始化结构体 Defining and Instantiating Structs


### Using the Field Init Shorthand when Variables and Fields Have the Same Name


### Creating Instances From Other Instances With Struct Update Syntax

### 使用元祖结构体创建不同类型的数据在无命名字段情况下 Using Tuple Structs without Named Fields to Create Different Types


### Unit-Like Structs Without Any Fields

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type. Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself. We’ll discuss traits in Chapter 10.
你也可以顶一个一个没有任何字段的结构体，这种结构体被称为 *类单元结构体(unit-like structs)*，因为他们的行为和 `()`，一个中单元类型。类单元结构体对于那些需要在某种类型上实现某种特征但是又不想这个类型本身存储任何数据的情境下非常有用。我们将在第10章中详细讨论

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

### 3.1 定义方法 Defining Methods
让我们来改变 `area` 函数的实现，本来这个函数是用一个 `Rectangle` 实例来作为参数的，现在我们来用一个带着 `area` 方法的结构体来代替这种写法, 代码在 代码清单5-13:
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
To define the function within the context of Rectangle, we start an impl (implementation) block. Then we move the area function within the impl curly brackets and change the first (and in this case, only) parameter to be self in the signature and everywhere within the body. In main, where we called the area function and passed rect1 as an argument, we can instead use method syntax to call the area method on our Rectangle instance. The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.
为了在 `Rectangle` 的上下文中定义一个函数，我们先顶一个 `impl` 的代码块。然后我们把 `area` 函数搬到 `impl` 关键字后面的大括号中，并且修改函数的签名中的第一个参数为 `self`。
In the signature for area, we use &self instead of rectangle: &Rectangle because Rust knows the type of self is Rectangle due to this method’s being inside the impl Rectangle context. Note that we still need to use the & before self, just as we did in &Rectangle. Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.
在这个方法的签名中，我们用 &self 来代替 参数 `rectangle: &Rectangle`
We’ve chosen &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter. Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.

The main benefit of using methods instead of functions, in addition to using method syntax and not having to repeat the type of self in every method’s signature, is for organization. We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.




The main benefit of using methods instead of functions, in addition to using method syntax and not having to repeat the type of self in every method’s signature, is for organization. We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.


### 3.2 Methods with More Parameters
### 3.3 Associated Functions
### 3.4 Multiple `impl` Blocks
### 3.5 Summary

















