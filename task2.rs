// Написать программу, которая параллельно рассчитает квадраты чисел, взятых из массива (массив инициализировать 1..N), и выведет их в stdout. Числа могут быть выведены в произвольном порядке.Использовать только стандартную библиотеку.
// +

use std::sync::mpsc;
use std::thread;

fn main() {
    let n = 10;
    let numbers: Vec<u32> = (1..=n).collect();

    let (tx, rx) = mpsc::channel();

    for &num in &numbers {
        let tx = tx.clone();
        thread::spawn(move || {
            let square = num * num;
            tx.send((num, square))
                .expect("Не удалось отправить через канал");
        });
    }

    drop(tx);

    for received in rx {
        println!("Квадрат числа {}: {}", received.0, received.1);
    }
}
