// Сделать трейт Action с методом say, который должен выводить сообщение в консоль.Сделать структуру Person, которая содержит строковое имя.Сделать структуру Person, которая содержит строковое имя.
// +

trait Action {
    fn say(&self);
}

struct Person {
    name: String,
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let person = Person {
        name: String::from("Danil"),
    };

    person.say();
}
