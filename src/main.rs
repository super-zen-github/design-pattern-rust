mod singleton;
mod factory;

use factory::FactoryClass;

fn main() {
    println!("Hello, world!");

    let sc = singleton::get_instance();
    sc.print();


    let fact = FactoryClass::new();
    let target = fact.create_complicated_instance();
    target.print();
}
