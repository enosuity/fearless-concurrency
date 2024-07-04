#![allow(unused)]

use core::num;
use std::{
    error::Error, fs::File, io::{self, BufRead, Write}, path::Components, rc::Rc, sync::{
        mpsc::{self, channel},
        Arc, Mutex
    }, thread::{self, sleep, spawn}, time::Duration
};

use singleton_wrapper::*;
use performance_considerations::mutex_vs_rwlock_benchmarking;


use custom_lock_guard::*;


#[derive(Debug)]
struct ArrayWrapper<T, const N: usize> {
    elements: [T; N],
}

impl <T, const N: usize> ArrayWrapper<T, N> {
    fn new(elements: [T; N]) -> Self {
        ArrayWrapper {
            elements
        }
    }

    fn length(&self) -> usize {
        N
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < N {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, val: T) {
        if index < N {
            self.elements[index] = val;
        }
    }
}

fn use_wrapper_array() {
    let mut arr = ArrayWrapper::new([1,5,6,3,2,8,7]);
    let n = arr.length();
    println!("arr: {:?}", arr);

    arr.set(5, 10);
    
    println!("arr: {n}");
    println!("arr: {:?}", arr);
}

fn use_simple_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut threads_list = Vec::new();
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = spawn(move || {
            let mut counter = counter.lock().unwrap();
            println!("i: {}", i);
            sleep(Duration::from_millis(300));
            *counter +=5;
        });
        threads_list.push(handle);        
    }

    for thrd in threads_list {
        thrd.join().unwrap();
    }

    println!("counter => {}", *counter.lock().unwrap());    
}

const PATH: &str = "./";

fn handle_mutex_poison() -> Result<(), Box<dyn Error>>{
    let file_path = format!("{}{}", PATH, "hello.txt");
    let hello_file = File::create(file_path)?;
    
    let file = Arc::new(Mutex::new(hello_file));
    let mut handles = Vec::new();
    
    for i in 0..10 {
        let file_clone = Arc::clone(&file);
        let hnd = spawn(move || {
            let mut file = file_clone.lock().unwrap();
            let s = format!("\ni: {}", i);
            file.write(s.as_bytes());
            sleep(Duration::from_millis(250));
        });
        handles.push(hnd);
    }

    for hnd in handles {
        hnd.join();
    }

    println!("Happy Documented Birthday Goldy Bhai");
    Ok(())

}

struct ThreadSafeStack<T> {
    stack: Mutex<Vec<T>>,
}

impl <T> ThreadSafeStack<T> {
    fn new() -> Self {
        ThreadSafeStack { stack: Mutex::new(Vec::new()) }
    }

    fn push(&self, item: T) {
        let mut stack = self.stack.lock().unwrap();
        stack.push(item);        
    }

    fn pop(&self) -> Option<T> {
        let mut stack = self.stack.lock().unwrap();
        stack.pop()
    }    
}

fn use_thread_safe_stack() {
    let pipes = Arc::new(ThreadSafeStack::new());
    for i in 0..10 {
        let pipes_clone = Arc::clone(&pipes);
        let _ = spawn(move || {
            let mut pipe = pipes_clone.stack.lock().unwrap();
            println!("{i} pushing...");
            pipe.push(i);

            let popped_elem = pipe.pop().unwrap();
            println!("{popped_elem} popped up...");
        }).join();
    }
    println!("pipes : {:?}", pipes.stack.lock().unwrap());   

}



fn main() {
    println!("Hello world!");
    use_of_custom_lock_guard();
    use_of_rw_custom_lock_guard()
    
    // mutex_vs_rwlock_benchmarking();
    // show_case_singleton_wrapper();
    // use_thread_safe_stack();
    // handle_mutex_poison();
    // use_simple_mutex();

    // use_wrapper_array();



    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..34 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle)
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("The counter is: {}", *counter.lock().unwrap() );


    // let (sender, receiver) = channel();
    // let sender1 = sender.clone();
    // type Communication = String;

    // thread::spawn(move || {

    //     let mut list = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     let final_words: Communication = String::from("Nice to meet you");
    //     list.push(final_words);

    //     for val in list {
    //         sender1.send(val).unwrap();
    //         sleep(Duration::from_secs(1));
    //     }
    // });

    // thread::spawn(move || {
    //     let st = String::from("Have a good day!");
    //     let list = vec![
    //         String::from("Goldy"),
    //         String::from("Bhai"),
    //         String::from("zinda"),
    //         String::from("hai"),
    //     ];

    //     for val in list {
    //         sender.send(val).unwrap();
    //         sleep(Duration::from_secs(1));
    //     }
    // });

    // for coming_msg in receiver {
    //    println!("Got: {coming_msg}"); 
    // }

    // println!("\nConnection is closed successfully. Bye"); 

    

}
