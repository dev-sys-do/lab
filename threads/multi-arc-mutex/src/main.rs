use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let timeout = 1;

    // Thread messages
    let messages = Arc::new(Mutex::new(vec![
        String::from("I'm the first thread!"),
        String::from("I'm the second thread!"),
    ]));

    println!("Initial Messages:");
    for (i, m) in messages.lock().unwrap().iter().enumerate() {
        println!("\tMessage #{}: [{}]", i, m);
    }

    // Thread vector, tracking all created threads
    let mut handles = Vec::new();

    for i in 0..messages.lock().unwrap().len() {
        let messages = Arc::clone(&messages);

        // Each thread is owning a reference counted mutex of the messages vector
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(timeout));

            // Let's lock and modify the messages
            let mut locked_messages = messages.lock().unwrap();
            locked_messages[i] = format!("I'm thread number {}!", i);
        });

        // Add the created thread to the thread vector.
        handles.push(handle);
    }

    // Now wait for all threads to terminate.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Messages:");
    for (i, m) in messages.lock().unwrap().iter().enumerate() {
        println!("\tMessage #{}: [{}]", i, m);
    }
}
