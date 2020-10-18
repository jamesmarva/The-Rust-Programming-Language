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


## 1.2 用函数来重构代码(Refactoring Using Functions)


## 1.3 重构的时候使用闭包储存代码 (Refactoring with Closures to Store Code)


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
代码13-6 

# 2 (使用迭代器处理元素序列) Processing a Series of Items with Iterators

## 2.1 The Iterator Trait and the next Method

## 2.2 Methods that Consume the Iterator

## 2.3 产生其他迭代器的方法(Methods that Produce Other Iterators)


## 2.4 实现 Iterator trait 来创建自定义迭代器 (Creating Our Own Iterators with the Iterator Trait)


# 3 改进 I/O 项目(Improving Our I/O Project)


# 4 性能对比：循环 VS 迭代器(Comparing Performance: Loops vs. Iterators)



# 5 总结 Summary

