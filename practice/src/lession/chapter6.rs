#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[test]
fn enum_demp() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);

    let some_number = Some(5);
    let some_string = Some("a string");
    println!("{:?}", some_number); // Some(5)
    println!("{:?}", some_number.unwrap()); // 5

    let absent_number: Option<i32> = None;
    assert_eq!(None.unwrap_or("bike"), "bike");
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    // assert_eq!("222", "44");
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
#[test]
fn test_01() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {
        println!("{}", num_spaces);
    }
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
