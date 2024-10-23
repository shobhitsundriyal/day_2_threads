fn hello_thread() {
    println!("Hello, from thread!");
}

fn main() {
    println!("Hello, from main thread!");

    let thread_handle = std::thread::spawn(hello_thread);
    thread_handle.join().unwrap(); // wait for thread to finish, sometimes it may happen threadd is not finished yet, so we need to wait for it to finish, if we comment this line, then the program might exit before the thread finishes

    // generaly its good idea to wait for thread to finish, so that we can get the result of the thread, if we don't wait for thread to finish, then we might not get the result of the thread
}
