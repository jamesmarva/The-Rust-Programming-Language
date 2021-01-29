fn main() {

    let dog = PetDog;
    // println!("{}", dog.name());

    println!("{}", PetDog::name());
    println!("{}", <PetDog as Animal>::name())
}

trait Animal {
    fn name() ->  String;
}

struct PetDog;

impl PetDog {
    fn name() -> String {
        String::from("moon（明月）")
    }
}

impl Animal for PetDog {
    fn name() -> String{
        String::from("Alaskan Malamute（阿拉斯加雪橇犬）")
    }
}