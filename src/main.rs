mod simple;
mod batch;
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

enum Mode {
    Perform,
    _Pass,
    Mute,
}
use Mode::*;

struct Wrapper<T> {
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
    pub fn using(&self, mode: Mode) {
        match mode {
            Mode::Perform => {
                println!(">>> {} <<<", self.name);
                (self.op)();
                println!("^^^ {} ^^^\n", self.name);
            }
            Mode::_Pass => {
                println!("xxx {} xxx\n", self.name);
            }
            Mode::Mute => ()
        }
    }
}

fn main() {
    println!();
    Wrapper::new("Simple", simple::main).using(Mute);
    Wrapper::new("Petersburg", petersburg::main).using(Mute);
    Wrapper::new("Cacher", cacher::main).using(Mute);
    Wrapper::new("Polymorphism", polymorphism::main).using(Mute);
    Wrapper::new("Arc", arc::main).using(Mute);
    Wrapper::new("SpawnThread", thread::main).using(Mute);
    Wrapper::new("JsonLess", jsonless::main).using(Mute);
    Wrapper::new("Batch", batch::main).using(Perform);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all() {
        main()
    }
}
