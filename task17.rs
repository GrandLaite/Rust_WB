// Реализовать структуру-счетчик, которая будет инкрементироваться в конкурентной среде. По завершению программа должна выводить итоговое значение счетчика.
// +
use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    count: Mutex<i32>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: Mutex::new(0),
        }
    }

    fn increment(&self) {
        let mut num = self.count.lock().unwrap();
        *num += 1;
    }

    fn get_value(&self) -> i32 {
        let num = self.count.lock().unwrap();
        *num
    }
}

fn main() {
    let counter = Arc::new(Counter::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Итоговое значение счетчика: {}", counter.get_value());
}
