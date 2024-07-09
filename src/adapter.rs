/// Adapter design pattern
/// 1. resolve the bad design of interface
/// 2. unify the interface design of multiple classes
/// 3. replace the dependent outside system
/// 4. compatible with old version of interfaces
/// 5. compatible with different types of data
pub trait TargetTrait {
    fn f1(&self);
    fn f2(&self);
}


struct OriginStruct {}

impl OriginStruct {
    fn ff1(&self) {
        println!("I am function ff1 of origin struct");
    }

    fn ff2(&self) {
        println!("I am function ff2 of origin struct");
    }
}

pub struct Adapter {
    o: OriginStruct
}

impl Adapter {
    pub fn new() -> Self {
        let o = OriginStruct{};
        Self{o}
    }
}

impl TargetTrait for Adapter {
    fn f1(&self) {
        self.o.ff1();
    }

    fn f2(&self) {
        self.o.ff2();
    }
}
