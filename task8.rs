// Реализовать конкурентную запись данных в map несколькими способами: Mutex с HashMap, DashMap
// +

use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn mutex_hashmap() {
    let map_mutex = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let map_mutex = Arc::clone(&map_mutex);
        let handle = thread::spawn(move || {
            let mut map = map_mutex.lock().unwrap();
            map.insert(i, i * 10);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let map = map_mutex.lock().unwrap();
    println!("HashMap with Mutex: {:?}", *map);
}

fn dashmap() {
    let dash_map = Arc::new(DashMap::new());
    let mut dash_handles = vec![];

    for i in 0..10 {
        let dash_map = Arc::clone(&dash_map);
        let handle = thread::spawn(move || {
            dash_map.insert(i, i * 10);
        });
        dash_handles.push(handle);
    }

    for handle in dash_handles {
        handle.join().unwrap();
    }

    println!("DashMap: {:?}", dash_map);
}

fn main() {
    mutex_hashmap();
    dashmap();
}
