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


    slice_array();


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

fn slice_array() {
  let a = [1, 2, 3, 4, 5, 6];

  let slice = &a[1..3];

  assert_eq!(slice, &[2, 3]);

  println!("{}", &slice[0]);
  println!("{}", &slice[1]);
}
