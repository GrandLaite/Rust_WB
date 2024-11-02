// Разработать программу нахождения расстояния между двумя точками, которые представлены в виде структуры Point с инкапсулированными параметрами x,y и конструктором.
// +
use std::f64;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

fn main() {
    let point1 = Point::new(0.0, 0.0);
    let point2 = Point::new(3.0, 4.0);

    let distance = point1.distance(&point2);

    println!("Расстояние между точками: {}", distance);
}
