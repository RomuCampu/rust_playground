fn main() {
    separator();

    //___________ Create with the structs syntax ___________//

    let user1 = User {
        username: String::from("some user name"),
        email: String::from("someuseremail@example.com"),
        sign_in_count: 2,
        active: true,
    };
  
    println!("Username: {}\nEmail: {}\nSignin Count: {}\nActive: {}", 
            user1.username, user1.email, user1.sign_in_count, user1.active);


    separator();
    //___________ Create with functions ___________//

    let mut user2 = build_user(String::from("user 2 name"), String::from("user2@example.com"));

    user2.username = String::from("user2Name");

    println!("{}", user2.username);
    separator();

    //___________ Create from existing structs ___________//

    let user3 = User {
        email: String::from("user3email@example.com"),
        username: String::from("User3username"),
        active: user1.active, // by calling specific field or another struct
        ..user2 // or asigning the rest of the values of another struct
    };

    println!("{}", user3.active);
    separator();

    // Tupple Structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.2);
    println!("{}", origin.2);

    separator();

}


struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tupple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 2,
        active: true,
    }
}

fn separator() {
    let separator = String::from("___________________________________");
    println!("{}", separator);
}