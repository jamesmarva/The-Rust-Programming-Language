



# 1 面向对象语言的特点（Characteristics of Object-Oriented Languages）

```rust

```
## 1.1 对象包含数据以及行为（Objects Contain Data and Behavior）


## 1.2 封装隐藏了实现细节(Encapsulation that Hides Implementation Details)

```rust
pub struct AveragedCollection { 
    list: Vec<i32>, 
    average: f64, 
} 
```
代码17-1 在 `AveragedCollection` 结构体维护了一个整型的列表和集合里的平均值

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```
代码 17-2


## 1.3 Inheritance as a Type System and as Code Sharing



# 2 Using Trait Objects That Allow for Values of Different Types




# 3 Implementing an Object-Oriented Design Pattern


