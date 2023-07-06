#[derive(Debug)]
struct Circle(u32);

impl Circle {
    fn new(r: u32) -> Self {
        Self(r)
    }

    fn set(&mut self, r: u32) {
        self.0 = r;
    }

    fn square(&self) -> f64 {
        let r = self.0 as f64;

        std::f64::consts::PI * r
    }

    fn clone(&self) -> Self {
        Self(self.0)
    }
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(&self) -> u32 {
        self.width * self.height
    }

    fn set(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
        }
    }
}

fn main() {
    let mut c = Circle::new(15);

    println!("{:?} has square {}", c.clone(), c.square());

    c.set(20);

    dbg!(c);

    let mut r = Rect::new(10, 15);

    println!("{:?} has square {}", r.clone(), r.square());

    r.set(40, 30);

    dbg!(r);
}
