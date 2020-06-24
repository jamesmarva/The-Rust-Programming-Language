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


### 3.1 
### 
### 
### 

















