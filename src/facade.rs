/// Facade design pattern
/// 1. combine multiple methods' invoking
/// 2. resolve bad performance
/// 3. resolve problem of distributed transaction
struct C {}


impl C {
    fn f1(&self) {
        println!("I am the first step");
    }

    fn f2(&self) {
        println!("I am the second step");
    }

    fn f3(&self) {
        println!("I am the third step");
    }
}


pub trait FacadeTrait {
    fn whole(&self);
}

pub struct Facade {
    c: C
}

impl Facade {
    pub fn new() -> Self {
        let c = C{};
        Self {
            c
        }
    }
}

impl FacadeTrait for Facade {
    fn whole(&self) {
        self.c.f1();
        self.c.f2();
        self.c.f3();
    }
}
