/// Singleton design pattern [creation]
/// only have one instance

#[derive(Debug, Clone, Copy)]
pub struct SingleClass {
    a_field: usize,
}

static mut SC: Option<SingleClass> = None;

pub fn get_instance() -> SingleClass {
    unsafe {
        match &SC {
            Some(s) => *s,
            None => {
                let s = SingleClass{
                    a_field: 15,
                };
                SC = Some(s);
                return s;
            },
        }
    }
}


impl SingleClass {
    pub fn print(self) {
        print!("{:?}", self.a_field);
    }
}
