mod singleton;

fn main() {
    println!("Hello, world!");

    let sc = singleton::get_instance();
    sc.print();
}
