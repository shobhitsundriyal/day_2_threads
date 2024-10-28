use std::sync::Mutex;

static MY_SHARED_NUMBER: Mutex<i32> = Mutex::new(5);
fn main() {
     
    let lock1 = MY_SHARED_NUMBER.lock().unwrap();

    std::mem::drop(lock1); // manually release the lock1 

    if let Ok(lock2) = MY_SHARED_NUMBER.try_lock() {
        println!("Lock 2 is acquired");
    } else {
        println!("Lock 2 is not acquired");
    }
    // we can try to get lock2 here if lock1 is released, else lock2 will not be acquired
}
