use std::vec::Vec;

fn hello_thread(n: i32) {
    println!("Hello, from thread {}!", n);
}

fn main() {
    println!("Hello, from main thread!");

    // let thread_handle = std::thread::spawn(hello_thread);
    // thread_handle.join().unwrap(); // wait for thread to finish, sometimes it may happen threadd is not finished yet, so we need to wait for it to finish, if we comment this line, then the program might exit before the thread finishes

    // // generaly its good idea to wait for thread to finish, so that we can get the result of the thread, if we don't wait for thread to finish, then we might not get the result of the thread

    let mut thread_handles = Vec::new();

    for i in 0..5 {
        let thread_handle = std::thread::spawn(move || hello_thread(i));
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|handle| handle.join().unwrap()); // wait for all threads to finish, not nesseary it would be in order, it can be in any order, as thread creation is handled by OS

}
