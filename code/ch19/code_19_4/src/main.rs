fn main() {

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    for i in a.iter() {
        println!("i is: {}", i);
    }
    assert_eq!(a, &mut [1, 4, 3]);
    assert_eq!(b, &mut [4, 3, 6]);
}