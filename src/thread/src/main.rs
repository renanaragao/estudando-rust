use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    loop_thread();
    closure_thread();
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
