
use std::num::ParseIntError;
#[allow(dead_code)]
fn area_square_one(side: &str) -> i32 {
    let x:Result<i32, ParseIntError> = side.parse();
    let x = x.unwrap_or_else(|_e| 0);
    x*x
}
fn area_square(side: &str) -> i32 {
    let x:i32 = side.parse().unwrap();
    x*x
}

pub fn errors() {
    //let area = area_square("10");
    let area = area_square("10a");
    println!("area = {}", area);
    let area = area_square_one("10a");
    println!("area = {}", area);
}