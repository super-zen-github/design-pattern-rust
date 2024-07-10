/// Factory design pattern [creation]


pub struct FactoryClass {
}

pub struct ComplicatedClass {
    field: String,
}

impl FactoryClass {
    pub fn new() -> Self {
        FactoryClass{}
    }

    pub fn create_complicated_instance(self) -> ComplicatedClass {
        // some really complicated procedure to create ComplicatedClass instance
        ComplicatedClass{
            field: String::from("test"),
        }
    }
}

impl ComplicatedClass {
    pub fn print(self) {
        println!("{:?}", self.field);
    }
}
