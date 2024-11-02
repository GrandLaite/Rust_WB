// Дан массив чисел (инициализировать его 1..N). Используя параллельные вычисления, найти сумму квадратов этих чисел и вывести в stdout. Использовать только стандартную библиотеку и модули thread и mpsc.
// +

use std::sync::mpsc;
use std::thread;

fn main() {
    let n = 500;
    let numbers: Vec<u32> = (1..=n).collect();

    let (tx, rx) = mpsc::channel();

    for &num in &numbers {
        let tx = tx.clone();
        thread::spawn(move || {
            let square = num * num;
            tx.send(square).expect("Не удалось отправить через канал");
        });
    }

    drop(tx);

    let sum: u32 = rx.iter().sum();

    println!("Сумма квадратов составляет: {}", sum);
}
