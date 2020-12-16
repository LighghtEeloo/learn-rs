use crate::polymorphism::poly::*;

pub fn poly_main() {
    let mut vec: Vec<Box<dyn Entity>> = Vec::new();
    vec.push(Box::new(House {
        address: vec!(String::from("High Street")),
        size: 144,
    }));
    vec.push(Box::new(Student {
        name: String::from("Lighght"),
        age: 20,
        mass: 60.0,
    }));
    for elem in vec {
        println!("{}", elem.refer());
    }
}
