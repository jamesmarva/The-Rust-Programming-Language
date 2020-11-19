

# 1 自定义发布和开发的时候的配置（caidingCustomizing Builds with Release Profiles）



# 2 发布 crate 到 `Crates.io`Publishing a Crate to Crates.io

```
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
代码14-1 一个函数的文档（document comment）



可以运行 `cargo doc` 来生产文档注释的html文件

# 3 Cargo Workspaces


# 4 Installing Binaries from Crates.io with cargo install




# 5 Extending Cargo with Custom Commands










