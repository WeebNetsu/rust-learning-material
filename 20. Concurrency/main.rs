// below is how to import multiple items from std
use std::{thread, time::Duration, sync::mpsc, sync::Mutex};

// NOTE: all output in this tutorial might be slightly different
// than the output on the users computer (due to how threads are)
fn main() {
    // there are 2 types of threads
    // 1-to-1 (native/system) threads
    // read (user/program/green) threads 
    // rust only has 1-to-1 threads, to keep runtime low
    // user-threads are available by using crates that provide it

    // video part 1 and 2 are split by sections

    // ---- PART 1 (SECTION 1) ----
    // ---- THREADS ----
    
    // create threads
    /* thread::spawn(|| {
        // here we're creating 10 threads
        for i in 1..10 {
            println!("hi from thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hello from main thread ({})", i);
        thread::sleep(Duration::from_millis(1));
    } 
    
    println!("DONE!"); */

    // You'll notice that not all the threads gets compeleted
    // that is because if the main thread ends, all threads are
    // stopped and the program ends

    // ---- PART 2 ----
    // this will allow us to create a handler to handle our threads
    /* let handle = thread::spawn(|| {
        // here we're creating 10 threads
        for i in 1..10 {
            println!("hi from thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hello from main thread ({})", i);
        thread::sleep(Duration::from_millis(1));
    }

    // this will block the current thread (main thread) from
    // executing until the threads we created ends
    handle.join().unwrap();

    println!("DONE!"); */

    // ---- PART 3 ----
    /* let x = [9, 10, 5];

    /* let handle = thread::spawn(|| {
        // below will cause an error, because Rust does
        // not know if x is still valid when we try to access it
        // since we might delete it in the main thread
        println!("Here is an array: {:?}", x);
    }); */

    let handle = thread::spawn(move || {
        // here we're moving x into this thread
        // x is now no longer available in the main thread
        // but it can now be used in this thread
        println!("Here is an array: {:?}", x);
    });

    // this will block the current thread (main thread) from
    // executing until the threads we created ends
    handle.join().unwrap();

    println!("DONE!"); */

    // ---- PART 4 (SECTION 2) ----
    // ---- CHANNELS ----
    
    // mpsc = Multi-producer, single-consumer
    // mpsc is a channel that allows multiple threads to send messages

    // create a channel
    // creates a channel that returns a sender (tx) and a receiver (rx)
    /* let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let message = String::from("Hello World!");
        tx.send(message).unwrap();
        // note: message is no longer available in this thread
        // we moved ownership of message to the thread we sent it to 
    });

    // recv will block the current thread (main thread) from
    // executing until a message is received
    // you can use try_recv to check if a message is available
    // if not, it will return an error, try_recv will not block
    // the current thread (main thread)
    let received_message = rx.recv().unwrap();

    // in this example, we just shared data between the thread we
    // created and the main thread
    println!("MESSAGE: {}", received_message);

    println!("DONE!"); */

    // ---- PART 5 ----
    /* let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // here we'll be sending multiple messages
        let messages = [
            String::from("Hello World!"),
            String::from("Hello Rust!"),
            String::from("Hello Mpsc!"),
        ];

        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // here we're no longer using recv, instead we're
    // using rx as an iterator
    for received in rx {
        println!("MESSAGE: {}", received);
    }

    println!("DONE!"); */

    // ---- PART 6 ----
    /* let (tx, rx) = mpsc::channel();
    // if we want to send messages from multiple threads
    // we need to use tx, but tx can only be used once
    // since it is moved to be owned by the thread spawner
    // so we create a 2nd tx that is "cloned" from tx
    let tx2 = tx.clone();

    thread::spawn(move || {
        let messages = [
            String::from("Hello World!"),
            String::from("Hello Rust!"),
            String::from("Hello Mpsc!"),
        ];

        for msg in messages.iter() {
            tx.send(msg.clone()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // NOTE: this will send messages at basically the
    // same time as the thread we created above
    thread::spawn(move || {
        let messages = [
            String::from("This is"),
            String::from("my custom"),
            String::from("cool"),
            String::from("2nd thread!"),
        ];

        for msg in messages.iter() {
            // here we use tx2 instead of tx
            tx2.send(msg.clone()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("MESSAGE: {}", received);
    }

    println!("DONE!"); */
}