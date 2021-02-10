fn main() {

}


struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    let it = shoes.iter();
    let t: Vec<Shoe>  = shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect();
        
    // shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    t
}
