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

## 1.2 Refactoring Using Functions
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










