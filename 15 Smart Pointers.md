














解引用强制多态如何与可变性交互


# 3 用 Drop Trait 来清理代码 （Running Code on Cleanup with the Drop Trait）

```
struct CustomSmartPointer { 
    data: String, 
} 
 
impl Drop for CustomSmartPointer { 
    fn drop(&mut self) { 
        println!("Dropping CustomSmartPointer with data `{}`!", self.data); 
    } 
} 
 
fn main() { 
    let c = CustomSmartPointer { data: String::from("my stuff") }; 
    let d = CustomSmartPointer { data: String::from("other stuff") }; 
    println!("CustomSmartPointers created."); 
} 
```
代码 15-14 

## 3.1 用 `std::mem::drop` 提前丢弃值

```

```









