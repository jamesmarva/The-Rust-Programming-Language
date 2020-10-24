fn main() {
    
}

struct Cacher<T> 
    where T: 
        Fn(u32) -> u32    
{

    calcalution: T,
    value: Map<u32, u32>
}
