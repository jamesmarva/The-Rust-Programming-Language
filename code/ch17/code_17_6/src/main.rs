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

    pub fn add(&mut self, draw: T) {
        self.components.push(Box::new(draw));
    }
}

#[derive(Debug)]
pub struct Button {
    width: u32,
    height: u32,
    label: String,
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
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::("button1"),
            }),
        ]
    };
    screen.run();
}
