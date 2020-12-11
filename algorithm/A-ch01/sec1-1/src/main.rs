fn main() {
    //1-1
    println!("{}",1+2);
    println!("{}",3-4);
    println!("{}",5*6);
    println!("{}",8/4);
    println!("{}",8/5);
    //1-2
    //格式化用法，见std::fmt文档
    //format_string := <text> [ maybe-format <text> ] *
    //maybe-format := '{' '{' | '}' '}' | <format>
    //format := '{' [ argument ] [ ':' format_spec ] '}'
    //argument := integer | identifier
    //
    //format_spec := [[fill]align][sign]['#']['0'][width]['.' precision][type]
    //fill := character
    //align := '<' | '^' | '>'
    //sign := '+' | '-'
    //width := count
    //precision := count | '*'
    //type := identifier | '?' | ''
    //count := parameter | integer
    //parameter := argument '$'
    println!("{:.1}",8.0/5.0);
    println!("{:.2}",8.0/5.0);
    println!("{:.1}",(8/5) as f64);
    println!("{}",(8.0/5.0) as i32);
    //1-3
    println!("{:.8}",1.0+2.0*f64::sqrt(3.0)/(5.0-0.1));

    use std::collections::BinaryHeap;


}
