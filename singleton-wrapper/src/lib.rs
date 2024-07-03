#![allow(unused)]

use std::{fmt::Display, sync::{Arc, Mutex, Once}, thread::spawn};

#[cfg(feature = "singleton")]

#[derive(Debug, Clone, Copy)]
struct Singleton<T> {
    value: T,
}

impl<T> Singleton<T> 
where
    T: Copy + Clone
{
    fn set(&mut self, val: T) {
        self.value = val;
    }

    fn show(&self) -> T {
        self.value
        
    }
}
#[derive(Debug)]
struct SingletonWrapper<T> {
    instance: Arc<Mutex<Option<Singleton<T>>>>,
    init: Once,
    init_val: T,
}

impl <T> SingletonWrapper<T>
where T: Clone + Copy + Display + std::fmt::Debug
{
    fn new(init_val: T) -> Self {
        SingletonWrapper {
            instance: Arc::new(Mutex::new(None)),
            init: Once::new(),
            init_val
        }
    }

    fn get_instance(&self) -> Arc<Mutex<Singleton<T>>> {
        let inst = Arc::clone(&self.instance);

        self.init.call_once(|| {
            let mut inst_clone = inst.lock().unwrap();
            *inst_clone = Some(Singleton { value: self.init_val.clone() })
        }); 

        let x = Arc::new(Mutex::new(inst.lock().unwrap().as_ref().unwrap().clone()));
        println!("x: {:?}", x);
        x
    }    
}


pub fn show_case_singleton_wrapper() {
    let singleton_wrapper = Arc::new(SingletonWrapper::new(100));
    
    for i in 0..10 {
        let singleton_wrapper_dup = Arc::clone(&singleton_wrapper);
        
        let handle = spawn( move || {
            let instanse = singleton_wrapper_dup.get_instance();
            
            let mut obj = instanse.lock().unwrap();
            let mut new_val: i32 = obj.show();
            
            new_val = new_val.checked_add(i).unwrap();

            obj.set(new_val);       

            println!("Thread {} set value to {}", i, obj.show());
        }).join().unwrap();

    }  

    
    let instance = singleton_wrapper.get_instance();
    let singleton = instance.lock().unwrap();
    println!("Final singleton value : {}", singleton.show());

    
    
}


