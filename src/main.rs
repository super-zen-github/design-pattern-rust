mod singleton;
mod factory;
mod builder;
mod prototype;

use builder::SomePoolBuilder;
use factory::FactoryClass;
use prototype::ComplicatedEntity;

fn main() {
    println!("Hello, world!");

    let sc = singleton::get_instance();
    sc.print();


    let fact = FactoryClass::new();
    let target = fact.create_complicated_instance();
    target.print();


    let name = String::from("new_config");
    let max_total = 100;
    let max_idle = 10;
    let min_idle = 0;
    let builder = SomePoolBuilder::default();
    // should handle error instead of using unwrap
    let pool = builder.set_name(name).unwrap().set_max_total(max_total).unwrap().set_max_idle(max_idle).unwrap().set_min_idle(min_idle).unwrap().build().unwrap();
    pool.print();


    let c1 = ComplicatedEntity::new();
    c1.print();
    let mut c2 = c1.clone();
    c2.update_x(10);
    c2.print();
}
