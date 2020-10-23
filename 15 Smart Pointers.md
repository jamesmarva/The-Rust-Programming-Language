**指针 （pointer）** 是一个包含内存地址的变量的通用概念。

# 1 使用 `Box <T>` 指向堆上的数据 (Using Box<T> to Point to Data on the Heap)


## 1.1 使用 Box<T> 在堆上储存数据(Using a Box<T> to Store Data on the Heap)

## 1.2 Box 允许创建递归类型(Enabling Recursive Types with Boxes)

### 1.2.1 cons list 的更多内容(More Information About the Cons List)

### 1.2.2 Computing the Size of a Non-Recursive Type

### 1.2.3 Using Box<T> to Get a Recursive Type with a Known Size

# 2 通过 Deref trait 将智能指针当作常规引用处理(Treating Smart Pointers Like Regular References with the Deref Trait)
在实现了 `Deref` trait 之后，我们就可以去定制化 *解引用操作符了(dereference operator)* `*`的行为了。实现了 `Deref` trait 的智能指针可以被当成是常规的引用，你可以编写作用于引用的代码，同时这部分的代码也可以用于智能指针。

先来看看解引用操作符是如何作用于常规引用的。然后我来尝试自定义一个和 `Box<T>` 行为很像的类型，来看看为什么解引用操作不能在我们自定义的新的类型像个引用一样工作。我们去探索如何实现 `Deref` trait ，使智能指针可以像常规引用一样工作。然后我们会看看Rust `deref coercions` 的这个特性，以及它是如何处理引用，以及或者智能指针的。


## 2.1 解引用可以跟着指向值的指针。（Following the Pointer to the Value with the Dereference Operator）
常规引用是一种指针类型，理解指针的一种方式就是，把指针看成是指向存储在某个存储在其他位置的箭头。就像代码15-6 中的，

```rust
fn main(){
    let x = 5;
    let y = &x;
     
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```


# 3 







 