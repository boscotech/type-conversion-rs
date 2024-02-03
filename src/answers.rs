fn conversions() {
    let integer: u8 = 64;
    let stringint = "64";
    //convert integer into a u64 number using the `as` keyword
    let u64num: u64 = integer as u64;
    //convert integer into a u64 number using the .into() function
    let u64numinto: u64 = integer.into();
    //convert stringint into a u64 number using the .parse::<u64>().unwrap() function
    let u64numfromstring: u64 = stringint.parse::<u64>().unwrap();
    //convert u64num into a char using the `as` keyword
    let character: char = integer as char;
    //define a f32 literal using the type name after the value
    let f32num = 3f32;
    //define a name using String::from() function
    let name = String::from("lolpro11");
}
fn main() {
    conversions();
}

#[cfg(test)]
mod test;