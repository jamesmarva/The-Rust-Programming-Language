fn main() {
    mut_reference_right()
}

fn  not_mut() {
    let v = vec![1, 2, 3];

    // 如果v 前面没有声明 mut，这样很明显是不行的
    // v[1] = 2;
}

fn mut_vec() {
    let mut v = vec![1, 2, 3];
    // 这样是没有问题的，因为这样可以，这是个mut的对象，所以就可以对这个mut的数据结构
    // 里面的值进行修改操作
    v[1] = 3;
}

fn const_referece() {
    let v = vec![1, 2, 3];
    let v_reference = &v;
    println!("index 1: {}", v_reference[1]);
}

fn mut_reference() {
    let v = vec![1, 2, 3];
    // 因为这里得v的声明的时候就没有带着mut，所以要借用可变的，首先得要有，原来的owner
    // 是immutable的，也就是不可变的。既然自己都没有这个的东西，还要怎么往外借？
    // let v_mut_ref = &mut v;
    // v_mut_ref[1] = 1;

}

fn mut_reference_right() {
    let mut v = vec![1, 2, 3];
    let v_mut_ref: &mut [i32] = &mut v;
    v_mut_ref[0] = 12323;
    println!("indx 0: {}", v_mut_ref[0]);
}

