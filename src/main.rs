/*
 * dtop: A tool for measuring system utilization of applications
 *       and system performance.
 *
 * SPDX-License-Identifier: GPL-2.0
 *
 * Author: Calinyara <jiedeng@alumni.sjtu.edu.cn>
 */

use clap::{App, Arg};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let matches = App::new("cyclictest")
        .arg(Arg::with_name("THREAD_COUNT")
            .help("Number of threads to run")
            .required(true)
            .index(1))
        .arg(Arg::with_name("SLEEP_DURATION_MS")
            .help("Duration each thread should sleep for (in ms)")
            .required(true)
            .index(2))
        .arg(Arg::with_name("ITERATIONS")
            .help("Number of test iterations")
            .required(true)
            .index(3))
        .get_matches();

    let thread_count: usize = matches.value_of("THREAD_COUNT").unwrap().parse().expect("Invalid thread count");
    let sleep_duration_ms: u64 = matches.value_of("SLEEP_DURATION_MS").unwrap().parse().expect("Invalid sleep duration");
    let iterations: usize = matches.value_of("ITERATIONS").unwrap().parse().expect("Invalid iteration count");

    run_test(thread_count, sleep_duration_ms, iterations);
}

fn run_test(thread_count: usize, sleep_duration_ms: u64, iterations: usize) {
    let sleep_duration = Duration::from_millis(sleep_duration_ms);

    let mut max_latency_global = Duration::new(0, 0);
    let mut total_latency_global = Duration::new(0, 0);

    for _ in 0..iterations {
        let (max_latency, total_latency) = run_iteration(thread_count, sleep_duration);
        if max_latency > max_latency_global {
            max_latency_global = max_latency;
        }
        total_latency_global += total_latency;
    }

    let average_latency_global = total_latency_global / (thread_count * iterations) as u32;

    println!("Max latency over all iterations: {:?}", max_latency_global);
    println!("Average latency over all iterations: {:?}", average_latency_global);
}

fn run_iteration(thread_count: usize, sleep_duration: Duration) -> (Duration, Duration) {
    let handles: Vec<_> = (0..thread_count).map(|_| {
        thread::spawn(move || {
            let start_time = Instant::now();
            thread::sleep(sleep_duration);
            start_time.elapsed() - sleep_duration
        })
    }).collect();

    let mut max_latency = Duration::new(0, 0);
    let mut total_latency = Duration::new(0, 0);
    for handle in handles {
        let latency = handle.join().expect("Thread panicked");
        if latency > max_latency {
            max_latency = latency;
        }
        total_latency += latency;
    }
    (max_latency, total_latency)
}

