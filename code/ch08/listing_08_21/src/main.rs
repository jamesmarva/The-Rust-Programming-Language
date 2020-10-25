use std::collections::HashMap;

fn main() {
    
    let teams = vec![String::from("Blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];
    let mut scores : HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    let mut map = HashMap::new();
    map.insert("asdfsadf", 111);
}
