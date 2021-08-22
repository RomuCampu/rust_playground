fn main() {
    let mut s = String::from("Hello");
    change_str(&mut s);
    println!("{}", s);
}

fn change_str(some_str: &mut String) {
    some_str.push_str(", World!");
}
