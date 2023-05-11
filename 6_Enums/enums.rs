enum IpAddrKind {
    V4(String),
    V6(String),
}

// let home = IpAddrKind::V4(String::from("127.0.0.1"));
// let loopback = IpAddr::V6(String::from("::1"));

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let x: i8 = 5;
    // let y: Some<i8> = Some(5);
    
    // let sum = x+ y;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// match control flow
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i)  => Some(i + 1),
    }
}

// catch all patterns
fn catch_all_patterns() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat(){}
fn remove_fancy_hat(){}

