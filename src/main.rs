mod singleton;
mod factory;
mod builder;
mod prototype;
mod proxy;
mod bridge;
mod decorator;
mod adapter;
mod facade;
mod composite;
mod flyweight;
mod observer;
mod template;
mod strategy;
mod chain;

use std::rc::Rc;

use builder::SomePoolBuilder;
use factory::FactoryClass;
use prototype::ComplicatedEntity;
use proxy::*;
use bridge::*;
use decorator::*;
use adapter::*;
use facade::*;
use composite::*;
use flyweight::*;
use observer::*;
use template::*;
use strategy::*;
use chain::*;

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


    let fac = Facade::new();
    fac.whole();

    let mut directory = Directory::new(String::from("/test/"));
    directory.get_path();
    let file1 = File::new(String::from("/test/a.txt"));
    file1.get_path();
    let file2 = File::new(String::from("/test/b.txt"));
    file2.get_path();
    directory.add_sub_node(&file1);
    directory.add_sub_node(&file2);
    let nof = directory.number_of_files();
    println!("Number of files in directory: {nof}");

    
    let chess_base_factory = ChessBaseFactory::new();
    let unit1 = ChessUnit::new(Rc::new(chess_base_factory.get_base(&1).to_owned()), 1, 0);
    let unit2 = ChessUnit::new(Rc::new(chess_base_factory.get_base(&3).to_owned()), 0, 1);
    let mut chess_board = ChessBoard::new();
    chess_board.add_unit(1, unit1);
    chess_board.add_unit(2, unit2);
    chess_board.print();

    let mut subject = ConcreteSubject::new();
    let observer = ConcreteObserver::new();
    subject.register_observer(&observer);
    subject.notify_observers("this is a notification");


    let i = Instance::new();
    i.c();


    let strategy_factory = DiscountStrategyFactory::new();
    let order = Order::new(String::from("NORMAL"));
    let strategy = strategy_factory.get_strategy(order.get_type());
    strategy.cal_discount();


    let mut handler_a = ConcreteHandler::new("handler_a");
    let mut handler_b = ConcreteHandler::new("handler_b");
    let handler_c = ConcreteHandler::new("handler_c");
    handler_b.set_next(&handler_c);
    handler_a.set_next(&handler_b);
    handler_a.handle();
}
