fn main() {
    use std::collections::HashMap;
    let teams = vec![String::from("Blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];
    let mut scores : HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
}
