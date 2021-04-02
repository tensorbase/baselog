use criterion::{black_box, criterion_group, criterion_main, Criterion};
use log::*;
use baselog::*;
use std::thread;
use std::fs::File;
use std::time::Instant;

const NTHREADS: u32 = 4;

// Generates log messages
fn log(iter: u64, thread: u32) {
    let thread = thread.to_string();
    for n in 0..100 {
       info!("iter: {}, Thread: {}, item: {}", iter, thread, n); 
    }

}

#[allow(unused_must_use)]
pub fn criterion_benchmark(c: &mut Criterion) {
    // Setup global logger
    WriteLogger::init(LevelFilter::Info, Config::default(), File::create("log/arrayqueue.log").unwrap());

    c.bench_function("arrayqueue", move |b| {
        b.iter_custom(|iters| {

            // Start the bench:
            // Currently including the thread spawing overhead
            // and we don't clear the logs each iter.
            let start = Instant::now();

            for i in 0..iters {
            
                let mut handles = vec![];

                for t in 0..NTHREADS {
                    // create the thread and generate some log entries
                    let handle = thread::spawn(move || {
                       black_box(log(i, t))
                    });

                    // push the handle into the vec
                    handles.push(handle);
                }

                // join the handles in the vector
                for i in handles {
                    i.join().unwrap();
                }
            }           

            start.elapsed()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
