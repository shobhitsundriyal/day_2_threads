use std::thread;

fn hello_thread() {
    println!("Hello from a thread!, my name is {}", thread::current().name().unwrap());
}
fn main() {
    let thread = thread::Builder::new()
                    .name("Paplo".to_string())
                    .stack_size(size_of::<usize>() * 4)// by default stack size is 2MB, I thinks that's why it was failing for very large chunks of vector
                    .spawn(hello_thread)
                    .unwrap();
    thread.join().unwrap();
}
