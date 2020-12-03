- UnSafe Rust:
- Advanced traits:
- Advanced types:
- Advanced function and closures:
- Macros:

# 1 不安全Rust （Unsafe Rust）
到目前为止，我们所有涉及的代码都是在编译期就强制保证了内存安全的。

## 1.1 Unsafe Superpowers

- Dereference a raw pointer
- 调用不安全的函数或者方法
- 读取或者修改一个可变的静态变量
- 实现一个不全的trait
- 读取一个 `union` 的字段（field）

## 1.2 Dereferencing a Raw Pointer

不同于引用（Reference）和 智能指针（smart points），裸指针：
- 允许忽略借用元祖，可以同时有可变的和不可变的指针，或者多个指针同时指向一个位置
- 不保证指向一个有效的内存
- 允许裸指针是 null
- 不会自动清理

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as &mut i32;
}
```
19-1 


## 1.3 Calling an Unsafe Function or Method


### 1.3.1 Creating a Safe Abstraction over Unsafe Code
```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
```
19-4 使用安全的函数 `split_at_mut`



### 1.3.2 Using extern Functions to Call External Code


## 1.4 Accessing or Modifying a Mutable Static Variable

## 1.5 Implementing an Unsafe Trait

## 1.6 Accessing Fields of a Union

## 1.7  When to Use Unsafe Code

# 2 高级 Trait （Advanced Traits）

## 2.1 Specifying Placeholder Types in Trait Definitions with Associated Types