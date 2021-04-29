use std::thread;
use std::time::Duration;

fn main() {
    let timeout = 1;
    // Thread messages
    let messages = vec!["I'm the first thread!", "I'm the second thread!"];

    // Thread vector, tracking all created threads
    let mut handles = Vec::new();

    for message in messages {
        // Each thread is going to own its message
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(timeout));
            println!("{}", message);
        });

        // Add the created thread to the thread vector.
        handles.push(handle);
    }

    // Now wait for all threads to terminate.
    for handle in handles {
        handle.join().unwrap();
    }
}
