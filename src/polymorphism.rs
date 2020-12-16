pub struct Student {
    pub name: String,
    pub age: u32,
    pub mass: f64,
}

pub struct House {
    pub address: Vec<String>,
    pub size: u32,
}

pub trait Entity {
    fn refer(&self) -> String;
}

impl Entity for Student {
    fn refer(&self) -> String {
        format!("{}, age {}, weight {}kg", self.name, self.age, self.mass)
    }
}

impl Entity for House {
    fn refer(&self) -> String {
        format!("{:?} with {} m^2", self.address, self.size)
    }
}
