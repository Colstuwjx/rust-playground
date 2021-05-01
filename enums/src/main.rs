fn main() {
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;

    // let coin = Coin::Quarter(UsState::Alabama);
    // value_in_cents(coin);

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // let some_u8_value = 1u8;
    // match some_u8_value {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     _ => (),
    // }

    // let some_u8_value = Some(0u8);
    // if let Some(3) = some_u8_value {
    //     println!("three");
    // }
}

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i+1),
//     }
// }

// fn bad_plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i+1),
//     }
// }

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },  // patterns.
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         },
//     }
// }
