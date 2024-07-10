/// Decorator design pattern [structure]

pub trait IA {
    fn f(self);
}

struct D {}

impl D {
    fn new() -> Self {
        Self{}
    }
}

impl IA for D {
    fn f(self) {
        println!("I am D's f function");
    }
}


pub struct DDecorator {
    d: D,
}

impl DDecorator {
    pub fn new() -> Self {
        let d = D::new();
        Self {
            d
        }
    }
}


impl IA for DDecorator {
    fn f(self) {
        println!("Enhance functionality before");
        self.d.f();
        println!("Enhance functionality after");
    }
}
