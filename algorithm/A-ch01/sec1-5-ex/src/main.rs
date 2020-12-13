use std::io::{self, Read};
use std::error::Error;
use std::str::SplitWhitespace;

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

fn main() -> Result<(),Box<dyn Error>> {
    init!(stdin);
    let ex:String = read!(stdin);
    match ex.as_str() {
        "1-1" => ex_1(&mut stdin),
        "1-2" => ex_2(&mut stdin),
        "1-3" => ex_3(&mut stdin),
        "1-4" => ex_4(&mut stdin),
        _ => Ok(())
    }?;
    Ok(())
}

//1-1 average
fn ex_1(stdin:&mut SplitWhitespace) -> Result<(),Box<dyn Error>>{
    let a:f64 = read!(stdin);
    let b:f64 = read!(stdin);
    let c:f64 = read!(stdin);
    println!("{:.3}",(a+b+c)/3.0);
    Ok(())
}

fn ex_2(stdin:&mut SplitWhitespace) -> Result<(),Box<dyn Error>>{
    let f:f64 = read!(stdin);
    println!("{:.3}",5.0*(f-32.0)/9.0);
    Ok(())
}

fn ex_3(stdin:&mut SplitWhitespace) -> Result<(),Box<dyn Error>>{
    let n:i32 = read!(stdin);
    println!("{}",(1..=n).sum::<i32>());
    Ok(())
}

fn ex_4(stdin:&mut SplitWhitespace) -> Result<(),Box<dyn Error>>{
    let n:i32 = read!(stdin);
    use std::f64::consts::PI;
    println!("{}",(n as f64 / (180.0/PI)).sin());
    println!("{}",(n as f64 / (180.0/PI)).cos());
    Ok(())
}