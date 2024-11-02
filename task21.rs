// Разработать программу, которая перемножает, делит, складывает, вычитает две числовых переменных a,b, значение которых > 2^20.
// +
fn main() {
    let a: i64 = 2i64.pow(21); // a = 2^21
    let b: i64 = 2i64.pow(22); // b = 2^22

    // Выполняем арифметические операции
    let sum = a + b;
    let difference = a - b;
    let multiply = a * b;
    let divide = a / b;

    // Выводим результаты
    println!("Складываем: {}", sum);
    println!("Вычитаем: {}", difference);
    println!("Умножаем: {}", multiply);
    println!("Делим: {}", divide);
}
