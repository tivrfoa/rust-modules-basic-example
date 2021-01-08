// as main.rs has 'mod m1', I can use it here
use crate::m1::*;

pub fn hey_yo() {
    println!("hey yo!");

    m1::met1();
}
