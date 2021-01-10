use std::slice;

fn main() {
    // let mut v = vec![1, 2, 3, 4, 5, 6];
    // let (a, b) = split_at_mut(&mut v, 7);
    // for i in a.iter() {
    //     println!("i is {}", i);
    // }

    // for e in b.iter() {
    //     println!("e is {}", e);
    // }

    // let s: &mut [i32] = &mut [1, 2, 3, 4, 5, 6];
    // let (a, b) = split_at_mut_slice(s, 3);
    // for i in a.iter() {
    //     println!("i is {}", i);
    // }
    // mut 是否声明在变量的前面仅仅是决定这个变量是否可以被重新赋值。
    // 但是这个后面的这个号加一个mut是个什么作用？
    
    // let sie: &mut [i32] = &mut [1, 2, 3, 4, 5, 6];
    // let (a, b) = new_split_at_mut(sie, 3);
    // for i in a.iter_mut() {
    //     // let mut tmp_i = *i;
    //     if *i == 4 {
    //         *i = 1111;
    //     }
    //     println!("i: {}", i);
    // }


    // for i in b.iter() {
    //     println!("i: {}", i);
    // }

    let mut v = [1, 2, 3, 4, 5, 6];
    let s_mut = &mut v;
    let s_mut_0 = &mut v;
    // 为什么无法借用两次？会产生安全问题？
    s_mut[0] = 111;

}
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    // assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

fn split_at_mut_slice(s: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = s.len();
    // ptr: *mut i32
    let ptr = s.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            // add 我理解的是一种指针的移动的操作，会根据现在的地址然后add变量后得到一个新的地址
            slice::from_raw_parts_mut(ptr.add(mid), len)
        )
    }
}


/// 从这里可以看出一个很有趣的现象，就是
fn new_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid < len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr.add(mid), len -mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}