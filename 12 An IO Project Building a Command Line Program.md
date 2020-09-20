这个章节就是对前面你所学的技能的一次回顾，以及对标准库的特性的一次探索。


# 1 获取命令行的参数(Accepting Command Line Arguments)
让我们用命令 `cargo new` 创建一个新的项目。我们把我们的项目称之为`minigrep`，以便来区分 你的系统里的`grep` 工具。
```shell
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```
第一个任务就是让项目 `minigrep` 接受两个命令行的参数：一个文件名和用于搜索的字符串。也就是说，我希望在运行的时候，用`cargo run`，一个文件名，一个字符串用于搜索，如下所示：
```rust
$ cargo run searchstring example-filename.txt
```
目前，由 `cargo new` 生成的代程序不能处理我们提供的参数。`cargo.io` 上某些现有的库可以帮助我们处理这些命令行的参数，但是因为你只是学习这个概念，所以请你自己实现这些功能。

## 1.1  读取参数(Reading the Argument Values)
为了能让 `minigrep` 可以读取传递给他们命令行的参数，我们需要Rust 标准库的提供的函数，就是 `std::env::args`。这个函数将会返回我们传递给 `minigrep` 命令行参数所组成的迭代器。我们将会在第13章中全面介绍迭代器。目前，你只要知道两个有关于迭代器(iterator)的两个细节：迭代器会产生一系列的值，我们可以在迭代器上调用 `collect` 方法来将其转为一个集合(collection)，比如 vector，其中就包含迭代器生成的所有的元素。

代码12-1 中的代码，程序就可以读取传递给 `minigrep` 的所有的命令行的元素了，而且可以把这些元素收集到 vector中。

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```
代码12-1 收集所有的命令行参数到vector，然后输出他们

首先，我们用 `use` 语句把 `std::env` 模块引入作用域中，以便可以用 `args` 函数。注意，这里的 `std::env::args` 函数是嵌套在两个层级的模块中。就像我们在第7章中所讨论的那样，如果所需的函数是在嵌套在多个层级的模块中的额，那么通常是将其的父模块引入作用域中而不是直接把函数引入作用域中。这样，我们就可以轻松的使用 `std::env` 中的其他的函数了。比起 仅仅使用`args` 相比，使用`env::args`不会那么模凌两可，因为直接调用 `args`函数会让人误以为是当前模块里定义的函数。

> # The args Function and Invalid Unicode
> 注意，这个 `std::env::args` 在参数的包含非法 unicode 的时候会报错。如果你的程序需要传入一些非法字符作为参数，请用 `std::env::args_os` 来代替。

在 main 函数的第一行，我们用 `collect` 来将 迭代器的值转为 vector，里面都是迭代器里的值。我们可以用`collect`来创建多种类型的集合，因此我们需要显示注释 `args` 的类型，以指定我们想要一个字符的vector。尽管我们很少需要在Rust中去注释类型， `collect` 是一个你需要经常需要注释的函数，因为 Rust 无法推断出你想要的集合的类型。

最后，我们调用格式化程序，`:?`来打印 vector。让我们来先运行无参，然后在运行有参的情况：
```shell
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/minigrep`
["target/debug/minigrep"]
```

```shell
$ cargo run needle haystack
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/minigrep needle haystack`
["target/debug/minigrep", "needle", "haystack"]
```
请注意，vector的第一个元素是 `target/debug/minigrep`，这个是我们而精致的额文件的名称。 这个和 c 语言的参数列表是一致。


## 1.2 用变量来保存参数 (Saving the Argument Values in Variables)
打印vector的参数说明这个程序的可以访问命令行传给程序的参数。现在我们需要把两个参数保存在程序的变量中，以便程序的剩下的部分可以使用这两个值。
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```
代码12-2 创建用来保存参数的变量

正如我们在打印 vecotr的时候看到的，程序的名称占用了 vector的第一个值 `args[0]`，所以我们的要从 索引的 1 开始。 `minigrep` 的第一个参数就是我们需要搜索的字符串，因此我们吧第一个参数的引用放入变量 `query`。第二个参数是个文件的名字，所有我们将文件的名字的引用放到一个变量中。

我们临时打印这些变量的值，以证明代码是按照预期工作的。让我们用参数`test`和 `sample.txt` 来运行程序。
```rust
$ cargo run test sample.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep test sample.txt`
Searching for test
In file sample.txt
```
程序跑起来了。我们需要吧参数值保存到正确的变量中，稍后我们将会添加一些错误来处理一些潜在错误，比如用户在不提供的参数的时候，会出错。不过现在我们将会忽略这种情况，而改为继续讨论读取文件的功能。
# 2 读取文件(Reading a File)
现在我们将会添加功能来去读取指定命令行中的文件名的文件。首先，我们需要一个示例文件来进行测试：最好的用来确保 `minigrep` 工作的最好的文件类型就是在一个文本文件内有几行文字，并且含有重复的单词。Emily Dickinson的诗“I’m Nobody! Who are you?”就是很不错的例子。
```rust
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us - don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```
代码12-3：一首来自Emily Dickinson的诗

填充好文本之后，编辑 `src/main.rs` 并且添加读取文件的代码
```rust
use std::env;
use std::fs;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```
代码12-4 读取第二个参数指定文件的内容

首先，我们增加另一个 `use` 表达式来引入标准库的相关的部分：我们需要 `std::fs` 来处理文件。

在 `main` 函数中，我们再次添加了一个临时的`println!` 来输出文件内容，这样就可以验证程序是否是正确的。

第一个参数无所谓，但是以 *poemtxt* 来作为第二个参数。
```shell
$ cargo run the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us - don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```
代码读取并且打印了文件的内容。但是代码有缺陷，`main`函数有太多的职责：通常，如果每个函数仅有一个职责，那么功能将会更加清晰，更加易于维护。另一个问题是我们没有尽全力的处理错误。这个程序依旧很小，这些问题还不是问题，但是随着程序的开发，很难彻底将其修复。最好在开发程序的时候就尽早开始重构，因为重构少量的代码就会让容易的多。我们将在下一步继续。

# 3 重构以提高模块化和错误处理(Refactoring to Improve Modularity and Error Handling)
为了改进我们的程序，我们将会修复和程序结构相关的问题以及潜在的问题。

首先，我们的 `main` 函数现在执行两个任务：既要解析参数也要读取文件。对于这么小的程序，这都不是主要的问题。但是如果我们继续往 `main` 函数里扩展我们的程序，在 `main` 函数的处理的任务的数量将会增加。随着函数获取到更多的职能，在不破坏其单一的功能的情况下，就变得难以维护，难以测试。最好的做法就是将函数里的功能分开，以便每个功能都可以完成单一的职能。

第二个问题也和第一个问题有关：尽管 `query` 和 `filename` 都是我们程序的配置变量，像 `contents`这样的变量是用与执行程序的逻辑的。随着 `main` 函数边长，越来越多的变量会需要出现在作用域。作用域的变量越多，那么跟踪每个变量的目的就会变得困难。最好将配置变量都集合到一个结构中，以明确其用途。

第四个问题，我们反复用 `expect` 来处理不同的错误，如果用户没有传足够数量的参数给我们程序，他们将会得到一个 `index out of bounds` 的错误，但是这些问题还是没有得到很好的解释。

让我们来通过重构项目来解决以上四个问题。
##  3.1 在项目中的关注点的分离(Separation of Concerns for Binary Projects)
把多个职能分配给 `main` 函数的代码组织问题在许多的项目中都很常见。Rust 社区开发了一种流程，当 `main` 函数开始变得臃肿的时候，将拆分成单个职能的函数的指南。这个拆分的过程有以下的步骤：
- 把你的代码分成 *main.rs* 和 *lib.rs*，然后将你的程序的代码逻辑搬到 *lib.rs* 中。
- 只要你的解析的参数的代码量够小，那么它就能保留在 *main.rs*
- 当命令行的参数解析开始变的复杂的时候，请将他们提取到 *lib.rs* 中。

在这个过程之后，保留在 `main` 函数中的代码要被限制于以下的各项：
- 调用命令行的参数解析逻辑
- 设置任何其他的配置
- 在 *lib.rs* 调用 `run` 函数
- 要处理  `run` 函数中出现的错误。

这种模式是关于分类问题的：*main* 运行程序，而 *lib.rs* 用于处理手头的任务的所有的逻辑。因为你无法直接测试 `main` 的功能，所以只能把它们提取到 *lib.rs* 中进行测试。保留在 *main.rs* 中的代码量足够小，可以通过读代码来验证其正确性。让我们通过以下的步骤来重写我们的代码。

## 3.2 提取参数解析(Extracting the Argument Parser)
我们将会吧解析参数的功能提取到一个 `main` 函数将会调用的函数中。代码12-5 显示了定义了 `parse_config` 函数
```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --snip--

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
```
代码12-5 提取代码到 `parse_config` 

我们仍然会将参数先收集到 Vector 中，但是却没有在 `main` 函数中把处于 index  1 的位置的值赋予 `query`，把index 2 的位置的元素赋予 `filename`，而是直接将整个vector传给函数 `parse_config`。函数 `parse_config` 包含了确定要将哪个参数返回给哪个变量的逻辑，以及将它们返回给 `main` 函数。我们继续在 `main` 函数中创建`query`和 `filename` 两个变量。但是main函数不在负责命令行的参数和变量会如何对应。

对于这样的小程序而言，这样的重写似乎有点过了，但是我们正在逐步进行小型重构。在进行更改之后，再次运行程序确保这次参数解析的修改是ok的。经常检查你的程序是个好习惯，这样有助于你确定问题发生的原因。
## 3.3 统一配置值(Grouping Configuration Values)
我们采用另一个方案来改进我们 `parse_config` 函数。目前，我们这个函数返回的是一个元祖，但是随后我们再次将元祖分解。这就表示我们还没有正确的将功能抽象。

另一个可以改进的部分就是 函数 `parse_config` 的`config` 的部分，这样就意味着我们返回的两个值都是相关的，都是配置值的一部分。我们可以将他们放到一个数据结构中，然后给每个字段一个有意义的名称。这样有利于以后的开发人员维护代码的是明白每个值的含义，以及它们之间的关系。

> 注意，当复杂类型更加合适的时候，使用基本类型就被成为一种 *原始* 的反模式。

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    // --snip--

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```
代码 12-6 把 `parse_config` 的返回对象改为 `Config` 结构体。

我们添加了一个名为 `Config` 的结构体，其中包含两个字段 `query` 和 `filename` 。现在函数`parse_config`表明了它会返回一个 `Config` 类型的值。在函数体 `parse_config` 中，我们曾经用返回字符串切片的数据作为返回值，我们现在定义了持有 `String` 类型得值的对象的结构体`Config`。`main` 函数是`agrs` 的所有者，并且允许 `parse_config` 函数来借用他们，这就意味着如果`Config` 尝试获取 `args` 中值的所有权，这将会违反 Rust 的借用规则。

我们有很多种不通的方式来管理 String 数据，最简单，但是效率较低的方式就是，在String的值上调用 `Clone` 方法。这将为 `Config` 结构拥有一个完整的数据副本，这种做法比用引用的方式要花更多的时间和空间。可是，克隆数据的方式也让我们的代码看起来更加简单，因为这样就不用管理引用的生命期限(lifetime)了,在这种情况下，就是为了代码简单性而放弃了一些性能的妥协。

> # 使用 clone 方法的权衡 (The Trade-Offs of Using clone)
> 许多 Rustaceans 会避免用 `clone` 来解决所有权的问题，因为这种方案的运行成本很高。在第13章，你将会学到如何用更有效率的方式来处理这种情况。

## 3.4 给结构体 `Config` 创建一个结构体 (Creating a Constructor for Config)
目前为止，我们已经从 `main` 函数中提取了解析命令行参数的职能，并且把这个功能放到 `parse_config` 函数中。这样有利于我们看到的 `query` 和 `filename`的值是相关的，并且应该在我们的代码中传达这种关系。

因此， 既然 `parse_config` 的目的是为了创建一个 `Config`实例，我们就可以把 `parse_config` 将一个普通的函数更改为结构体 `Config` 的一个名字是 `name` 的函数。这样的修改会让代码更加好用。通过将调用`Config::new` 来创建结构体`Config`的实例。
```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    // --snip--
}

// --snip--

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

```
## 3.5 Fixing the Error Handling

### 3.5.1 Improving the Error Message





### 3.5.2 Returning a Result from new Instead of Calling panic!



### 3.5.3 Calling Config::new and Handling Errors




## 3.6 Extracting Logic from main


## 3.7 Splitting Code into a Library Crate
目前为止看起来，我们 `minigrep`看起来还好。现在我们将分割 *src/main.rs* 的代码，并且将代码放入 *src/lib.rs* 中，并且将对其进行测试，并且有了更少职责的 *src/main.rs*。

让我们把不是 `main` 函数的代码都移动从 *src/main.rs* 到 *src/lib.rs*
- `run` 函数的定义
- 和 `use` 表达式相关的概念
- `Config` 的定义
- 函数 `Config::new` 的定义


```rust
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}               
```
代码12-13 移动 `Config` 和 `run` 到 *src/lib.rs*

```rust
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        // --snip--
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```