fn main() {
    let mut num = 5;

    // 不可变转成裸指针的格式
    let r1 = &num as *const i32;

    // 可变转成裸指针的格式
    let r2 = &mut num as *mut i32;
}
