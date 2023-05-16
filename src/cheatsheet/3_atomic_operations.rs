use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};

fn main() {
    let spinlock = Arc::new(AtomicUsize::new(1));
    let mut handles = vec![];

    for i in 0..10 {
        let spinlock_clone = Arc::clone(&spinlock);
        handles.push(thread::spawn(move ||
            {
                println!("thread {} before increment", i);
                let old_thread_count = spinlock_clone.fetch_add(1, Ordering::SeqCst);
                println!("thread {} after increment, old count {}", i, old_thread_count);
                old_thread_count+1
            }
        ));
    }

    // Wait for the other threads to release the lock
    while spinlock.load(Ordering::SeqCst) != 11 {
        hint::spin_loop();
    }
}
