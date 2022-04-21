use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let timeout = 1;
    // Thread messages
    let messages = Arc::new(vec!["First message", "Second message"]);

    // Thread vector, tracking all created threads
    let mut handles = Vec::new();

    for tid in 0..5 {
        let messages = Arc::clone(&messages);
        // Each thread owns an atomically counted reference on mesages
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(timeout));
            for message in messages.as_ref().into_iter() {
                println!("tid {} - message: {}", tid, message);
            }
        });

        // Add the created thread to the thread vector.
        handles.push(handle);
    }

    // Now wait for all threads to terminate.
    for handle in handles {
        handle.join().unwrap();
    }
}
