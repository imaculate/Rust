use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10
        {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..5
    {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1))
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("Here is a vector {:?}", v);
    });

    // v can no longer be used since it was moved
    handle2.join().unwrap();

    let (tx, rx) = mpsc::channel();
    
    /*thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("Sent {}", val); // doesn't compile since val has been moved
    });

    // recv blocks but try_recv checks for msgs now, should be called in a loop
    let msg = rx.recv().unwrap();
    println!("Got : {}", msg);*/

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || 
    {
        let v = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")];
        for msg in v
        {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || 
    {
        let v = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you")];
        for msg in v
        {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for r in rx
    {
        println!("Got: {}", r);
    }

    // shared state
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10
    {
        let cm = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num_ref = cm.lock().unwrap();
            *num_ref += 1;
        });
        handles.push(handle);
    }

    for h in handles
    {
        h.join().unwrap();
    }

    println!("Final result in mutex {}", counter.lock().unwrap());
}
