
pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
pub struct SelectBox {
    width: u32,
    height: u32, 
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("select :{:?}", self);
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
        println!("Button : {:?}", self);
    }
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>
}

impl Screen{
    fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

fn main() {


    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
