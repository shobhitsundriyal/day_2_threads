use std::sync::Mutex;

static MY_SHARED_NUMBER: Mutex<i32> = Mutex::new(5);

fn poisioner() {
    let mut num = MY_SHARED_NUMBER.lock().unwrap();
    *num += 1;
    panic!("poisioner panicked");
    // thread would lock the mutex and panic, not releasing the lock
}
fn main() {
    let hanlde = std::thread::spawn(poisioner); // this thread would panic without releasing the lock
    let result = hanlde.join();
    println!("{:?}", result);

    // try to lock the mutex from main thread
    let lock = MY_SHARED_NUMBER.lock();
    println!("lock from main thread{:?}", lock);

    // we can get the data from the mutex if we really need it
    let recoverd_data = lock.unwrap_or_else(|poisoned| {
        return poisoned.into_inner(); // this would return the data from the mutex even if it is poisoned(lock not released)
    });
    println!("recoverd data from mutex: {:?}", recoverd_data);
}
// rust calls this "Poisoning" the mutex