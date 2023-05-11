pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }
    fn cook_order(){}

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting;
// use crate::back_of_house;
// use crate::back_of_house;
// use std::fmt::Result;
// use std::io::Result as IoResult;
// use std::{cmp::Ordering, io};
// use std::io::{self, Write};
// use std::collections::*;


mod customer{
    use crate::front_of_house::hosting::add_to_waitlist;
    use crate::back_of_house;
    pub fn eat_at_restaurant() {
        // Absolute path
        // crate::front_of_house::hosting::add_to_waitlist();
        // Relative path
        add_to_waitlist();
        
        // all enum fields are public
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}