fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}


fn plus_one(x: Option<i32>) ->Option<i32>
{
    match x
    {
        Some(i) => Some(i+1),
        _ => None
    }
}

#[derive(Debug)]
enum UsState
{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8
{
    match coin
    {
        Coin::Penny =>
        {
            println!("lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>
        {
            println!("state quarter from {:?}!", state);
            25
        }
    }
}