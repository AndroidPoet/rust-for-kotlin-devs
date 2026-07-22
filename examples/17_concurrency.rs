//! §19 — Fearless concurrency: Arc + Mutex, and the `move` closure.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc = atomic shared ownership; Mutex = safe interior mutability.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // `move` transfers ownership of `c` into the thread.
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    // If this compiles, it has no data races — that's the borrow checker's payoff.
    println!("final count = {}", *counter.lock().unwrap()); // 10
}
