fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let (a, b) = split_at_mut(&mut v, 3);

}

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     assert!(mid <= len);
//     (&mut slice[..mid],
//     &mut slice[mid..])
// }


fn split_at_mut<'a>(slice: &'a mut [i32], mid: usize) -> (&'a mut [i32], &'a mut [i32]) {
    let len = slice.len();
    let l1 = mid;
    let l2 = len - mid;
    
}