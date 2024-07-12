/// Observer design pattern [action]
/// or Publish-Subscribe design pattern


pub trait Subject<'a> {
    fn register_observer(&mut self, observer: &'a dyn Observer);
    fn notify_observers(&self, msg: &str);
}

pub trait Observer {
    fn update(&self, msg: &str);
}

pub struct ConcreteSubject<'a> {
    observers: Vec<&'a dyn Observer>,
}

impl<'a> ConcreteSubject<'a> {
    pub fn new() -> Self {
        let observers: Vec<&'a dyn Observer> = Vec::new();
        Self { observers }
    }
}

impl<'a> Subject<'a> for ConcreteSubject<'a> {
    fn register_observer(&mut self, observer: &'a dyn Observer) {
        self.observers.push(observer);
    }


    fn notify_observers(&self, msg: &str) {
        for observer in self.observers.iter() {
            observer.update(msg);
        }
    }
}

pub struct ConcreteObserver {}
impl ConcreteObserver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Observer for ConcreteObserver {
    fn update(&self, msg: &str) {
        println!("received message: {}", msg);
    }
}
