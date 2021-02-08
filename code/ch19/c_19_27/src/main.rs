fn main() {

    let l_of_num = vec![1, 2, 3];
    let l_of_string: Vec<String>  = l_of_num.iter()
        .map(|x| x.to_string())
        .collect();
    for s in l_of_string.iter() {
        println!("{}", s);
    }

    let list_of_num = vec![1, 2, 3];
    let list_of_string: Vec<String> = list_of_num.iter()
        .map(ToString::to_string)
        .collect();
    for s in list_of_string.iter() {
        println!("{}", s);
    }
}
