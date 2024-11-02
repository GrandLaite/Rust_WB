// Дана переменная int64. Разработать программу которая устанавливает i-й бит в 1 или 0.
// +
fn one_bit(num: i64, pos: usize) -> i64 {
    num | (1 << pos)
}

fn zero_bit(num: i64, pos: usize) -> i64 {
    num & !(1 << pos)
}

fn main() {
    let num: i64 = 0b1011; // число
    let bitpos = 2; // позиция бита

    let new_num = one_bit(num, bitpos);
    println!(
        "Число после установки {}-го бита в 1: {:b}",
        bitpos, new_num
    );

    let new_num = zero_bit(num, bitpos);
    println!(
        "Число после установки {}-го бита в 0: {:b}",
        bitpos, new_num
    );
}
