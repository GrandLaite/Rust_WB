// Реализовать постоянную запись данных в канал (главный поток). Реализовать набор из N воркеров, которые читают произвольные данные из канала и выводят в stdout. Необходима возможность выбора количества воркеров при старте.
// +
use std::io::{self, Write};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    print!("Введите количество воркеров: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let workers: usize = input.trim().parse().expect("Введите корректное число");

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];
    for id in 0..workers {
        let worker_rx = Arc::clone(&rx);
        let handle = thread::spawn(move || loop {
            let received = {
                let receiver = worker_rx.lock().unwrap();
                receiver.recv()
            };

            match received {
                Ok(data) => {
                    println!("Воркер {} получил данные: {}", id, data);
                    thread::sleep(Duration::from_millis(10));
                }
                Err(_) => {
                    println!("Воркер {} завершает работу", id);
                    break;
                }
            }
        });
        handles.push(handle);
    }

    let mut counter = 0;
    loop {
        let data = format!("Данные {}", counter);
        if tx.send(data).is_err() {
            break;
        }
        counter += 1;
        thread::sleep(Duration::from_millis(100));
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Все воркеры завершили работу. Программа завершается.");
    Ok(())
}
