use std::slice;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut v, 7);
    for i in a.iter() {
        println!("i is {}", i);
    }

    for e in b.iter() {
        println!("e is {}", e);
    }
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
