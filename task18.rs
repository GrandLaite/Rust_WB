// Разработать программу, которая переворачивает подаваемую на ход строку (например: «главрыба — абырвалг»). Символы могут быть unicode.
// +
fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let input = "главрыба";
    let reversed = reverse(input);
    println!("{}", reversed);
}
