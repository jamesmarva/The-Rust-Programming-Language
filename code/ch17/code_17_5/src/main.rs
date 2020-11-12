


pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run (&self) {
        for c in self.components.iter() {
            println!("待写");
            c.draw();        
        }
    }
}



fn main() {

    

}
