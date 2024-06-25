#[derive(Clone)]
pub struct ComplicatedEntity {
    x: i32,
    y: String,
    z: Vec<String>
}


impl ComplicatedEntity {
    pub fn new() -> Self {
        // really complicated procedures to create an instance of ComplicatedEntity
        let x = 15;
        let y = String::from("Large set of data from database");
        let z1 = String::from("data from io 1");
        let z2 = String::from("data from io 2");
        let z = vec![z1, z2];
        Self {
            x,
            y,
            z,
        }
    }

     pub fn print(&self) {
        println!("ComplicatedEntity[x: {}, y: {}, z: {:?}]", self.x, self.y, self.z);
    }

    pub fn update_x(&mut self, x: i32) {
        self.x = x;
    }
}
