
pub struct A {}
pub struct B {}

pub struct Proxy{}

pub trait ProxyTrait<Request> {
    fn invoke(&self, t: Request);
}


pub trait Request {
    fn process(self);
}

impl Request for A {
    fn process(self) {
        println!("A process done!");
    }
}

impl Request for B {
    fn process(self) {
        println!("B process done!");
    }
}

impl<T: Request> ProxyTrait<T> for Proxy {
    fn invoke(&self, request: T) {
        println!("{}", "Doing something like auth or log");
        request.process();
    }
}

impl Proxy {
    pub fn new() -> Self {
        Self{}
    }
}

impl A {
    pub fn new() -> Self {
        Self{}
    }
}

impl B {
    pub fn new() -> Self {
        Self{}
    }
}
// impl<T: Request> ProxyTrait<T> for Proxy {
//     fn invoke(t: T) {
//         t.process();
//     }
// }
