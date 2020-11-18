
pub trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>
}

impl Screen {
    fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

fn main() {
    let s = Screen {
        components : vec![
            Box::new(String::from("Hi")),
        ]
    };
}
