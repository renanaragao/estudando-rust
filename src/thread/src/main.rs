use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    loop_thread();
    closure_thread();
    channel_thread();
}

fn loop_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn closure_thread() {
    let v = Arc::new(vec![1, 2, 3]);
    let v_clone = v.clone();

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v_clone);
    });

    println!("Here's a vector: {:?}", v);

    handle.join().unwrap();
}

fn channel_thread() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}
