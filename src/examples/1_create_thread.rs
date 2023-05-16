use std::thread;
use std::time::{Duration, Instant};


fn main() {
    let handle = thread::spawn(||
        {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread done");
            start.elapsed().as_millis()
        }
    );

    let result:u128 = handle.join().unwrap();

    println!("thread took {}ms", result);
}
