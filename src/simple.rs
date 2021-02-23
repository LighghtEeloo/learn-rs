
pub fn main() {
    let string = String::from("Hello.");
    borrow_and_return(&string);
    println!("{}", string);

    //// won't compile.
    // let mut a = 1;
    // let p = &mut a;
    // a = 2;
    // *p = 3;
}

fn borrow_and_return(string: &String) -> &String {
    string
}
