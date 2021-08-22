fn main() {
    let hello = String::from("Hello world!");

    // first_word work on slices of &String
    let word = first_word(&hello[..]);

    println!("{}", &word);

    let string_literal = "Coucou rust";

    // first_word work on slices of string literals
    let word = first_word(&string_literal[..]);
    
    println!("{}", &word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax.
    let word = first_word(string_literal);

    println!("{}", &word);


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}












// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }