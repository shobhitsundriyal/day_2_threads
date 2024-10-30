use std::{ collections::VecDeque, fmt::format, sync::Mutex, time::Duration};
use once_cell::sync::Lazy;
// VecDeques are a double-ended queue, which means that you can push and pop from both the front and the back of the queue. This is useful for a work queue because you can push work onto the back of the queue and pop work off the front of the queue.

static WORK_QUEUE: Lazy<Mutex<VecDeque<String>>> = Lazy::new(|| Mutex::new(VecDeque::new())); // Lazy take a function
fn main() {
    let cpu_count = num_cpus::get();// rust dont have a built-in function to get the number of CPUs on the system, so we use the num_cpus crate
    println!("Number of CPUs: {}", cpu_count);

    let mut threads = Vec::with_capacity(cpu_count);
    let mut broadcast = Vec::with_capacity(cpu_count);

    for cpu in 0..cpu_count {
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        broadcast.push(tx); //push sender to broadcast vector

        let thread = std::thread::spawn(move || {
            while rx.recv().is_ok() {
                let mut lock = WORK_QUEUE.lock().unwrap(); // lock the WORK_QUEUE
                if let Some(work) = lock.pop_front() {
                    // pop_front() removes the first element from the queue and returns it, cpu would start working on the work
                    std::mem::drop(lock);
                    println!("CPU{}: {}", cpu, work);
                    std::thread::sleep(Duration::from_secs(2));// sleep for 2 second so that we can see the output
                    println!("CPU{cpu}: done");
                } else {
                    println!("CPU{} found no work", cpu);
                }
            }
        });
        threads.push(thread);
    }

    loop {
        let sent = {
            let mut lock = WORK_QUEUE.lock().unwrap();
            let len = lock.len();
            println!("There are {len} items in the queue");
            if len < 5 {
                lock.push_back(format!("Hello {len}").to_string());
                true
            } else {
                false
            }
        };
        if sent {
            broadcast.iter().for_each(|tx| tx.send(()).unwrap());
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
