#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn print_state(&self) {
        print!("Selected state is {self:?}")
    }
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let coin = Coin::Quarter(UsState::Alabama);

    let in_cents = value_in_cents(&coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Value of six is {six:?}");
    println!("Value of none is: {none:?}");

    if let Coin::Nickel = coin {
        println!("Nickel")
    }

    if let Coin::Quarter(state) = &coin {
        println!("Coin state {state:?}")
    }

    let dice_roll = 9;
    match dice_roll {
        1 => println!("You got 1. Sad!"),
        _ => println!("Hello World"),
    }

    coin_quarter_and_alabama(&coin);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn coin_quarter_and_alabama(coin: &Coin) -> bool {
    let Coin::Quarter(state) = coin else {
        return false;
    };
    state.print_state();
    match state {
        UsState::Alabama => true,
        UsState::Alaska => false,
    }
}
