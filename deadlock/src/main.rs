use std::sync::Mutex;
fn main() {
    let shared_resource = Mutex::new(0);

    let lock1 = shared_resource.lock().unwrap();
    let lock2 = shared_resource.lock().unwrap();
    // This will cause a deadlock, as lock1 is already locked and the thread will wait for lock1 to be released
    // while holding lock2. Progam will be stuck here.
}
