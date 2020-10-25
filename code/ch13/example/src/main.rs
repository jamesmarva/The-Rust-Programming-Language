use std::collections::HashMap;

fn main() {
    
}

struct Cacher<T> 
    where T: 
        Fn(u32) -> u32    
{

    calcalution: T,
    value: HashMap<u32, u32>
}
