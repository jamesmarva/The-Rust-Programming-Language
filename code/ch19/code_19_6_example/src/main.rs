use std::slice;
fn main() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    let (a, b) = split_at_mut(&mut arr, 3);
    for item in a {
        println!("item: {}", item);
    }
    println!("b len: {}", b.len());
    for ele in b {
        println!("ele: {}", ele);
    }

}


fn split_at_mut(slice: &mut [i32], mid: usize) ->(&mut [i32], &mut [i32]){
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid),  len - mid))
    }
}