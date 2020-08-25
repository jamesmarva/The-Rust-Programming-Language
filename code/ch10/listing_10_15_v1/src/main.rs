fn largest_ele<T:PartialOrd> (list: &[T]) -> &T {
    let mut res: &T = &list[0];
    for item in list {
        if item > res {
            res = item
        }
    }
    res
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_ele(&number_list);
    println!("The largest nubmer is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_ele(&char_list);

    println!("The largest char is {}", result);
}