/// Template method design pattern [action]

pub trait Template {
    fn a(&self);
    fn b(&self);
    fn c(&self) {
        self.a();
        self.b();
    }
}

pub struct Instance {}

impl Instance {
    pub fn new() -> Self {
        Self {}
    }
}

impl Template for Instance {
    fn a(&self) {
        println!("I am fn a");
    }


    fn b(&self) {
        println!("I am fn b");
    }
}
