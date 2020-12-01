fn main() {

    enum Message {
        Hello {
            id: i32,
        }
    }

    let mut msg = Message::Hello{id: 5};

    match msg {
        Message::Hello{id: val @ 3..=6} => {
            println!("id is in range: {}", val);
        },
        Message::Hello{id: 8..=12} => {
            println!("id is in another range");
        },
        Message::Hello{id} => {
            println!("Found some other, {}", id);
        }
    }

    msg = Message::Hello{id: 15};

    match msg {
        Message::Hello{id: val @ 3..=6} => {
            println!("id is in range: {}", val);
        },
        Message::Hello{id: 8..=12} => {
            println!("id is in another range");
        },
        Message::Hello{id: id_val} => {
            println!("Found some other, {}", id_val);
        }
    }
}
