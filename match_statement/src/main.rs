fn main() {
   let penny = value_in_cents(Coin::Penny);
   let quarter = value_in_cents(Coin::Quarter(UsState::Alabama));

   println!("{}", penny);
   println!("{}", quarter);


   let five = Some(5);
   let six = plus_one(five);
   let none = plus_one(None);

   println!("{:#?}", six);
   println!("{:#?}", none);

   let some_other_val = 0u8;
   match some_other_val {
       1 => println!("Onee"),
       3 => println!("Three"),
       5 => println!("Five"),
       7 => println!("Seven"),
       9 => println!("Nine"),
       _ => (),
   }
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}