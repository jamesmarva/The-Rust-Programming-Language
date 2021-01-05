fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}


fn demo_1() {
    let mut n = 5;
    let r1 = &n as *const i32;
    let r2 = &mut n as *mut i32;
}

fn demo_2() {
    let mut n = 4;
    let r1 = &n as *const i32;
    let r2 = &mut n as *mut i32;
}