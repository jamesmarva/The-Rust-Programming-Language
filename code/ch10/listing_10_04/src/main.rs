fn largest_32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest

}

fn largest_char (list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 50, 25, 100, 65];

    let res_int = largest_32(&number_list);

    println!("The largest number is {}", res_int);

    let char_list =  vec!['y', 'm', 'a', 'q'];

    let res_char = largest_char(&char_list);
    println!("The largest char is {}", res_char);



}
