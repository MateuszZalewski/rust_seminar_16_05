use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            1,
            2,
            3,
            4,
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            101,
            102,
            103,
            104,
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mut total_received: u32 = 0;
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    total_received += received;

    println!("total received: {}", total_received);
}
