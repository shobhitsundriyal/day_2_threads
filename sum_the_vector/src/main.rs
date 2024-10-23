use std::time::Instant;

fn main() {
    let start = Instant::now();
    const N_THREADS: usize = 8;

    let sum_this_vector: Vec<u32> = (0..90001).collect();

    let mut thread_handles = Vec::new();
    let chunks = sum_this_vector.chunks(N_THREADS);

    for chunk in chunks {
        let my_chunk = chunk.to_owned();

        thread_handles.push(std::thread::spawn(move || {
            return my_chunk.iter().sum::<u32>();
        }));
    }

    let mut total_sum: u32 = 0;

    for handle in thread_handles {
        total_sum += handle.join().unwrap();
    }

    println!("Total sum: {}", total_sum);
    println!("Time elapsed: {:?}", start.elapsed());

    // single threaded
    let start = Instant::now();
    let sum_this_vector: Vec<u32> = (0..90001).collect();
    let total_sum: u32 = sum_this_vector.iter().sum();
    println!("Total sum: {}", total_sum);
    println!("Time elapsed: {:?}", start.elapsed());

    //for 10000 elements, single thread is faster than multi-thread 38.5ms vs 158microseconds
    //for 90000 elements, single thread is faster than multi-thread 255ms vs 1.4ms
    // my theory most  likely due to the overhead of creating threads and context switching

}
