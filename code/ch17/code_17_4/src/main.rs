fn main() {
    
}

pub trait Draw {
    fn draw(&self);
}


pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();            
        }
    }

    pub fn add(&mut self, d: dyn Draw + 'static) {
        self.components.push(Box::new(d));
    }
}


