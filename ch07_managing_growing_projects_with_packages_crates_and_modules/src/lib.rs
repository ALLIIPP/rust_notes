
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){ 
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub mod hosting{
   pub fn add_to_waitlist(){}
}



/*
use std::fmt::Result;
use std::io::Result as IoResult;

/////////////////
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
        fn seat_at_table(){}
    }

    mod serving{
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}


mod back_of_house{
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

//use crate::back_of_house::Breakfast;
pub use self::back_of_house::Breakfast;

pub fn eat_at_restaurant(){
    let mut meal = Breakfast::summer("Rye");
    
    meal.toast = String::from("wheat");
    
    println!("Id like {} toast please", meal.toast);

    
    
}
*/
