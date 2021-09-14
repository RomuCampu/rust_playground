#[allow(dead_code)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {
            println!("Added to restaurant");
        }
    }

    pub mod server {
        fn server_takes_order() {
            println!("Can I get your order?");
        }

        pub mod serving {
            pub fn take_order() {
                crate::front_of_house::server::server_takes_order();
                println!("Done!")
            }
        }
    }
}

fn serve_order() {
    println!("Serving order..");
}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {
        println!("Cooking incorrect order...");
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_wait_list();
    crate::front_of_house::server::serving::take_order();
    // crate::front_of_house::back_of_house::fix_incorrect_order();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();
}
