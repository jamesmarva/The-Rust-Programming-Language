
pub trait Draw{
    fn draw(&self);
}

#[derive(Debug)]
struct Button {
    value: String,
    width: u32,
    height: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("value: {}", self.value);
    }
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T> 
    where T: Draw,
{
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }

    pub fn addDraw(&mut self, draw: T) {
        self.components.push(draw);
    }
}


fn main() {
    

}
