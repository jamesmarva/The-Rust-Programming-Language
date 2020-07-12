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

```rust
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
```
↑ Listing 8-10: Defining an enum to store values of different types in one vector

Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. A secondary advantage is that we can be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector. Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled, as discussed in Chapter 6.

如果Rust允许一个Vector 里面可以保存任意类型，那么在类执行一些特定操作的时候就会出现错误。比如有些结构体定义了一些方法，别的没有，那们就会出现异常。使用枚举，再加上 `match` 表达式，就意味着Rust将在编译的时候能确保处理所有的情况，就像第 6 章那样

到目前为止，我们已经讨论了一写使用 `Vector` 的最常用的方法，如果你想了解所有 Vec 上所定义的所有有用的API，那么你就要去看API文档。还有些很有用的操作，除了 `push` 之外， ` pop` 方法来可以返回并且删除最后一个元素。接下来继续下一个集合类型：字符串(String)


# 2 使用 `String` 来保存 `UTF-8` 编码格式的文本( Storing UTF-8 Encoded Text with Strings)

We talked about strings in Chapter 4, but we’ll look at them in more depth now. New Rustaceans commonly get stuck on strings for a combination of three reasons: Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8. These factors combine in a way that can seem difficult when you’re coming from other programming languages.

把字符放在集合的这个内容来讲是比较容易让人理解的，因为字符串以一个字节的集合来实现的，而且在这些字节被解释为文本的时候提供了很多有用方法。在这一小节，我们将会谈论到的在 `String` 中的在别的集合中都有的操作，比如创建，读取，更新。也会讨论到这些操作和别的集合的不同点，因为人与计算机对 `String` 的不同的理解导致了在字符串中使用索引变得无比复杂。

### 2.1 字符串是什么？What Is a String?
In Chapter 4, we talked about string slices, which are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices.
我们首先定义术语 *string* 的含义。在Rust核心的语法中，只有一种 `string`(译者注：：注意，这里的s是小写的) 类型，就是 `str` 切片，通常是以借用的形式看到的`&str`。在第四章中，我们讨论了*字符串切片(string slices)*，这是对一些 UTF-8 编码格式的字符串的引用。比如的， 字符文件字存储在存储在程序的二进制文件中，因此这里的就是字符串切片(string slice)。
`String` 是有Rust的标准库(Rust’s standard library)提供的，注意上的是核心库( core language)，他是一个可以边长，可修改的，可以被所有的，并且是 UTF-8 编码格式的字符串类型。当 Rustaceans 在Rust 中提到 “strings” 的时候，通常指的就是这 `String` 类型 和 字符串切片`&str`类型，而不仅仅是这两种类型之一。经过本小节是关于`String` 的，但是在 rust  的标准库中都大量使用到了两种类型，并且 `String` 和字符串切片(strnig slice)  都是 UTF-8 编码格式的。
Rust’s standard library also includes a number of other string types, such as OsString, OsStr, CString, and CStr. Library crates can provide even more options for storing string data. See how those names all end in String or Str? They refer to owned and borrowed variants, just like the String and str types you’ve seen previously. These string types can store text in different encodings or be represented in memory in a different way, for example. We won’t discuss these other string types in this chapter; see their API documentation for more about how to use them and when each is appropriate.

### 2.1 创建一个 `String` Creating a New String
在 `Vect<T>` 和 `String` 中有很多相同的操作方法，我们先用`new` 函数来创建一个字符串：
```rust
fn main() {
    let mut s = String::new();
}
```
↑ 代码 8-11: 创建一个空字符
这行代码创建了一个名为 `s` 新的空字符串，然后我们可以把我们的数据加到其中。通常我们都会用一些初始的数据作为字符串的开头。如果你想查看某些对象中的数据，可以使用使用`to_stirng`来查看这个类型上的数据，这个方法可以用在任意的类型上的。
```rust
fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}
```
↑ 代码 8-12: 利用 `to_string` 来根据文本来创建一个字符串
上面这个代码创建了一个包含“initial contents” 的字符串。

我们也可以用 ``String::from 来根据一个文本字符串来创建一个字符串(String)，代码 8-13 和 代码 8-12 达到效果是一样的。
```rust
fn main() {
    let s = String::from("initial contents");
}
```
↑ Listing 8-13: Using the String::from function to create a String from a string literal

```rust
fn main() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
```
↑ Listing 8-14: Storing greetings in different languages in strings


### 2.2 更新一个字符串(Updating a String)
字符串的长度是可以边长的，同时，这个字符串的内容也是可变的，就像 `Vec<T>` 一样。另外，你也可以用 `+` 或者的宏函数 `format!` 来进行字符串的串联。
##### 2.2.1 用 `push_str` 和 `push` 来进行字符串的增加
通过用 `push_str` 方法来把字符串切片来增加到字符的末尾。
```rust
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
}
```
↑ Listing 8-15: Appending a string slice to a String using the push_str method

变量 `s` 的字符串就变成了 `foobar`。`push_str`方法会使用字符串切片，这种情况比较适用于，我们只想要这个数据的值，不用非的获取参数的所有权的情况。比如，如果在代码 8-16 中，如果把 `s2` 被添加到 `s1` 就无法使用就是很坑的。
```rust
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}
```
↑ 代码 8-16: 把 `s2` 的内容添加到 `s1` 之后

`push` 把单个字符增加到字符串(String)的后面。
```rust
fn main() {
    let mut s = String::from("lo");
    s.push('l');
}
```
↑ 代码 8-17: 用push 字符串的后面增加一个字母

上面的这个字符串 `s` 的结果就是 `lol`

##### 2.2.2 用 `+` 或者 宏函数 `format` 把字符串串联起来
```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}
```
↑ Listing 8-18: Using the + operator to combine two String values into a new String value



### 2.3  字符串中的索引(Indexing into Strings)

### 2.4 Slicing Strings
### 2.5 Methods for Iterating Over Strings

### 2.6 Methods for Iterating Over Strings

### 2.7 Strings Are Not So Simple


# 3 Storing Keys with Associated Values in Hash Maps

### 3.1 Creating a New Hash Map

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```
Listing 8-20: Creating a new hash map and inserting some keys and values


```rust
fn main() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}
```
Listing 8-21: Creating a hash map from a list of teams and a list of scores





### 3.2 Hash Maps and Ownership


```rust
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

```

### 3.3 Accessing Values in a Hash Map

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
}

```

### 3.4 Updating a Hash Map

Although the number of keys and values is growable, each key can only have one value associated with it at a time. When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned. You could replace the old value with the new value, completely disregarding the old value. You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value. Or you could combine the old value and the new value. Let’s look at how to do each of these!



### 3.5 Only Inserting a Value If the Key Has No Value


### 3.6 Updating a Value Based on the Old Value
```rust
fn main() {
    use std::collections::HashMap;
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

```
Listing 8-26: Counting occurrences of words using a hash map that stores words and counts

This code will print {"world": 2, "hello": 1, "wonderful": 1}. The or_insert method actually returns a mutable reference (&mut V) to the value for this key. Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

we must first dereference count using the asterisk (*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.




### 3.6 Hashing Functions
By default, HashMap uses a “cryptographically strong”1 hashing function that can provide resistance to Denial of Service (DoS) attacks. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait. We’ll talk about traits and how to implement them in Chapter 10. You don’t necessarily have to implement your own hasher from scratch; crates.io has libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.

1 [https://www.131002.net/siphash/siphash.pdf](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)






### 4 Summary








