use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>; // kinda pointer to a function

fn hi_there() {
    println!("Hi there!");
}

fn main() {
    let (channel_sender, channel_receiver) = mpsc::channel::<Job>();

    let thread_handle = std::thread::spawn(move || {
        while let Ok(job_fn) = channel_receiver.recv() {
            job_fn(); // excute the job(function)
        }
    });

    let job1 = || println!("hello from job1");
    let job2 = || {
        for i in 0..5 {
            println!("hello from job2: {}", i);
        }
    };
    
    channel_sender.send(Box::new(hi_there)).unwrap();
    channel_sender.send(Box::new(job1)).unwrap();
    channel_sender.send(Box::new(job2)).unwrap();
    // box is a smart pointer that points to heap memory, which points to function in this case
    // wrapping the function in a box so we get pointer to the function, that can be executed with () inside the thread

    thread_handle.join().unwrap();
}
