fn main() {
    let a = CustomSmartPointer {data: String::from("aaa")};
    let b = CustomSmartPointer {data: String::from("bbbb")};
    drop(a);
    println!("finish.");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("drop {}", self.data);
    }
}


