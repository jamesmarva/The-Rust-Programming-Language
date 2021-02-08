# 1 Setting Up a New Project

# 2 Processing a Guess
### 2.1 Handling Potential Failure with the Result Type


### 2.2 Printing Values with println! Placeholders


### 2.3 Testing the First Part


# 3 Generating a Secret Number


### 3.1 Using a Crate to Get More Functionality


如果我们在这里没有增加这个类型的声明，那么Rust在编译的时候输出错误信息，这个信息让我们知道，编译器需要我们告诉它更多的信息，让它明白我们想使用什么数据类型
```shell
$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ consider giving `guess` a type

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
error: could not compile `no_type_annotations`.

To learn more, run the command again with --verbose.
```

### 3.2 Ensuring Reproducible Builds with the Cargo.lock File

### 3.3 Updating a Crate to Get a New Version

### 3.4 Generating a Random Number


# 4 Comparing the Guess to the Secret Number
```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("please input your guessing number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("your input number is :{}", guess);

    let guess : u32 = guess.trim().parse().expect("asdfasdfadsf");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("you get it!"),
    }
}
```

# 5 Allowing Multiple Guesses with Looping

### 5.1 Quitting After a Correct Guess

### 5.2 处理不合法输入 (Handling Invalid Input)




通过最后一个小调整，我们就完成了这个猜谜游戏。回顾一下，这个程序会打印出用户猜测的数字。这个才测试中的效果很好，但是却毁了游戏，让我们删除 `print!` 的输出的密码。最后的代码在代码清单 2-6 :
```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```
代码清单 `2-6` 完整的猜谜游戏的代码

# 6 Summary
恭喜你，到了这里，已经成功的创建了一个猜数字游戏。
创建这个项目的目的是为了手把手教你关于Rust的几个概念：`let`，`match`，`methods`，`associated functions`，`the use of external crates`诸如等等……在接下来的章节中，你会学习到有关于这几个概念的更多细节。第三章节中会有更多的编程语言的相关概念，比如 关键字  `variables`, `data types`还有函数，以及如何使用这些。第四章探讨的是关于“所有权”的概念。第五章讨论 结构体和使用方法的语法。第六章将解释“枚举”是如何工作的


```rust
```

