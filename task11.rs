// Дана последовательность температурных колебаний (для примера: -25.4, -27.0 13.0, 19.0, 15.5, 24.5, -21.0, 32.5). Объединить данные значения в интервалы с шагом в 10 градусов. Последовательность в подмножноствах не важна.Пример: [-30,-20):{-25.0, -27.0, -21.0}, [10, 20):{13.0, 19.0, 15.5}, [20, 30): {24.5}, etc.
// +
use std::collections::BTreeMap;

fn group(temperatures: Vec<f64>) -> BTreeMap<(i32, i32), Vec<f64>> {
    let mut grouped: BTreeMap<(i32, i32), Vec<f64>> = BTreeMap::new();

    for &temp in &temperatures {
        let low = ((temp / 10.0).floor() * 10.0) as i32;
        let high = low + 10;

        grouped.entry((low, high)).or_insert(Vec::new()).push(temp);
    }

    grouped
}

fn main() {
    let temperatures = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];

    let result = group(temperatures);

    for (range, temps) in result {
        println!("{:?} -> {:?}", range, temps);
    }
}
