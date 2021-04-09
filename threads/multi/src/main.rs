use std::thread;
use std::time::Duration;

fn main() {
    let messages = vec!["I'm the first thread!", "I'm the second thread!"];
    let mut handles = Vec::new();

    for message in messages {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            println!("{}", message);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
