// Реализовать несколько способов остановки thread’ов и tokio task’ов: закрытие канала, tokio_util::CancellationToken
// +

// Для thread
use flume::unbounded;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = unbounded(); // Flume канал для SPMC

    // Создаем несколько потоков, которые будут читать из канала
    let handles: Vec<_> = (0..5)
        .map(|id| {
            let rx = rx.clone(); // Клонируем получателя для каждого потока
            thread::spawn(move || {
                while let Ok(data) = rx.recv() {
                    println!("Поток {} получил: {}", id, data);
                    thread::sleep(Duration::from_millis(50));
                }
                println!("Поток {} завершает работу", id);
            })
        })
        .collect();

    // Главный поток отправляет данные
    for i in 0..10 {
        tx.send(i).unwrap();
        thread::sleep(Duration::from_millis(200));
    }

    // Закрываем канал
    drop(tx);

    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Программа завершена.");
}

// Закрытие tokio task
use tokio::task;
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let token_child = token.child_token();

    // Запуск нескольких задач с использованием tokio::task
    let tasks: Vec<_> = (0..5)
        .map(|id| {
            let token = token_child.clone();
            task::spawn(async move {
                loop {
                    tokio::select! {
                        _ = token.cancelled() => {
                            println!("Задача {} завершает работу", id);
                            break;
                        }
                        _ = sleep(Duration::from_millis(200)) => {
                            println!("Задача {} выполняется", id);
                        }
                    }
                }
            })
        })
        .collect();

    // Основной поток ждет 1 секунду, затем инициирует отмену
    sleep(Duration::from_secs(1)).await;
    token.cancel();

    // Ожидание завершения всех задач
    for task in tasks {
        let _ = task.await;
    }

    println!("Программа завершена.");
}
