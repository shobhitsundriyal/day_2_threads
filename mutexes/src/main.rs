use std::sync::Mutex;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut handles = Vec::new();

    for _ in 0..10 {

        let handle = std::thread::spawn(|| {
            let mut numbers = NUMBERS.lock().unwrap();
            numbers.push(5);
            // no need release the lock, it will be released when the guard goes out of scope
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{:#?}", NUMBERS.lock().unwrap());
}

// to access mutex data, we need to lock it first
// when we lock a mutex, we get a guard object that will release the lock when it goes out of scope