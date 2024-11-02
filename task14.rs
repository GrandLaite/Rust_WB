// Подготовьте пример программы, которая в рантайме способна определить тип переменной, используйте std::any::*.
// +
use std::any::{Any, TypeId};

fn print_type<T: Any>(_value: &T) {
    let type_id = TypeId::of::<T>();

    if type_id == TypeId::of::<i32>() {
        println!("Это тип i32!");
    } else if type_id == TypeId::of::<f64>() {
        println!("Это тип f64!");
    } else if type_id == TypeId::of::<String>() {
        println!("Это тип String!");
    } else {
        println!("Какой-то неизвестный тип!");
    }
}

fn main() {
    let test_int = 42;
    let test_float = 3.14;
    let test_string = String::from("Hello");

    print_type(&test_int);
    print_type(&test_float);
    print_type(&test_string);
}
