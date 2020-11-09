use std::thread;

fn main() {

    let vec = vec![1, 3, 4, 5, 6];
    let handle = thread::spawn(move|| {
        println!("vec is {:?}", vec);
    });
    drop(vec);

    // let h2 = thread::spawn(move|| {
    //     println!("vec ii{:?}", vec);
    // });

    handle.join().unwrap();
}
