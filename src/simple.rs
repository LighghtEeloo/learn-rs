
pub fn main() {
    let string = String::from("Hello.");
    borrow_and_return(&string);
    println!("{}", string);
}

fn borrow_and_return(string: &String) -> &String {
    string
}
