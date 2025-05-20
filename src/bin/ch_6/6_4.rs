// Chapter 6.2: The match control flow construct
// Listing 6-4: Patterns that bind to values
#![allow(dead_code)]

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quater from {:?}!", state);
            25
        }
    }
}

pub fn main() {
    let coin = Coin::Quater(UsState::Iowa);

    value_in_cents(coin);
}
