// lib.rs 자체로도 crate 모듈
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {
        }
    }

    fn front_house_func() {}

    pub mod serving {
        pub fn take_order() {
        }

        fn something1() {
            super::front_house_func();
        }

        fn something2() {
            super::hosting::add_to_waitlist();
        }
    }
}

mod customer {
    pub use crate::front_of_house::hosting::add_to_waitlist as add_wait;
    pub fn eat_somegthing() {
        super::eat_at_restaurant();
        add_wait();
    }
}


pub fn eat_at_restaurant() {
    crate::front_of_house::serving::take_order();
    customer::add_wait();
    // front_of_house::hosting::add_to_waitlist();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod my_function;
mod some;
use some::SomeStruct;

fn some_function() {
    my_function::do_something();
    let mystruct = SomeStruct;
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