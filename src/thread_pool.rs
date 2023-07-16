use std::{sync::{Arc, mpsc::{Receiver, Sender, channel}, Mutex}, thread};

use crate::resource_sys::system::System;



pub struct ThreadPool {
    sender: Sender<Box<dyn System + Send>>
}

impl ThreadPool {
    fn new(pool_size: usize) -> ThreadPool {

        let (sender, recv) = channel::<Box<dyn System + Send>>();

        let recv = Arc::new(Mutex::new(recv));

        for i in 0..pool_size {

            let recv = recv.clone();

            thread::spawn( move || {
                let recv = recv.lock().unwrap();

                let val = recv.recv().unwrap();

                drop(recv);

                println!("WORKER {} RECEIVED JOB", i);

                // val.call_system(args)
            });

        }

        ThreadPool {
            sender
        }

    }
}
 