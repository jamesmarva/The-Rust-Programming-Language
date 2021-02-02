fn main() {
    let v0 = vec![1, 3, 4, 5123, 11, 3, 4, 66];
    println!("larget: {}", largest(&v0))
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut rst: &T = &list[0];
    let len = list.len();
    let mut idx = 1;
    while idx < len {
        if &list[idx] > rst {
            rst = &list[idx]
        }
        idx += 1
    }
    rst
}