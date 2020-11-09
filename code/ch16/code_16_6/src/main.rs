use std::sync::mpsc;
fn main() {
    let (t, r) = mpsc::channel();

    t.send("hhhh").unwrap();
    let s = r.recv().unwrap();
    println!("Got: {}", s);
}
