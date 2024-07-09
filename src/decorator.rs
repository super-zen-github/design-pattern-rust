pub trait IA {
    fn f(self);
}

pub struct D {}

impl D {
    pub fn new() -> Self {
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
    pub fn new(d: D) -> Self {
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
