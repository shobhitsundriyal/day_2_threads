use std::time::Instant;
use std::thread;

fn main() {
    let start = Instant::now();
    const N_THREADS: usize = 8;

    let sum_this_vector: Vec<u32> = (0..90001).collect();
    let chunks = sum_this_vector.chunks(N_THREADS);

    
    let total_sum = thread::scope(|s| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            // let my_chunk = chunk.to_owned();// no need to clone, coz scope has access to this chunk
            
            thread_handles.push(s.spawn(move || {
                return chunk.iter().sum::<u32>();
            }));
        }
        thread_handles.into_iter()
            .map(|i| i.join().unwrap())
            .sum::<u32>();
    });
    // thread is created in a scope, it is much more managable and we directly get to result out of the scope which can be assigned to a variable
    println!("Total sum: {:?}", total_sum);
    println!("{:?}", start.elapsed())

}
