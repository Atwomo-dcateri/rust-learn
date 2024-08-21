mod art;
use crate::art::*;
use cargo_crates1::add_one;


fn main() {
    let red = kinds::PrimaryColor::Red;
    let yellow = kinds::PrimaryColor::Yellow;
    
    let c3 = utils::mix(red, yellow);

    println!("c3 is {:?}", c3);
    
}
