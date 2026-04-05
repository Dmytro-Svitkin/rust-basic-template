use std::io::{self,Write};

fn war()->String{
    String::from("~WAR~")
}

fn input(msg:&str)->String{
    print!("{}",msg);
    io::stdout().flush().unwrap();
    let mut result:String=String::new();
    io::stdin().read_line(&mut result).unwrap();
    result
}

fn main() {
}
