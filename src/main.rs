mod cacher;
mod polymorphism;
mod arc;

use crate::cacher::cache_main::cache_main;
use crate::polymorphism::poly_main::poly_main;

pub struct Wrapper<T> 
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
    Wrapper::new("Cacher", cache_main).using(false);
    Wrapper::new("Polymorphism", poly_main).using(false);
    Wrapper::new("Arc", arc::arc_main).using(true);
}
