use crate::utils::*;

pub fn main() {
    // This variable declaration is where it's value is specified.
    let apple = Arc::new("the same apple");
    
    std::thread::spawn(move || {
        for i in 0..20 {
            // Here there is no value specification as it is a pointer to a reference
            // in the memory heap.
            let apple = Arc::clone(&apple);
        
            thread::spawn(move || {
                println!("Thread {} spawn.", i);
                thread::sleep(Duration::from_secs(2));
            
                // As Arc was used, threads can be spawned using the value allocated
                // in the Arc variable pointer's location.
                println!("{:?}", apple);
            });
        }
    }).join().expect("Thread Panicked.");

}
    