// Разработать программу, которая будет последовательно отправлять значения в канал, а с другой стороны канала — читать. По истечению N секунд программа должна завершаться.
// +

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let n = 5;

    let (tx, rx) = mpsc::channel();

    let sender = thread::spawn(move || {
        let mut counter = 0;
        loop {
            counter += 1;
            if tx.send(counter).is_err() {
                break;
            }
            thread::sleep(Duration::from_millis(500));
        }
    });

    let start = Instant::now();
    while start.elapsed().as_secs() < n {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(value) => println!("Получено: {}", value),
            Err(_) => {
                println!("Таймаут при чтении из канала.");
                break;
            }
        }
    }

    sender.join().unwrap();

    println!("Программа завершена.");
}
