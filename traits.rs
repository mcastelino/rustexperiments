fn main() {
    trait HasArea {
        fn new() -> Self;
        fn area(&self) -> f64;
    }

    #[derive(Debug)]
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl HasArea for Circle {
        fn new() -> Circle {
            Circle {
                x: 0f64,
                y: 0f64,
                radius: 10f64,
            }
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    #[derive(Debug)]
    struct Square {
        x: f64,
        y: f64,
    }

    impl HasArea for Square {
        fn new() -> Square {
            Square { x: 10f64, y: 10f64 }
        }
        fn area(&self) -> f64 {
            self.x * self.y
        }
    }

    fn print_area<T: HasArea + std::fmt::Debug>(shape: T) {
        println!("This shape {:?} has an area of {}", shape, shape.area());
    }

    let c = Circle::new();
    let s = Square::new();

    print_area(c);
    print_area(s);
}
