mod singleton;
mod factory;
mod builder;
mod prototype;
mod proxy;
mod bridge;
mod decorator;
mod adapter;

use builder::SomePoolBuilder;
use factory::FactoryClass;
use prototype::ComplicatedEntity;
use proxy::*;
use bridge::*;
use decorator::*;
use adapter::*;

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

    let proxy = Proxy::new();
    let q_a = A::new();
    let q_b = B::new();
    proxy.invoke(q_a);
    proxy.invoke(q_b);

    let telephone_msg_sender = TelephoneMsgSender::new(vec![String::from("1234567890")]);
    let email_msg_sender = EmailMsgSender::new(vec![String::from("aaa@bbb.com")]);

    let s_notification = ServereNotification::new(&telephone_msg_sender);
    let n_notification = NormalNotification::new(&email_msg_sender);
    s_notification.notify(String::from("emergency issue"));
    n_notification.notify(String::from("normal issue"));


    let d_d = DDecorator::new();
    d_d.f();


    let adpt = Adapter::new();
    adpt.f1();
    adpt.f2();
    
}
