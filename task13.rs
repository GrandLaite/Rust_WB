// На вход, через стандартный поток ввода, поступает последовательность строк, строки могут повторяться. Необходимо вывести в стандартный поток вывода строки, исключив повторения, не используя std::collections::*.
// +
// После ввода последовательности строк и нажатия Enter нажимаем Ctrl + Z. Если вводим последовательность get post delete post get delete, то в итоге получим get post delete.
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut unique = Vec::new();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            for word in line.split_whitespace() {
                if !unique.contains(&word.to_string()) {
                    unique.push(word.to_string());
                    print!("{} ", word);
                }
            }
        }
    }
    println!();
}
