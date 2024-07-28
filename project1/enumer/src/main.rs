
//#[derive(Debug)]
/*enum Coin{

    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {

    match coin{
        Coin::Penny => {
            println!("Luck Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
} */
/*enum UsState{

    Alabama,
    Alaska,
}
enum Coin1{

    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents1(coin: Coin1) -> u8{
    
    match coin{

        Coin1::Penny => 1,
        Coin1::Nickel => 12,
        Coin1::Dime => 13,
        Coin1::Quarter(state) => {

            println!("State quarter from {:?}", state);
            25
        },

    }
}

fn main() {
    //let state = UsState;

    
    let s = value_in_cents1(Coin1::Quarter((UsState::Alabama)));

    println!("s {}", s);
}*/

fn plus_one(num: Option<i32>) -> Option<i32>{

    match num{

        Option::None => None,
        Option::Some(i) => Some(i + 1),
    }
    
}
fn main(){

    let five = Some(5);
    let six = plus_one(five);

    let none = plus_one(None);

    println!("six {:?} none {:?}", six, none);

}