fn main() {
    trait HasArea {
        fn area(&self) -> f64;
    }

    #[derive(Debug)]
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl HasArea for Circle {
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
        fn area(&self) -> f64 {
            self.x * self.y
        }
    }

    fn print_area<T: HasArea + std::fmt::Debug>(shape: T) {
        println!("This shape {:?} has an area of {}", shape, shape.area());
    }

    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 100.0,
    };

    let s = Square { x: 10.0, y: 20.0 };

    print_area(c);
    print_area(s);
}
