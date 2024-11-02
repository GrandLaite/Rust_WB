// Разработать программу, которая переворачивает слова в строке. Пример: «snow dog sun — sun dog snow»..
// +
fn reverse(input: &str) -> String {
    input
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    let word = "snow dog sun";
    let reversed = reverse(word);
    println!("{}", reversed);
}
