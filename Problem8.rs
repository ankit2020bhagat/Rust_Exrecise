use std::io;
fn main(){
    let mut days=String::new();
    println!("Enter the number of days");
    io::stdin().read_line(&mut days).expect("failed to read line");
    let days:i32=days.trim().parse().expect("invalid input");
    let year=days/365;
    println!("year {}",year);
    let week=(days%365)/7;
    println!("week {} ",week);
    let day=(days%365)%7;
    println!("days {}",day);
} 