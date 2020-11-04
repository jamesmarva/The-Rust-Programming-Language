fn main() {

    let mut v = vec![1, 3, 4, 5, 7, 8, 10];
    v.push(1);
    v.push(2);

    v[1] = 100;
    println!("{}", &v[1]);
    

    let mut v1: Vec<String> = Vec::new();
    v1.push(String::from("hhhhhh"));
    v1.push(String::from("qqqqqqqqq"));
}
