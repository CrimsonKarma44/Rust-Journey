pub mod threaded {
    use std::thread;
    use std::time::Duration;

    pub fn bland_threading() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn wait_threading() {
        let handler = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        handler.join().unwrap();
    }

    pub fn move_closure() {
        let v = vec!(1, 2, 3);

        let handler = thread::spawn(move || {
            println!("Here's a vector: {v:?}");
        });

        handler.join().unwrap();
    }

    // Multiple Producers Single Consumer (MPSC)
    use std::sync::mpsc;
    
    pub fn channel() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let value = String::from("Hi");
            tx.send(value).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }

    pub fn channel_ownershp_transference() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }
    }
    pub fn channel_mpsc() {
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
            println!("Got: {received}");
        }
    }

    // Mutex: mutual exclusion
    pub mod concurrency_shared_state {
        use std::sync::{Mutex, Arc};
        use std::thread;
        
        pub fn basic() {
            let m = Mutex::new(5);

            {
                let mut num = m.lock().unwrap();
                *num = 6;
            }

            println!("m = {m:?}");
        }

        // Atomic reference counting
        pub fn shared_across_threads() {
            let counter = Arc::new(Mutex::new(0));
            let mut handles = vec![];

            for _ in 0..10 {
                let counter = Arc::clone(&counter);

                let handle = thread::spawn(move || {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                    println!("{}", *num);
                });
                
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            println!("Result: {}", *counter.lock().unwrap());
        }
    }
}
