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

fn string_to_int(x:String)->Result<isize,String>{
    let y:&str=x.trim();
    if y.chars().all(|ch|ch.is_ascii_digit()||ch=='-'){
        return Ok(y.parse::<isize>().unwrap())
    }
    Err(war())
}

fn string_to_natural(x:String)->Result<usize,String>{
    let y:&str=x.trim();
    if y.chars().all(|ch|ch.is_ascii_digit()){
        return Ok(y.parse::<usize>().unwrap())
    }
    Err(war())
}

fn main() {
}
