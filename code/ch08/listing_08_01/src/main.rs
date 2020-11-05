fn main() {
    println!("Hello, world!");
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(5);
    v.push(5);
    v.push(5);
    v.push(5);
    v.push(5);
  
    v.push(5);
    let mut v1 : Vec<i32>  = Vec::new();
    v1.push(6);


    // 创建
    let mut string_vec: Vec<String> = Vec::new();
    string_vec.push(String::from("adfadfs"));
    use_vec_in_function(string_vec);
}

fn use_vec_in_function(v: Vec<String>) {
    for i in v.iter() {
        println!("i: {}", i);
    }
}
