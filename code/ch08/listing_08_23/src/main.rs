use std::collections::HashMap;
fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score: {:?}", score);

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
}
