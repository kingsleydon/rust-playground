trait AreaCalculator {
    fn area(&self) -> f64;
}

fn calculate_area<T: AreaCalculator>(shape: T) {
    println!("The area is: {}", shape.area());
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

impl AreaCalculator for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl AreaCalculator for Triangle {
    fn area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }
}

impl AreaCalculator for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let circle = Circle { radius: 2.0 };
    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };
    let square = Square { side: 5.0 };

    calculate_area(circle);

    calculate_area(triangle);

    calculate_area(square);
}
