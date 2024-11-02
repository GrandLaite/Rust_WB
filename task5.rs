// Программа должна аккуратно завершаться по нажатию Ctrl+C. Выбрать и обосновать способ завершения работы всех воркеров. Использовать tokio и flume (или другую аналогичную библиотеку для spmc/mpmc-каналов).
// +

use futures::future::join_all;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> io::Result<()> {
    print!("Введите количество воркеров: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let workers: usize = input.trim().parse().expect("Введите корректное число");

    let (tx, rx) = flume::unbounded();
    let rx = Arc::new(rx);

    let shutdown_flag = Arc::new(AtomicBool::new(false));

    let mut handles = vec![];
    for id in 0..workers {
        let worker_rx = Arc::clone(&rx);
        let worker_shutdown_flag = Arc::clone(&shutdown_flag);
        let handle = tokio::spawn(async move {
            loop {
                if worker_shutdown_flag.load(Ordering::Relaxed) {
                    println!("Воркер {} завершает работу", id);
                    break;
                }

                match worker_rx.recv_async().await {
                    Ok(data) => {
                        println!("Воркер {} получил данные: {}", id, data);
                        sleep(Duration::from_millis(10)).await;
                    }
                    Err(_) => break,
                }
            }
        });
        handles.push(handle);
    }

    let sender_shutdown_flag = Arc::clone(&shutdown_flag);
    let sender_handle = tokio::spawn(async move {
        let mut counter = 0;
        loop {
            if sender_shutdown_flag.load(Ordering::Relaxed) {
                break;
            }
            let data = format!("Данные {}", counter);
            if tx.send_async(data).await.is_err() {
                break;
            }
            counter += 1;
            sleep(Duration::from_millis(100)).await;
        }
    });

    signal::ctrl_c()
        .await
        .expect("Не удалось установить обработчик Ctrl+C");
    println!("Получен сигнал завершения, закрываем программу...");

    shutdown_flag.store(true, Ordering::Relaxed);

    join_all(handles).await;

    sender_handle.await.unwrap();

    println!("Все воркеры завершили работу. Программа завершается.");
    Ok(())
}
