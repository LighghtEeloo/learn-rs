mod cacher;
mod polymorphism;

use crate::cacher::cache_main::cache_main;
use crate::polymorphism::poly_main::poly_main;

struct Wrapper<T> 
where 
    T: Fn() -> ()
{
    name: &'static str,
    op: T,
}
    
impl<T> Wrapper<T>
where 
    T: Fn() -> ()
{
    pub fn new(name: &'static str, op: T) -> Self {
        Wrapper {
            name,
            op
        }
    }

    pub fn using(&self) {
        println!(">>> {} <<<", self.name);
        (self.op)();
        println!("^^^ {} ^^^\n", self.name);
    }
}

fn main() {
    println!();
    Wrapper::new("Cacher", cache_main).using();
    Wrapper::new("Polymorphism", poly_main).using();
}
