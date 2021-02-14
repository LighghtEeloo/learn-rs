use crate::utils::*;

pub fn main() {
    std::thread::spawn(move || {
        let vec_handle: Vec<thread::JoinHandle<_>> = vec!(0,1,2,3,4).iter().map(|&i: &i32| thread::spawn(move || {
            println!("> Thread #{} spawn.", i);
            thread::sleep(Duration::from_millis(2000));
            println!("^ Bye, #{}!", i);
        })).collect();
        let _vec_handle: Vec<()> = vec_handle.into_iter().map(|x| x.join().expect("Thread Panicked.")).collect();
    }).join().expect("Thread Panicked.");


    for i in 0..10 {
        println!("* Main #{}!", i);
        thread::sleep(Duration::from_millis(100));
    }
}