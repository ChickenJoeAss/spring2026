use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define termination signal
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create channel
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut producer_handles = vec![];
    let mut consumer_handles = vec![];

    // TODO: Create 2 producer threads
    for i in 0..2 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(i, tx_clone, ITEM_COUNT / 2);
        });
        producer_handles.push(handle);
    }

    // TODO: Create 3 consumer threads
    for i in 0..3 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(i, rx_clone);
        });
        consumer_handles.push(handle);
    }

    // wait for producers
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // send termination signals
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // wait for consumers
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// TODO: Implement producer
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    for i in 0..item_count {
        let num = (id as i32) * 100 + i as i32; // simple numbers
        println!("Producer {} produced {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(200));
    }
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let num = rx.lock().unwrap().recv().unwrap();

        if num == TERMINATION_SIGNAL {
            println!("Consumer {} stopping", id);
            break;
        }

        println!("Consumer {} received {}", id, num);
    }
}