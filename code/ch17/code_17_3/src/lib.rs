pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();           
        }
    }

    pub fn add(&mut self, d: Draw) {
        self.components.push(d);

    }
}