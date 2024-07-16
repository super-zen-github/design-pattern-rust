/// Chain of responsibility design pattern

use std::option::Option;

pub trait Handler<'a> {
    // type Next;
    fn set_next(&mut self, next: &'a Self);
    fn get_next(&self) -> Option<&Self>;
    fn do_handle(&self) -> bool;
    fn handle(&self) {
        let handled = self.do_handle();
        if handled {
            match self.get_next() {
                Some(next) => next.handle(),
                None => println!("End of chain"),
            }
        } else {
            println!("Stop chain");
        }
    }
}

pub struct ConcreteHandler<'a> {
    next: Option<&'a Self>,
    filter: String,
}

impl<'a> ConcreteHandler<'a> {
    pub fn new(filter: &str) -> Self {
        Self {
            next: None,
            filter: filter.into(),
        }
    }
}

impl<'a> Handler<'a> for ConcreteHandler<'a> {
    fn set_next(&mut self, next: &'a ConcreteHandler) {
        self.next = Some(next);
    }
    fn get_next(&self) -> Option<&Self> {
        self.next
    }
    fn do_handle(&self) -> bool {
        println!("Handler {} doing", self.filter);
        true
    }
}
