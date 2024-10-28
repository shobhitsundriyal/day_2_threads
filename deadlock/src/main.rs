use std::sync::Mutex;
fn main() {
    let shared_resource = Mutex::new(0);
    {
        let lock1 = shared_resource.lock().unwrap();
    }
    let lock2 = shared_resource.lock().unwrap();
}
