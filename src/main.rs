mod m1;
mod brother1;

// use crate::m1::m1::*;

fn main() {
    println!("Hello, world!");

    let o1 = Some(33);

    println!("{:?}", o1);

    // met1();
    m1::m1::met1();

    m1::out_m();

    brother1::hey_yo();
}
