/// Strategy design pattern [action]
use std::collections::HashMap;

pub trait DiscountStrategy {
    fn cal_discount(&self);
}

pub struct NormalDiscount {}
impl NormalDiscount {
    pub fn new() -> Self {
        Self {}
    }
}
impl DiscountStrategy for NormalDiscount {
    fn cal_discount(&self) {
        println!("Normal discount");
    }
}
pub struct PromotionDiscount {}
impl PromotionDiscount {
    pub fn new() -> Self {
        Self {}
    }
}
impl DiscountStrategy for PromotionDiscount {
    fn cal_discount(&self) {
        println!("Promotion discount");
    }
}

pub struct DiscountStrategyFactory {
    strategies: HashMap<String, Box<dyn DiscountStrategy>>
}

impl DiscountStrategyFactory {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        let n_d = Box::new(NormalDiscount::new());
        let p_d = Box::new(PromotionDiscount::new());
        m.insert(String::from("NORMAL"), n_d as Box<dyn DiscountStrategy>);
        m.insert(String::from("PROMOTION"), p_d as Box<dyn DiscountStrategy>);
        Self { strategies: m }
    }

    pub fn get_strategy(&self, order_type: &str) -> &dyn DiscountStrategy {
        self.strategies.get(order_type).unwrap().as_ref()
    }
}

pub struct Order {
    order_type: String
}

impl Order {
    pub fn new(order_type: String) -> Self {
        Self { order_type }
    }

    pub fn get_type(&self) -> &str {
        &self.order_type
    }
}
