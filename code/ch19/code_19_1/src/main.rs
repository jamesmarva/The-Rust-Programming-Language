fn main() {
    let mut n = 5;

    // 不可变转成裸指针的格式
    let r1 = &n as *const i32;

    // 可变转成裸指针的格式
    let r2 = &mut n as *mut i32;
}
