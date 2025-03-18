use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum MainMessage { Incr, Get, Quit }

enum SpawnMessage { Get(usize) }


fn main() {
    // make a channel going in and a channel going out
    let (spawn_tx, main_rx) = mpsc::channel();
    let (main_tx, spawn_rx) = mpsc::channel();
    // Arc for shared ownership and Mutex for interior mutability
    let counter = Arc::new(Mutex::new(0));
    let thread_counter = Arc::clone(&counter);
    
    let spawn = thread::spawn(move || {
        // move keyword in closure captures ownership of thread_counter
        loop {
            match spawn_rx.recv().unwrap() {
                MainMessage::Quit => break,
                MainMessage::Incr => {
                    *thread_counter.lock().unwrap() += 1
                }
                MainMessage::Get => {
                    spawn_tx.send(
                        SpawnMessage::Get(
                            *thread_counter.lock().unwrap()
                        )
                    ).unwrap()
                }
            }
        }
    });
    
    let send_messages = [MainMessage::Incr, 
                         MainMessage::Incr,
                         MainMessage::Get, 
                         MainMessage::Quit];

    for msg in send_messages {
        main_tx.send(msg).unwrap();
    }
    
    spawn.join().unwrap(); // make a race condition by putting this last

    // use destructuring assignment to unpack c
    let SpawnMessage::Get(c) = main_rx.recv().unwrap();
    println!("All messages received: {}", *counter.lock().unwrap() == c);
    
}
