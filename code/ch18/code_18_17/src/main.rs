fn main() {

    foo(3, 4);
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

