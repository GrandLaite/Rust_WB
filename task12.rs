// Реализовать пересечение двух неупорядоченных множеств.
// +

use std::collections::HashSet;

fn main() {
    let set1: HashSet<i32> = [55, 41, 14, 95, 52].iter().cloned().collect();
    let set2: HashSet<i32> = [30, 41, 55, 6, 95].iter().cloned().collect();

    let cross: HashSet<_> = set1.intersection(&set2).cloned().collect();

    println!("Пересечение множеств: {:?}", cross);
}
