// Разработать программу, которая проверяет, что все символы в строке уникальные (true — если уникальные, false etc). Функция проверки должна быть регистронезависимой. Например: abcd — true abCdefAaf — false aabcd — false
// +
fn un_symbol(input: &str) -> bool {
    let mut checked = Vec::new();

    for c in input.to_lowercase().chars() {
        if checked.contains(&c) {
            return false;
        }
        checked.push(c);
    }

    true
}

fn main() {
    let input = "aabcd";
    let result = un_symbol(input);
    println!("{} — {}", input, result);
}
