use std::sync::atomic::AtomicI32;

static COUNTER: AtomicI32 = AtomicI32::new(0); // I am thinking it like js object, its not mutable but its value is mutable, also simple add operations are not allowed on atomic types, there are methods for that

fn main() {
    let mut thread_handles = Vec::new();

    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_000 {
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                // ordering is relaxed because we dont care about the order of the operations
                // there are other orderings as well
                // Relaxed: No guarantees are made about the order of operations or memory visibility.
                // sometimes we may want some ordering guarantees, but here it does not matter if thread1 increments before thread2 or vice versa as long as the they both do it synchronously
            }
        });
        thread_handles.push(handle);
    }

    thread_handles.into_iter().for_each(|h| h. join().unwrap());

    println!("Result: {}",  COUNTER.load(std::sync::atomic::Ordering::Relaxed));
    // now we are using load method to read the value of the counter, it would give us the correct counter val
    // Result: 1_000_000

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
    */
}
