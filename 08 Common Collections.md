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
最后一个日常会用的数据结构是 HashMap，HashMap 存储了键(key) 和 值(value)的映射关系的映射关系，通过一个哈希函数，它确定了要把键与值存在内存哪个位置。大多数的语言都支持这种数据结构，只不过他们有不同的名字罢了，比如，hash、map、Object、hash table、dictionary和associative array。这些代表都是相同的数据结构，只不过有着不同的名字罢了。
当你想不依赖索引，而是用某个数据类型的对象来查找数据的时候，HashMap 会非常有用。比如在某个游戏中，你可以用队伍来作为键(key)，然后用的这个队伍的得分来作为值(value)，这样你就利用Hash Map 根据队伍的名称来直接队伍的得分了。
在本节中，我们将会介绍HashMap的基本的API，但是在
### 3.1 Creating a New Hash Map
你可以用关键字 `new` 创建一个空的HashMap，用的insert方法来插入数据，在代码 8-20 中的，我们会跟踪两个名字分别为 blue 和 yellow 的队伍的得分。
```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```
↑ 代码 8-20: 创建一个Hash Map，然后插入一些数据

注意，这里我们是用的标准库里的HashMap这个数据结构。在我们三个常见的集合中的，这个集合是我们最不常见的集合。所以这个就不会自动的被引入作用域中。HashMap也没有得到标准库的支持。比如，没有内置的标准洪来支持他们。

就像Vector一样，HashMap是把数据存在堆中的。上面这个例子的中，HashMap的键是String类型的，值是i32类型的。就像Vector一样，HashMap有同质的特性，也就是说，所有的key 要是同一种类型的，所有的value也要是同一种类型的。

Another way of constructing a hash map is by using iterators and the collect method on a vector of tuples, where each tuple consists of a key and its value. We’ll be going into more detail about iterators and their associated methods in the ”Processing a Series of Items with Iterators” section of Chapter 13. The collect method gathers data into a number of collection types, including HashMap. For example, if we had the team names and initial scores in two separate vectors, we could use the zip method to create a vector of tuples where “Blue” is paired with 10, and so forth. Then we could use the collect method to turn that vector of tuples into a hash map, as shown in Listing 8-21.
另一种构造HashMpa的方法就是在一个元祖构造的数组上用迭代器，还有 `collect` 方法。每个元组都是一个映射关系。我们将在第13涨的 “使用迭代器来处理项目” 中介绍迭代器以及其的使用方法。collect 方法可以把数据收到多种的集合的数据类型中，当然也包括HashMap。比如，我们在有个两个队伍的名称，以及两队伍的得分。，用 `zip` 方法创建一个元组的 Vector 。然后使用collect 方法来把元组的 vector 转化为 HashMap。

```rust
fn main() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}
```
↑ 代码 8-21: 两个数组来创建HashMap

类型注解HashMap<_, _> 这里是必须的，因为这里的 collect 方法会收集到很多的数据，但是rust是不知道你想要哪个数据类型的，除非你自己指定。但是对于key和value的类型的参数，我们用下划线，rust可以根据向量中的数据的额类型推断出HashMap中的数据类型。
### 3.2 HashMap 和 所有权(Hash Maps and Ownership)
如果类型有复制的特性的话，就像 `i32` 这种的数据类型，这些值会被复制HashMap中。有所有权的特性的数据类型，HashMap就会成为他们的所有者。
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
↑ 代码 8-22: 一旦插入就拥有所有权的HashMap
当调用了 `insert` 的方法之后，我们不能再使用 已经成为 key(`field_name`) 和 value(`field_value`) 的变量了。

如果我们是把引用插入道德HashMap中，那么值就不会被搬家到HashMap中。引用的值至少要保证在HashMap的有效期的期间是有效的(也就是不能被drop和move？)。在第10章，将会讨论关于生命周期验证参考的内容。
### 3.3 获取在HashMap中值 (Accessing Values in a Hash Map)
可以用key来获取相应的HashMap中的值。

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
↑ 代码 8-23 通过队伍的名来获取队伍的分数
这里的 分数(score) 和 蓝队是分数，而且这里的值一定是`Some(&10)`，这个结果是被`Some`这个结构体包装的，因为get方法会返回一个 `Option<&V>` 类型的对象；如果key其实对应的value在HashMap中并不存在，那么调用get方法之后就会返回 None。这个程序需要被 `Option` 这个结构体处理一下。
就像在 `Vector` 的使用一样，可以用迭代的方法来遍历HashMap中的所有的键值对( key/value pair)

### 3.4 更新HashMap (Updating a Hash Map)

Although the number of keys and values is growable, each key can only have one value associated with it at a time. When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned. You could replace the old value with the new value, completely disregarding the old value. You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value. Or you could combine the old value and the new value. Let’s look at how to do each of these!
经过key 和 value 的数量是不断增长的，每个key只能对应一个value。当你想要的更新HashMap的值的时候，那么你就要根据现有的值进行相应的更新了。
1 用新值替换旧值
2 保留旧值，放弃新值。
3 忽略新值，仅当键(key)不存在的时候才进行插入
4 结合新值和旧值。

##### 3.4.1 覆写值(Overwriting a Value)
如果我们将一个键(key) 和 一个值(value) 插入到HashMap中，那然后用相同的键(key) 插入一个不同的值(value)，那么与这个键相关联的值就会被替换。就像代码8-24中那样
```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}
```
↑ 代码 8-24: 用一个相同的key替换已经存在的值。

##### 3.4.2 当key没有value的时候才尽心插入(Only Inserting a Value If the Key Has No Value) 
通常，用HashMap的是，要先判断一个键是否已经存在了。HashMap对于这个操作有特别的API，在调用的 `entry` 的时候，你可以把想要的检查的键(key)作为参数传递给 `entry` 方法，entry 方法的返回值是一个名称是 Entry 的结构体，它表示了一个也许存在，也许不存在值的Entry。我们要先检查是否 `Yellow` 队伍是否存在，如果不存在，再继续插入值：
```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
```
↑ 代码 8-25: 用 entry 方法，只有当key不存在的是，才进行插入




##### 3.4.3 Only Inserting a Value If the Key Has No Value


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








