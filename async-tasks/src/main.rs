use std::time::Duration;
use std::future::Future;
use std::pin::{Pin, pin};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    
    println!("Using trpl::spawn_task and handle with await to complete");
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        // assign spawn_task to handle and await here to interleave
        handle.await.unwrap();
    });
    
    println!("Using trpl::join to await two anonymous futures");
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };
        
        let fut2 = async {
            for i in 1..5 {
                println!("hello number {i} from the second task!");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };
        
        trpl::join(fut1, fut2).await;
    });
    
    println!("Using multiple producers with async blocks");
    trpl::run(async {
        /* This is a fundamental tradeoff: we can either deal with a 
         * dynamic number of futures with join_all, as long as they all 
         * have the same type, or we can deal with a set number of 
         * futures with the join functions or the join! macro, even if 
         * they have different types. This is the same as working with 
         * any other types in Rust, though. Futures are not special.
         */
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            /* Pin is a wrapper type in the standard library that 
             * ensures that the value it wraps cannot be moved in memory 
             * after it has been pinned. It is primarily used for 
             * working with self-referential types and asynchronous 
             * programming.
             */
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });
        /* making this vec work requires a trait object! 
         * we also must implement Pin<&mut dyn Future<Output = ()>> to 
         * wrap the future trait objects in, avoiding a heap allocation.
         */
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];
        // get all futures joined using join_all and an iterable
        trpl::join_all(futures).await;
    });
    
    println!("Make one future yield to another");
    trpl::run(async {
        let slow = async {
            println!("slow has started");
            trpl::yield_now().await;
            println!("slow has finished");
        };
        let fast = async {
            println!("fast has started");
            println!("fast has finished");
            trpl::yield_now().await;
        };
        // use race function, ignore return value   
        trpl::race(slow, fast).await;
    });
    
    println!("Use a stream made from an iterator");
    trpl::run(async {
        let values = [1,2,3,4,5,6,7,8,9,10];
        let iter = values.map(|n| n*2);
        let stream = trpl::stream_from_iter(iter);
        
        let mut filtered = 
            stream.filter(|value| value % 3 == 0 || value % 5 == 0);
        // streams are async implementations of iterators, uses StreamExts
        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
    
    println!("\
Implement a ReceiverStream which converts an async channel receiver \
into a Stream");
    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval #{count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);
        
        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    /* To sleep between messages in the get_messages function without 
     * blocking, we need to use async. However, we cannot make 
     * get_messages itself into an async function, because then we would 
     * return a Future<Output = Stream<Item = String>> instead of just a 
     * Stream<Item = String>>. The caller would have to await 
     * get_messages itself to get access to the stream. But remember: 
     * everything in a given future happens linearly; concurrency 
     * happens between futures. Awaiting get_messages would require it 
     * to send all the messages, including sleeping between sending each 
     * message, before returning the receiver stream. As a result, the 
     * timeout would end up useless. There would be no delays in the 
     * stream itself: the delays would all happen before the stream was 
     * even available. Instead, we leave get_messages as a regular 
     * function which returns a stream, and spawn a task to handle 
     * the async sleep calls.
     */
    let (tx, rx) = trpl::channel();
    
    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;
            
            if let Err(send_error) = 
                tx.send(format!("Message: '{message}'")) {
                    eprintln!("Cannot send message `{message}`: {send_error}");
                    break;
                }
        }
    });
    
    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();
    
    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });
    // ReceiverStream is a wrapper for the channel receiver we made here
    // ReceiverStream implements the Stream Trait
    ReceiverStream::new(rx)
}
