use std::vec::Vec;

fn hello_thread(n: i32) {
    println!("Hello, from thread {}!", n);
}

fn double_number_n_times(number: u32, times: u32) -> u32  {
    let mut result = number;
    for _ in 0..times {
        result *= 2;
    }
    return result;
}

fn main() {
    println!("Hello, from main thread!");

    let mut thread_handles = Vec::new();

    for i in 0..5 {
        let thread_handle = std::thread::spawn(move || double_number_n_times(i, 5));
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|handle| 
        println!("{}", handle.join().unwrap())
    );

}
