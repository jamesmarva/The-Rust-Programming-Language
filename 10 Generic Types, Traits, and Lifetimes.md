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


# 1 Generic Data Types

用泛型来创建函数签名或者结构体，这样我们就可以用多种不同的具体数据类型了。先来看看如何用泛型来定义函数，结构体，枚举类型，以及方法。然后在讨论泛型是如何影响代码的性能的。


### 1.1 函数定义(In Function Definitions)

When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value. Doing so makes our code more flexible and provides more functionality to callers of our function while preventing code duplication.

Continuing with our largest function, Listing 10-4 shows two functions that both find the largest value in a slice.


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