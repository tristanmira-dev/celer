use std::{sync::{Arc, mpsc::{Receiver, Sender, channel}, Mutex}, thread};



pub struct ThreadPool {
    sender: Sender<Box<dyn FnOnce() + Send>>
}

impl ThreadPool {
    fn new(pool_size: usize) -> ThreadPool {

        let (sender, recv) = channel::<Box<dyn FnOnce() + Send>>();

        let recv = Arc::new(Mutex::new(recv));

        for i in 0..pool_size {

            let recv = recv.clone();

            thread::spawn( move || {
                let recv = recv.lock().unwrap();

                let val = recv.recv().unwrap();

                drop(recv);

                println!("WORKER {} RECEIVED JOB", i);

                val()
            });

        }

        ThreadPool {
            sender
        }

    }
}
 