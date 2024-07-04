#![allow(unused)]

use std::{fmt::Debug, ops::{Deref, DerefMut}, sync::{Arc, Mutex, MutexGuard, RwLock, RwLockWriteGuard}};

struct CustomLock<T: Debug> {
    data: Arc<Mutex<T>>,
}

impl<T: Debug> CustomLock<T> {
    fn new(val: T) -> Self {
        CustomLock { data: Arc::new(Mutex::new(val)) }
    }

    fn lock(&self) -> CustomLockGuard<T> {
        let guard = self.data.lock().unwrap();
        CustomLockGuard { 
            data: Some(guard),
            custom_lock: Arc::clone(&self.data)
        }

    }
    
}

struct CustomLockGuard<'a, T: Debug> {
    data: Option<MutexGuard<'a, T>>,
    custom_lock: Arc<Mutex<T>>
}

impl<T: Debug> Drop for CustomLockGuard<'_, T> 
where T: Debug
{
    fn drop(&mut self) {
        println!("CustomLockGuard dropping...");
    }    
}

impl<T> Deref for CustomLockGuard<'_, T> 
where T: Debug
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
       self.data.as_ref().unwrap() 
    }    
}

impl <T> DerefMut for CustomLockGuard<'_, T> 
where T: Debug
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.as_mut().unwrap()    
    }
    
}

#[cfg(feature = "lock-guard")]
pub fn use_of_custom_lock_guard() {
    use std::{thread::spawn, time::Instant};

    let time = Instant::now();
    let custom_lock = Arc::new(CustomLock::new(100));

    for i in 0..10 {
        let custom_lock_clone = Arc::clone(&custom_lock);
        spawn(move || {
            let mut custom_guard = custom_lock_clone.lock();
            *custom_guard += 1 ;
            
            // println!("custom_guard : {:?}", *custom_guard);

        }).join().unwrap();
    }

    let final_lock = custom_lock.lock();
    println!("final_lock : {:?}", *final_lock);

    println!("total spent time: {:?}", time.elapsed());

    println!("\nBye Guard!");
}



// by using RwLock


struct RwCustomLock<T: Debug> {
    data: Arc<RwLock<T>>,
}

impl<T: Debug> RwCustomLock<T> {
    fn new(val: T) -> Self {
        RwCustomLock { data: Arc::new(RwLock::new(val)) }
    }

    fn lock(&self) -> RwCustomLockGuard<T> {
        let guard = self.data.write().unwrap();
        RwCustomLockGuard { 
            data: Some(guard),
            custom_lock: Arc::clone(&self.data)
        }

    }
    
}

struct RwCustomLockGuard<'a, T: Debug> {
    data: Option<RwLockWriteGuard<'a, T>>,
    custom_lock: Arc<RwLock<T>>
}

impl<T: Debug> Drop for RwCustomLockGuard<'_, T> 
where T: Debug
{
    fn drop(&mut self) {
        println!("RwCustomLockGuard dropping...");
    }    
}

impl<T> Deref for RwCustomLockGuard<'_, T> 
where T: Debug
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
       self.data.as_ref().unwrap() 
    }    
}

impl <T> DerefMut for RwCustomLockGuard<'_, T> 
where T: Debug
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.as_mut().unwrap()    
    }
    
}

#[cfg(feature = "lock-guard")]
pub fn use_of_rw_custom_lock_guard() {
    use std::{thread::spawn, time::Instant};

    let time = Instant::now();
    let custom_lock = Arc::new(RwCustomLock::new(100));

    for i in 0..10 {
        let rw_custom_lock_clone = Arc::clone(&custom_lock);
        spawn(move || {
            let mut rw_custom_guard = rw_custom_lock_clone.lock();
            *rw_custom_guard += 1 ;
            
            println!("custom_guard : {:?}", *rw_custom_guard);

        }).join().unwrap();
    }

    let final_lock = custom_lock.lock();
    println!("final_rw_lock : {:?}", *final_lock);
    println!("Rwlock total spent time: {:?}", time.elapsed());

    println!("\nBye RwGuard!");
}