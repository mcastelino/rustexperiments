// Why is this a bad pattern
// vs the idiomatic https://doc.rust-lang.org/1.11.0/book/method-syntax.html#builder-pattern
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Circle {
    fn new() -> Circle {
        Circle {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        }
    }

    fn x(&mut self, coordinate: f64) -> &mut Circle {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut Circle {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut Circle {
        self.radius = radius;
        self
    }

    fn finalize(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
        }
    }
}

fn main() {
    let mut c = Circle::new().x(1.0).y(2.0).radius(2.0).finalize();

    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);

    c.radius(10.0);
    println!("area: {}", c.area());
}
