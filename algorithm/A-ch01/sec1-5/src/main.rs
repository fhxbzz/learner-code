use std::io::{self, Read};
use std::error::Error;

macro_rules! init {
    ($i:ident) => {
        let mut buffer = String::new();
        io::stdin().take(3000).read_to_string(&mut buffer)?;
        let mut $i = buffer.split_whitespace();
    }
}
macro_rules! read {
    ($i:ident) => ($i.next().unwrap().parse()?)
}

fn main() -> Result<(),Box<dyn Error>>{
    //init!(stdin);
    //A1
    //let a = 11111*11111;//6个、9个溢出
    //let a = 111111111f64*111111111f64;
    //let a = 1/0;
    //println!("{}",a);

    println!("{} {}",i32::max_value(),i32::min_value());
    println!("{}",std::f64::DIGITS);
    println!("{}",std::f64::EPSILON);
    Ok(())
}
