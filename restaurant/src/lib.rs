// Chapter 7.2: Defining modules to control scope and privacy
// Listing 7-1: A front_of_house module containing other modules that then contain functions

#![allow(dead_code)]

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
