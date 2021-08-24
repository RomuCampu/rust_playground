fn main() {
    let s1 = String::from("Hello Marcelos");
    // let s2 = s1;
    let s3 = s1.clone();

    // s2.push_str(" testing");
    // println!("{} from rust", s2);
    println!("{}, {}", s1, s3);
}
