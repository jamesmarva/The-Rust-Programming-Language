Rust’s standard library includes a number of very useful data structures called collections. Most other data types represent one specific value, but collections can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time. In this chapter, we’ll discuss three collections that are used very often in Rust programs:

A vector allows you to store a variable number of values next to each other.
A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.
To learn about the other kinds of collections provided by the standard library, see the documentation.

We’ll discuss how to create and update vectors, strings, and hash maps, as well as what makes each special.

# 1 Storing Lists of Values with Vectors

### 1.1 创建一个新的 `Vector` (Creating a New Vector)


### 1.2 更新 `Vector` (Updating a Vector )

### 1.3 Dropping a Vector Drops Its Elements


### 1.4 读取 `Vector` 中的元素 (Reading Elements of Vectors)


### 1.5 遍历Vector里的所有元素 (Iterating over the Values in a Vector)

```rust
fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}
```

### 1.6 Using an Enum to Store Multiple Types

At the beginning of this chapter, we said that vectors can only store values that are the same type. This can be inconvenient; there are definitely use cases for needing to store a list of items of different types. Fortunately, the variants of an enum are defined under the same enum type, so when we need to store elements of a different type in a vector, we can define and use an enum!



For example, say we want to get values from a row in a spreadsheet in which some of the columns in the row contain integers, some floating-point numbers, and some strings. We can define an enum whose variants will hold the different value types, and then all the enum variants will be considered the same type: that of the enum. Then we can create a vector that holds that enum and so, ultimately, holds different types. We’ve demonstrated this in Listing 8-10.

fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
↑ Listing 8-10: Defining an enum to store values of different types in one vector

Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. A secondary advantage is that we can be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector. Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled, as discussed in Chapter 6.

如果Rust允许一个Vector 里面可以保存任意类型，那么在类执行一些特定操作的时候就会出现错误。比如有些结构体定义了一些方法，别的没有，那们就会出现异常。使用枚举，再加上 `match` 表达式，就意味着Rust将在编译的时候能确保处理所有的情况，就像第 6 章那样

到目前为止，我们已经讨论了一写使用 `Vector` 的最常用的方法，如果你想了解所有 Vec 上所定义的所有有用的API，那么你就要去看API文档。还有些很有用的操作，除了 `push` 之外， ` pop` 方法来可以返回并且删除最后一个元素。接下来继续下一个集合类型：字符串(String)


# 2 使用 `String` 来保存 `UTF-8` 编码格式的文本( Storing UTF-8 Encoded Text with Strings)

We talked about strings in Chapter 4, but we’ll look at them in more depth now. New Rustaceans commonly get stuck on strings for a combination of three reasons: Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8. These factors combine in a way that can seem difficult when you’re coming from other programming languages.

It’s useful to discuss strings in the context of collections because strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text. In this section, we’ll talk about the operations on String that every collection type has, such as creating, updating, and reading. We’ll also discuss the ways in which String is different from the other collections, namely how indexing into a String is complicated by the differences between how people and computers interpret String data.


### 2.1 What Is a String?












