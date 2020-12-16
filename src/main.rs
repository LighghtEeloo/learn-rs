mod cache;
mod cache_main;

mod polymorphism;
mod poly_main;

use crate::cache_main::cache_main;
use poly_main::poly_main;

fn main() {
    cache_main();
    poly_main();
}
