use std::env;
use std::fs;

pub fn print_type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

use super::*;

#[test]
fn conversions() {
    let test = fs::read_to_string("src/main.rs").expect("Should have been able to read the file");
    if !test.contains("let u64num: u64 = integer as u64;") || !test.contains("let character: char = integer as char;") {
        panic!("You didn't use the `as` keyword!");
    }
    if !test.contains("let u64numinto: u64 = integer.into();") {
        panic!("You didn't use the .into() function!");
    }
    if !test.contains("let u64numfromstring: u64 = stringint.parse::<u64>().unwrap();") {
        panic!("You didn't use the .parse::<u64>().unwrap() function!");
    }
    if !test.contains("f32;") {
        panic!("You forgot to use a f32 literal!");
    }
    if !test.contains("let name = String::from(") {
        panic!("You forgot to use String::from()!");
    }
}