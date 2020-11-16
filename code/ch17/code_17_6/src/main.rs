pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    components: Vec<Box<T>>,
}

impl<T> Screen<T>
    where T: Draw 
{
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {

    fn draw(&self) {
        println!("draw Button {:?}", self);
    }
}

#[derive(Debug)]
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw SelectBox {:?}", self);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no"),
                ],
            })
        ]
    };
    screen.run();
}
