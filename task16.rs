// Реализовать бинарный поиск встроенными методами языка.
// +

fn main() {
    let arr = [1, 3, 5, 7, 9, 11, 13];

    match arr.binary_search(&3) {
        Ok(index) => println!("Индекс элемента: {}", index),
        Err(_) => println!("Элемент не найден"),
    }
}
