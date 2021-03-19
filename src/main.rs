mod simple;
mod petersburg;
mod cacher;
mod polymorphism;
mod arc;
mod thread;
// mod shared;
mod jsonless;

pub mod utils {
    pub use std::thread;
    pub use std::time::Duration;
    pub use std::sync::Arc;
    pub use rand::Rng;
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
    pub use std::collections::HashMap;
}

pub struct Wrapper<T> {
    name: &'static str,
    op: T,
}
    
impl<T> Wrapper<T> where T: Fn() -> ()
{
    pub fn new(name: &'static str, op: T) -> Self {
        Wrapper {
            name,
            op
        }
    }
    pub fn using(&self, switch: bool) {
        if switch {
            println!(">>> {} <<<", self.name);
            (self.op)();
            println!("^^^ {} ^^^\n", self.name);
        } else {
            println!("xxx {} xxx\n", self.name);
        }
    }
}

fn main() {
    println!();
    // Wrapper::new("Simple", simple::main).using(false);
    // Wrapper::new("Petersburg", petersburg::main).using(false);
    Wrapper::new("Cacher", cacher::main).using(true);
    // Wrapper::new("Polymorphism", polymorphism::main).using(false);
    // Wrapper::new("Arc", arc::main).using(false);
    // Wrapper::new("SpawnThread", thread::main).using(false);
    // Wrapper::new("JsonLess", jsonless::main).using(true);
}
