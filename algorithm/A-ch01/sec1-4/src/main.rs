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
    init!(stdin);
    let n:i32 = read!(stdin);
    let m:i32 = read!(stdin);
    let a = (4*n-m)/2;
    let b = n-a;
    if m%2==1 || a<0 || b<0 {
        println!("No answer");
    } else {
        println!("{} {}",a,b);
    }
    Ok(())
}
