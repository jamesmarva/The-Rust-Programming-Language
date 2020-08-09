fn main() {
    println!("Hello, world!");


    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];
    
    for item in number_list {
        if item > largest {
            largest = item;
        }
    }
    println!("The largest number is {}", largest);

}
