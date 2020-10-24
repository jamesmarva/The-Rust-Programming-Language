Rust从很多现存的语言中吸取了灵感。其中一个比较明显的影响就是 *函数式编程(functional programming)*。函数式编程用函数作为参数，以及作为其他函数的返回值使用，然后把函数赋值给变量进行使用。

这章我们不讨论函数式编程是什么的问题，而是展示 `Rust` 一些功能上被其他的认为是函数式类似的特性。

接下来我们将会涉及：
1. 闭包。可以存储在变量里的类函数的结构
2. 迭代器，处理元素序列的方式
3. 如何用上面的两个特性进行改进 I/O 项目
4. 两个特性的性能。

Rust还有其他特性，比如，matching 模式以及枚举，这个我们在别的章节已经涉及了。精通闭包和迭代器是编写高性能的函数式编程的关键，我们将花费一整章来介绍他们。
# 1 闭包：可以捕获环境参数的匿名函数(Closures: Anonymous Functions that Can Capture Their Environment)
Rust的闭包是可以被保存进一个变量，以及传递给另一个函数的匿名函数。你可以在创建闭包之后，在下文中使用闭包计算。和函数不同的是，闭包是允许捕获调用者作用域中的参数的，我们将会展示如何定义闭包以及如何使用这些功能。
## 1.1 创建闭包，以及定义一个它的抽象行为(Creating an Abstraction of Behavior with Closures)
让我们来看一个稍后要执行的闭包的示例。在这个过程中，我们会涉及闭包的语法，类型推断和 `trait`

先来考虑一个假设的情况：我们在一个专门为客户定制健身计划的APP的初创企业工作。后端是用Rust编写的，而生成健身计划的算法需要考虑多个因素：用户的年龄，身体的素质指数，运动偏好，最近的锻炼的记录，以及用户指定的强度参数。在这个例子中这个算法不重要，重要的是这个算法的*计算时间*。我们希望在需要的时候调用算法，并且希望只调用一次，不用让用户等太久。

我们会模拟这种假设函数来假设这种算法，`simulated_expensive_calculation`主要进行的是等待2秒人，然后继续打印`calculating slowly...`
```rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {}
```
代码13-1 一个用来代替计算的假设函数，时间时间2秒

在main的函数包含着app中很重要的代码。这个就代表用户请求健身计划的时候，这个代码就会被调用。因为前端开发和闭包的使用没什么关系，所以这里就没有先显示前端的代码，用硬编码的程序的输入输出来进行代替输入。

必要的输入是一下：
- 来自用户的参数 `instensity`，用户请求的时候指定的参数，用来指示用选择的高强度的计划还是低强度的计划。
- 一个随机数，用来改变计划中的一些情况。

输出是个锻炼计划，代码12-2展示了 我们将会使用功能的代码。
```
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
```
代码13-2 `main` 函数用硬编码来模拟用户的输入和随机数

为了简单点，我们把变量`simulated_user_specified_value `和变量 `simulated_random_number`硬编码为 `10` 和 `7`，在实际情况中，我们会从app端获取用户的强度的参数，然后用 `rand` crate 来生成随机数，就像第二章的生成随机数那样。

现在有整体的代码环境了，让我们来继续生成计划的代码。这个代码13-3 就包含了现在最重要的生成计划的代码了。
```rust
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```
代码13-3 程序的业务逻辑，根据输入的 `simulated_expensive_calculation` 来生成健身计划的业务逻辑代码


### 1.1.1 用函数来重构代码(Refactoring Using Functions)
```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}
```
代码13-4 用 `simulated_expensive_calculation` 调换一个位置，并且把结果存在变量中。

这个修改统一了所有的调用，都把这些耗时的调用封装到了 `simulated_expensive_calculation` 函数中了，这样就解决了在第一个 `if` 的代码块中的必须要要调用两次的问题。但是不幸的是，这样的解决方式会让不必要条件下依然触发调用函数，比如在第二个的 `if` 的代码块中是没有必要调用函数的。

我们想要在我们的程序某处中定义我们的代码，但是仅仅在需要的时候调用。这就是闭包的应用场景之一了。

### 1.1.2 重构的时候使用闭包储存代码 (Refactoring with Closures to Store Code)


```rust
let expensive_closure = |num| {
    println!("calculating slowly.");
    thread::sleep(Duration::from_secs(1));
    num
};
```
代码13-5 定义一个闭包函数，并且赋值到 `expensive_closure` 变量

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly.");
        thread::sleep(Duration::from_secs(1));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            )
        }
    }
}
```
代码13-6 调用定义的 `expensive_closure` 

现在这个耗时的计算仅仅出现在一个地方，

但是我们又出现了代码13-3 的问题，就是在一个 `if` 代码块里调用了两次闭包，这两次的调用让用户多等了一倍的时间。可以在 `if` 的代码块里创建一个变量来存储这个计算结果，但是闭包还是可以有另一个解决方案。稍后的我们将会讨论这个解决方案。不过先让我们来讨论为什么在闭包的定义和所涉及的 `trait` 里面没有类型声明。
## 1.2 闭包的类型推断和注释(Closure Type Inference and Annotation)
闭包不要求在参数和返回值钱生命类型，不要像函数那么干。函数要求类型声明是因为函数是暴露给别的用户使用的。严格的定义对于别的用户来说，可以确保每个人都同意函数如何使用，返回值是什么。但是闭包不是这总用于暴露给别的用户的接口：它们是被存储在变量中的，不用给他们命名，或者暴露给使用我们库的用户。

闭包通常短，而且只在很小范围的代码环境中使用。在这些有限制的上下文中，编译器可以可靠地的推断出参数还有范围值的类型，和Rust是如何推断出大部分变量的类型一样。

如果强制让程序员在这些匿名函数中进行类型和返回值的类型声明会很烦人，并且会在很大程度上浪费了编译器已经可用的信息了。

和某些变量类似，如果你想要增加明确性和代码的清晰度，则可以加上类型声明，只不过这样会增加代码的冗余度。代码13-5 如果加上类型声明就会像代码13-7这样：
```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
}
```
代码13-7 增加了类型和返回值声明的闭包

有了类型声明的闭包的语法和函数就更像了。下面是对函数和闭包的语法对比。下面展示了闭包语法是如何类似于函数语法的，除了竖线不能换成括号的情况之外：
```rust
fn add_one_v1 (x: u32) -> u32 {x + 1}

let add_one_v2 = |x: u32| -> u32 {x + 1};
let add_one_v2 = |x| {x + 1};
let add_one_v3 = |x| x + 1;
```
第一行是个函数的定义，第二行就是完整的有类型声明的闭包的定义。第三行则是闭包省略了类型声明的定义，第四行是在去掉了大括号的定义，因为整个闭包体只有一行代码。后三行都是闭包的定义，并且都有相同的行为。

闭包定义将为每个参数和返回值推断出具体的类型。比如代码13-8 的这个简短的定义。这个闭包仅仅是作为返回传递给它的参数的值。实际的情况中，这个闭包其实没什么用，仅仅是做为一个例子。注意，这里的闭包没有增加类型声明，如果我们调用两次这个闭包，第一次是将 `String` 类型作为参数的，第二次是将 `u32` 作为参数的，就会得到一个错误。
```
let example_closure = |x| x;
let s = example_closure(String::from("hell")):
let n = example_closure(5);
```
代码13-8 尝试调用一个参数被推断为两个不同类型的闭包。

编译器会给我们一个错误信息：
```
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |                             ^
  |                             |
  |                             expected struct `std::string::String`, found integer
  |                             help: try using a conversion method: `5.to_string()`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example`.

To learn more, run the command again with --verbose.
```
第一次的调用，编译器会推断闭包的参数和返回值的类型都是`String`。这些类型会被锁定到闭包中，如果再次调用的时候用了不同的类型，就会出错。
## 1.3 用一个有泛型参数和 `Fn` trait将闭包保存起来(Storing Closures Using Generic Parameters and the Fn Traits)
回到我们的生成健身计划的APP，在代码13-6里，我们的代码依然调用了昂贵的计算的闭包。解决这个问题的方法就是在多次的闭包的地方用一个变量把计算的结果都记录下来，但是这样导致的问题就是会有很多个重复保存变量的地方。

幸运的是，仍然还有一个可用方案可以用。我们可以创建一个结构体来保存闭包，然后缓存了调用了闭包的值，这样接下里的代码就不用保存这些结果到变量中了。你也许已经知道了这个模式了，就是*memoization* 或者 *lazy evaluation（惰性求值）* 

```rust
struct Cacher<T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
```
代码13-9 定义一个 `Cacher` 结构体来存放闭包到 `calcalution` 变量中，保存 `Option<T>` 到变量 value 中。

## 1.4 利用Cacher的限制(Limitations of the Cacher Implementation)
值的缓存是个使用很广的实用行为，我们也许希望在代码中的别的闭包也使用他们。可是，目前 `Cacher` 的实现仍然存在两个小问题，这就使得在不同的上下文中复用变得很困难。

第一个问题是，`Chcher` 的实例对于 `value` 放来的任何参数都是返回相同的值，也就说下面的这个测试例子会失败：
```rust
fn main() {
    let mut c = Cacher::new(|a| a);
     
     let v1 = c.value(1);
     let v2 = c.value(2);

     assert_eq!(v2, 2);
}
```
这个测试用返回传递给它的参数作为返回值的闭包来创建了一个新的 `Cacher`。用 1 和 2 分别调用 `Cacher` 实例的 `value` 方法，我们希望使用 2 的时候，value会返回一个 2。

使用代码13-9 和 代码13-10 的 `Cacher` 实现进行测试，它会在调用 `asser_eq!` 的时候失败。
```
$ cargo test
   Compiling cacher v0.1.0 (file:///projects/cacher)
    Finished test [unoptimized + debuginfo] target(s) in 0.72s
     Running target/debug/deps/cacher-4116485fb32b3fff

running 1 test
test tests::call_with_different_values ... FAILED

failures:

---- tests::call_with_different_values stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src/lib.rs:43:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    tests::call_with_different_values

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```
这里的问题是我们第一次用 1 来调用 `c.value` 的时候，`Cacher` 就把 `Some(1)` 保存到了 `self.value` 中了。之后不管传什么值，调用value，返回都会返回1.

尝试修改 `Cacher`，用一个 hash map 来代替单独的一个值。hashmap 的 key是传递进来的 arg的值，但是value则是对应的 key 的闭包返回的值。之前我们会用 `self.value` 来看是 `Some` 还是 `None`，但是现在我们会用去map中找值，如果存在就返回，如果不存在就调用闭包，把结果保存map 对应 `arg` 的位置。

当前的 `Cacher` 实现的第二个问题是，我们只能传入一个 `u32` 的值，并且返回一个 `u32` 的值的闭包。举个栗子，我们想要能够缓存一个 传入字符串切片(string slice)，然后返回这个切片长度的闭包的结果。为了解决这个问题，尝试引入更多的泛型参数来增加 `Cacher` 的灵活性。
## 1.5 使用闭包来使用环境的变量(Capturing the Environment with Closures)

# 2 (使用迭代器处理元素序列) Processing a Series of Items with Iterators

## 2.1 The Iterator Trait and the next Method

## 2.2 Methods that Consume the Iterator

## 2.3 产生其他迭代器的方法(Methods that Produce Other Iterators)


## 2.4 实现 Iterator trait 来创建自定义迭代器 (Creating Our Own Iterators with the Iterator Trait)


# 3 改进 I/O 项目(Improving Our I/O Project)


# 4 性能对比：循环 VS 迭代器(Comparing Performance: Loops vs. Iterators)



# 5 总结 Summary

