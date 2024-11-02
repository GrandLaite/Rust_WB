// Удалить i-ый элемент из вектора.
// +
fn main() {
    let mut vec = vec![10, 20, 30, 40, 50];

    let i = 2;

    if i < vec.len() {
        vec.remove(i);
    } else {
        println!("Индекс выходит за границы вектора");
    }

    println!("Вектор после удаления: {:?}", vec);
}
