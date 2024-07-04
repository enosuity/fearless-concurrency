#![allow(unused)]



use std::{sync::{Arc, Mutex, RwLock}, thread::{sleep, spawn}, time::{Duration, Instant}};
#[cfg(feature = "mutex_vs_rwlock")]

const NUM_THREADS: usize = 10;

const NUM_ITERATIONS: usize = 1_000_000;



fn mutex_benchmarking(no_of_threads: usize) -> Duration{
    let now_mutex = Instant::now();
    let counter = Arc::new(Mutex::new(0));
    let mut list = vec![];
    
    for i in 0..no_of_threads {
        let counter_clone = Arc::clone(&counter);
        let handle = spawn(move || {
            for j in 0..NUM_ITERATIONS {
                let mut counter = counter_clone.lock().unwrap();
                *counter += 1;
            }
        });
        list.push(handle);
    }

    for hnd in list {
        hnd.join().unwrap();
    }

    let val = counter.lock().unwrap();
    println!("total (in mutex){}", val);

    let t =  now_mutex.elapsed();
    println!("mutex took time : {:?}", t);
    t
}

fn rwlock_benchmarking(no_of_threads: usize) -> Duration {
    let now_rwlock = Instant::now();
    let counter = Arc::new(RwLock::new(0));
    let mut list = vec![];
    
    for i in 0..no_of_threads {
        let counter_clone = Arc::clone(&counter);
        let handle = spawn(move || {
            for j in 0..NUM_ITERATIONS {
                let mut counter = counter_clone.write().unwrap();
                *counter += 1;
            }
        });
        list.push(handle);
    }

    for hnd in list {
        hnd.join().unwrap();
    }

    let val = counter.write().unwrap();
    println!("total (in rwlock){}", val);
    let t = now_rwlock.elapsed();
    println!("rwlock took time : {:?}", t);
    t

}
pub fn mutex_vs_rwlock_benchmarking(){
    let mut_duration = mutex_benchmarking(10);
    let rw_duration = rwlock_benchmarking(10);
    if rw_duration.as_secs_f32() < mut_duration.as_secs_f32() {
        println!("Rwlock has higher speed.");
    } else {
        println!("Mutex has higher speed.")
    }

}