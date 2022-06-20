// mod front_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         pub fn seat_at_table() {}
//     }

//     pub mod serving {
//         pub fn take_order() {}
//         pub fn take_payment() {}
//     }
// }
// fn call_order() {}

// mod back_house {
//     fn cook_order() {}
//     fn fix_order() {
//         super::call_order(); // lùi lại 1 level
//         cook_order();
//     }
// }

// fn eat_at_restanrant() {
//     crate::front_house::hosting::add_to_waitlist();

//     front_house::hosting::add_to_waitlist();
// }

use rand::{Rng, RngCore, SeedableRng};

mod back_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }

    #[derive(Debug)]
    pub enum Salad {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn monday(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("banana"),
            }
        }
    }
}

mod front_house;

fn eat_res() {
    front_house::hosting::add_to_waitlist();
}

fn eat_at_restanrant() {
    let mut order = back_house::Breakfast::monday("Fish");
    order.toast = String::from("chicken");
    println!("order: {:#?}", order);

    let order1 = back_house::Salad::Soup;

    println!("order1: {:#?}", order1);
}

fn main() {
    eat_at_restanrant();
}
