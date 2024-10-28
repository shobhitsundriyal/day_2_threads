static mut COUNTER: u32 = 0; // shared mutable state

fn main() {
    let mut thread_handles = Vec::new();

    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_000 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        thread_handles.push(handle);
    }

    thread_handles.into_iter().for_each(|h| h. join().unwrap());

    println!("Result: {}", unsafe { COUNTER });
    // 1000 threads * 1000 increments = 1_000_000 this should be the answer
    /*
     result is alweys less than 1_000_000, also it is not deterministic
     because all 1000 threads are trying to increment the counter at the same time
     there are 3 steps in the increment operation
        1. read the value of the counter
        2. add 1 to the value
        3. write the new value back to the counter memory location
    if two threads read the value at the same time, both will increment the value and write it back
    the second thread will overwrite the value written by the first thread
    COUNTER = 0
    t1 reads COUNTER = 0
    t2 reads COUNTER = 0
    t1 writes COUNTER = 1
    t2 writes COUNTER = 1
    we lost one increment, this may happen multiple times, also might not happen at all
    that's why the result is less than 1_000_000
    
    unsafe is used to tell the compiler that we are taking the responsibility of the safety of the code
    but dont use it unless you are sure that the code is safe, its not as simple as just ignoring types in ts
    */

    /*
     we can use atomic types to solve this problem
     atomic types are types that can be safely shared between threads
     a thread wont read a value that is being written by another thread
    */
}
