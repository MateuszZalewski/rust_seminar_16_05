use std::thread;
use std::sync::{Arc, Mutex};


fn main() {
    let mut handles = vec![];
    let counter = Arc::new(Mutex::new(0));
    for i in 0..10 {
        let counter_shared = Arc::clone(&counter);
        handles.push(thread::spawn(move ||
            {
                println!("thread {} before increment", i);

                let mut counter = counter_shared.lock().unwrap();
                *counter += 1;
                println!("thread {} after increment", i);
                *counter
            }
        ));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} result {}", i, result);
    }
}
