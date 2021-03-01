use rand::random;

pub fn main() {
    let cir = 1e8 as usize;
    let mut x = 0;
    let mut vec: Vec<u64> = Vec::with_capacity(cir);
    while x < cir {
        let y = roll();
        vec.push(y);
        // println!("{}th try: {}.", x, y);
        x += 1;
    }
    let sum = vec.iter().fold(0, |x, y| x+y);
    let avg = sum as f64 / cir as f64;
    println!("Avg: {}.", avg)
}

fn roll() -> u64 {
    let mut cnt = 1;
    loop {
        if random() {
            cnt += 1;
        } else {
            break;
        }
    }
    2u64.pow(cnt)
}
