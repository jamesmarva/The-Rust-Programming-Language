

# 1 Closures: Anonymous Functions that Can Capture Their Environment

## 1.1 Creating an Abstraction of Behavior with Closures
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










