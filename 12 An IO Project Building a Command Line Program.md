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

第二个问题也和第一个问题有关：尽管 `query` 和 `filename` 都是我们程序的配置变量，像 `contents`这样的变量是用与执行程序的逻辑的。随着 `main` 函数的代码量的增长，越来越多的变量会需要出现在作用域。作用域的变量越多，那么跟踪每个变量的目的就会变得困难。最好将配置变量都集合到一个结构中，以明确其用途。

第四个问题，我们反复用 `expect` 来处理不同的错误，如果用户没有传足够数量的参数给我们程序，他们只会会得到一个提示信息是 `index out of bounds` 的Rust的内部错误，但是这些信息还是不足以知晓错误的原因。

让我们来通过重构项目来解决以上四个问题。
##  3.1 在项目中的关注点的分离(Separation of Concerns for Binary Projects)
把过多个功能，过多的职能的带你们放在 `main` 函数里的代码组织问题在许多的项目中都很常见。为了解决这个问题，Rust 社区开发了一种流程，当 `main` 函数开始变得臃肿的时候，将拆分成单个职能的函数的指南。这个拆分的过程有以下的步骤：
- 把你的代码分成 *main.rs* 和 *lib.rs*，然后将你的程序的代码逻辑搬到 *lib.rs* 中。
- 只要你的解析的参数的代码量够小，那么它们就保留在 *main.rs*
- 当命令行的参数解析的代码开始变的复杂的时候，请将他们提取到 *lib.rs* 中。

在这个过程之后，保留在 `main` 函数中的代码要被限制于以下的各项：
- 调用命令行的参数解析代码
- 设置任何其他的配置
- 调用 *lib.rs* 的 `run` 函数
- 要处理  `run` 函数中出现的错误。

这种模式是关于分类问题的：*main* 运行程序，而 *lib.rs* 用于处理手头的任务的所有的逻辑。因为你无法直接测试 `main` 的功能，所以只能把它们提取到 *lib.rs* 中进行测试。保留在 *main.rs* 中的代码量足够小，小到可以直接通过读代码来验证其正确性。让我们通过以下的步骤来重写我们的代码。

## 3.2 提取参数解析(Extracting the Argument Parser)
我们将要把解析参数功能的代码提取到一个 `main` 函数将会调用的单独的函数中，为了随后转移到 *src/lib.rs* 中做好准备。代码12-5 显示了定义了 `parse_config` 函数
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

我们仍然会将参数先收集到 Vector 中，但是却没有在 `main` 函数中把处于 索引1 的位置的值赋予 `query`，把索引为2 的位置的元素赋予 `filename`，而是直接将整个vector传给 `parse_config` 函数。函数 `parse_config` 包含了确定要将哪个参数返回给哪个变量的逻辑，以及将它们返回给 `main` 函数。我们继续在 `main` 函数中创建`query`和 `filename` 两个变量。但是main函数不在负责命令行的参数和变量会如何映射。

对于这样的小程序而言，这样的重写似乎有点过了，但是重构工作正是要这样小型的，缓慢的前进的。在进行更改之后，再次运行程序确保这次参数解析的修改是ok的。经常检查你的程序是个好习惯，这样有助于你确定问题发生的原因。
## 3.3 一统配置值(Grouping Configuration Values)
我们采用另一个方案来改进我们 `parse_config` 函数。目前，我们这个函数返回的是一个元祖，但是在使用的时候我们再次将元组拆分。这就表示我们还没有正确的将功能抽象。

另一个可以改进的部分就是 函数 `parse_config` 的`config` 的部分，这样就意味着我们返回的两个值都是相关的，都是配置值的一部分。我们可以将他们放到一个数据结构中，然后给每个字段一个有意义的名称。这样有利于以后的开发人员维护代码的是明白每个值的含义，以及它们之间的关系。

> 注意，当复杂类型更加合适的时候，使用基本类型就被成为一种 *基本类型偏执(primitive obsession)* 的反模式(anti-pattern)。

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



## 3.4 给结构体 `Config` 创建一个构造器 (Creating a Constructor for Config)
目前为止，我们已经从 `main` 函数中提取了解析命令行参数的职能，并且把这个功能放到 `parse_config` 函数中。这样有利于我们看到的 `query` 和 `filename`的值是相关的，并且应该在我们的代码中传达这种关系。

因此， 既然 `parse_config` 的目的是为了创建一个 `Config`实例，我们就可以把 `parse_config` 从一个普通的函数更改为结构体 `Config` 想关联的 `name` 的函数。这样的修改会让代码更加好用。通过将调用`Config::new` 来创建结构体`Config`的实例。
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
代码12-7 把 `parse_config` 代码放入 `Config::new`

上面这段代码将 `main` 函数中原来调用`parse_config`函数的位置修改为调用`Config::new`。将 `parse_config` 名字改成 `new` 并且把它移动到 `impl` 代码块中，这个功能将和 `Config` 的关联，并且尝试再次编译这个代码，确保可以运行的。

## 3.5 新增错误处理(Fixing the Error Handling)
现在，我们将会修复错误的处理。回想一下，如果vector中的元素少于三个，那么就会导致出现崩溃。在不带任何参数的情况下试运行一下。
```rust
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:27:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```
`index out of bounds: the len is 1 but the index is 1` 这样错误是针对程序员的错误。但是这个错误不会帮助我们知晓到底发生了什么，也不会告诉我们应该做什么，接下来我们解决这个问题。
### 3.5.1 改进错误信息(Improving the Error Message)
代码 12-8 我们在 `new` 函数中增加一个检查，这个检查可以确保这个vector中的元素至少三个。如果这个切片长度不够，那么程序在崩溃的时候会报错，并且会产生更好的错误信息。
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
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // --snip--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        // --snip--

        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```
代码 12-8 增加参数数量的检查器

这个代码和 [the Guess::new function we wrote in Listing 9-10]( s://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation)有点像，当参数超过了有效值的范围的时候，程序就会报错。我们将会检查 `args` 的长度，至少为3个。其余的功能要在这个满足条件下才能运行。如果这个 `args` 少于三个元素的话，这个条件就会成立，我们就会调用 `panic!` 来立即结束程序。

当我们在无参的情况下运行代码的时候，就会出现下面这个些错误。
```rust
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`
thread 'main' panicked at 'not enough arguments', src/main.rs:26:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```
这个输出无疑更好的：我们现在有了合理的错误消息，可是，我们也有不想给用户额外的消息。也许用代码9-10的技术不是最好的技术：调用一个 `panic!` 函数更像是个解决编程问题，而不是一个应用的问题。你可以用另一个技术来解决这个问题——返回一个 `Result` 对象来表示函数成功或是失败了。

### 3.5.2 new函数返回一个Result对象，而不是调用 `panic!` Returning a Result from new Instead of Calling panic!
我们可以返回一个 `Result` 值，这个值会在的成功的情况下返回一个包含 `Config` 对象的 `Result`值，会在失败的情况下返回一个带有问题描述的 `Result`对象。当我们调用 `Config::new` 的饿时候，就可以用这个 `Result` 来知晓返回的结果是否存在问题。然后，我们还可以更改在 `main` 函数中出现 `Err`，转换为更为实际的错误信息给用户。用这种方法可以避免调用 `panic!` 函数的时候在错误提示信息里的关于 `thread 'main'` 和 `RUST_BACKTRACE` 等内部信息。

代码12-9 将 `Config::new` 返回值修改为 `Result`。注意，这里的代码还不能编译，要到下一个示例修改了`main`函数的代码之后才能编译。
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
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```
代码12-9 函数`Config::new`返回一个 Result 对象

现在这个`new`函数返回的是一个 `Result` 类型的对象，在成功的时候返回一个 `Congfig` 实例，而在失败的时候带着一个 `&'static str`对象。

`new`的函数体的代码有两个修改：当没有足够的参数传进来的私活，不在调用 `panic!`,而是返回一个 `Err` 值。如果条件满足，就会返回一个 包裹着 `Config` 对象的 `OK`。这些代码修改使得函数符合它现在的类型签名。


### 3.5.3 调用 `Config::new` 并处理错误 Calling Config::new and Handling Errors
为了处理错误并打印出对用户友好的信息，我们需要修改 `main` 函数代码来处理 `Config::new` 返回的 `Result` 类型的值，如代码12-10所示。我们还需要去实现原来的 `panic!` 实现的代码职责：用非0的错误码来退出程序。非零的错误码就一个信号，习惯用来告诉调用的程序：这个程序是错误状态退出的。
```rust
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```
代码12-10 如果创建了 `Config` 失败了，就退出程序

上面的程序用了之前没有用过的方法：`unwrap_or_else`，这个函数在标准库中 `Result<T, E>`中。用 `unwrap_or_else` 可以让我们定义一些自定义的，不带 `panic!`的错误处理。如果 `Result`是 `Ok` 的值，这个方法就类似于 `unwrap`：他会返回内部的值。然而，如果值是 `Err`的时候，那么这个方法就会调用一个*闭包(closure)*，which is an anonymous function we define and pass as an argument to unwrap_or_else. 我们将会在第13章讨论更多关于闭包的知识。只是现在你只要知道 `unwrap_or_else` 会把的`Err`的值，也就是`not enough arguments`，传给 两个竖杠的中间的 `err`。

我们用 `use` 来引入 `process`。在错误的情况之后两行：我们打印了 `err`，然后直接调用了 `std::process::exit`。这个方法立即停止程序，并且将传给它的数字作为退出的状态码。这样就不会输出额外的输出了。
```
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48 secs
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

## 3.6 提取 `main` 函数的逻辑 Extracting Logic from main
以上是就是我们完成的配置解析的重构，接下来继续程序的逻辑重构。就像我们在 “二进制项目的分离重点” 里面所讨论的那样，我们将会提取出不属于配置或者处理错误的所有逻辑。一旦完成这些，我们就可以直接看代码来验证代码的正确性了，而且可以能够为所有的的逻辑编写测试用例了。

代码12-11 显示了提取出来的 `run` 函数。现在，我们仅仅写了一个小型的增量的提取函数。
```rust
use std::env;
use std::fs;
use std::process;

fn main() {
    // --snip--

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// --snip--

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```
代码12-11 提取 一个 `run` 的函数的包含剩下的程序逻辑




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