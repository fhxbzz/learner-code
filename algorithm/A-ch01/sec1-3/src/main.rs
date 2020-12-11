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

fn main() -> Result<(), Box<dyn Error>>{
    init!(stdin);
    //1-6
    //let n:i32 = read!(stdin);
    //let a = n/100;
    //let b = n/10%10;
    //let c = n%10;
    //println!("{}{}{}",c,b,a);
    //1-7
    //println!("{:03}",c*100+b*10+a);
    //1-8
    let mut a:i32 = read!(stdin);
    let mut b:i32 = read!(stdin);
    //用元组（推荐）
    //let (a,b) = (b,a);
    //倒酱油
    let c = a;
    a = b;
    b = c;
    //加减法
    //let c = a + b;
    //a = b;
    //b = c - a;
    //位运算
    //let c = a ^ b;
    //a = b;
    //b = c ^ a;
    println!("{} {}",a,b);
    Ok(())
}
