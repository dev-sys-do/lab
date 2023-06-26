use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (sender, receiver) = channel();

    // Clone the sender for having multiple producers
    let sender1 = sender.clone();

    thread::spawn(move || {
        sender.send("T1: Hello!").unwrap();
        sender.send("T1: World!").unwrap();
    });

    thread::spawn(move || {
        sender1.send("T2: Hello!").unwrap();
        sender1.send("T2: World!").unwrap();
    });

    for received in receiver {
        println!("Received: {}", received);
    }
}
