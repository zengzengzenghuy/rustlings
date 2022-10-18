// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.
// This program should wait until all the spawned threads have finished before exiting.

// Reference: https://nickymeuleman.netlify.app/blog/multithreading-rust

use std::thread;
use std::time::Duration;
use thread::JoinHandle;

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        let a = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
        });
        handles.push(a); // push thread into handles vec
    }

    let mut completed_threads = 0;
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        handle.join().unwrap(); // join every handle
        completed_threads += 1;
    }

    if completed_threads != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }
}
