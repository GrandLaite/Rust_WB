// Разработать конвейер чисел с помощью каналов. В первый канал с паузами пишутся числа из массива, проинициализированного 1..N. Первый thread читает из этого канала и пишет квадрат полученного числа во второй канал. Второй thread читает из второго канала и выводит в stdout.
// +
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let n = 10;
    let numbers: Vec<u32> = (1..=n).collect();

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let sender_thread = thread::spawn(move || {
        for num in numbers {
            tx1.send(num).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    let square_thread = thread::spawn(move || {
        while let Ok(num) = rx1.recv() {
            let square = num * num;
            tx2.send(square).unwrap();
        }
    });

    let output_thread = thread::spawn(move || {
        while let Ok(square) = rx2.recv() {
            println!("Квадрат числа: {}", square);
        }
    });

    sender_thread.join().unwrap();
    square_thread.join().unwrap();
    output_thread.join().unwrap();
}
