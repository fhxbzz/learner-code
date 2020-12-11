use std::io::{self, Read};
use std::error::Error;

macro_rules! init {
    ($i:ident) =>{
            let mut buffer = String::new();
            io::stdin().take(3000).read_to_string(&mut buffer)?;
            let mut $i = buffer.split_whitespace();
    };
}
macro_rules! read {
    ($i:ident) => ($i.next().unwrap().parse()?)
}

fn main() -> Result<(),Box<dyn Error>>{
    init!(stdin);
    //1-4
    //let a:i32 = read!(stdin);
    //let b:i32 = read!(stdin);
    //println!("{}",a+b);
    //1-5
    //
    use std::f64::consts::*;
    let r:f64 = read!(stdin);
    let h:f64 = read!(stdin);
    let s1 = PI * r * r;
    let s2 = PI * 2.0 * r * h;
    let s = 2.0 * s1 + s2;
    println!("{:.3}",s);
    Ok(())
}
