fn main() {
    let v = vec![1, 2, 4, 5, 6];
    let v1 = vec!['a', 'z', 'b', 'c', 'r', 't'];
    println!("largest: {}", larget(&v1));
}


fn larget<T>(l: &[T]) -> T 
    where T: PartialOrd + Copy 
{
    let mut rst = l[0];
    for &i in l {
        if i > rst {
            rst = i;
        }
    }
    rst
}