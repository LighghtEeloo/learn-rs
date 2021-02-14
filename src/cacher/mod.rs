pub mod cache;

use cache::Cacher;

use std::thread;
use std::time::Duration;
use rand::Rng;


fn calculation(num: u32) -> u32 {
    thread::sleep(Duration::from_secs(2));
    println!("Calculated: {}.", num);
    num
}


pub fn cache_main() {
    let rand_num: u32 = rand::thread_rng().gen_range(1, 101);
    let ran = || rand::thread_rng().gen_range(1, 3);
    let mut cal_cache = Cacher::new(|num| {
        calculation(num)
    });
    if rand_num < 90 {
        println!(
            "Slowly {}...",
            cal_cache.value(ran())
        );
        println!(
            "Slooowly {}...",
            cal_cache.value(ran())
        );
    } else {
        println!("God speed.");
    };
}
