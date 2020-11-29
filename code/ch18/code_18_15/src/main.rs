

fn main() {
    let mut msg = Message::ChangeColor(0, 160, 255);
    msg = Message::Quit;
    msg = Message::Move{
        x: 10, 
        y: 100,
    };
    let x = 100;
    let y = 200;
    msg = Message::Move{
        x, 
        y,
    };

    msg = Message::Write(String::from("tttttttttttttttttt"));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        },
        Message::Move {x, y} => {
            println!("Move in the x direction {} and in the y direction {}",
                x, 
                y);
        },
        Message::Write(text) => {
            println!("Text message: {}", text);
        }, 
        Message::ChangeColor(r, g, b) => {
            println!( 
                "Change the color to red {}, green {}, and blue {}", 
                r, 
                g, 
                b 
            ); 
        },
        _ => () 
    }
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
