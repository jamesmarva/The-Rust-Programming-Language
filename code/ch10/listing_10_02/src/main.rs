fn main() {
    
    let num_list = vec![34, 50, 25, 100, 65];

    let mut largest = num_list[0];
    
    // num_list moved here.
    for item in num_list {
        if item > largest {
            largest = item;
        }
    }


    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];
    for item in number_list {
        if item > largest {
            largest = item;
        }
    }

    println!("The largest number is {}", largest);
}
