fn main() {
    // let x = Some(50);
    // let x = Some(5);
    let x = None;
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("end: x = {:?},  y = {:?}", x, y);


    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 6;
    match x {
        1..=5 => println!("one through five"),
        6..=10 => println!("six through ten"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("default"),
    }
}
