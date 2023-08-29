use std::{sync::mpsc, thread, time::Duration};

fn main() {
    println!("*****BEGIN OF MAIN BLOCK*****");

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..=5 {
            println!("hi number {i} from SPAWNED thread, v: {:?}", v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=3 {
        println!("hi number {i} from MAIN thread");
        thread::sleep(Duration::from_millis(1));
    }

    println!("-----END OF MAIN BLOCK----");
    handle.join().unwrap();

    println!("*****BEGIN OF MAIN BLOCK 2 : 16.2*****");

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi tx1"),
            String::from("from tx1"),
            String::from("the tx1"),
            String::from("thread tx1"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more tx"),
            String::from("messages tx"),
            String::from("for tx"),
            String::from("you tx"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    println!("-----END OF MAIN BLOCK 2----");
}
