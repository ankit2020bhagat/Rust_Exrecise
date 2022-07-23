use std ::io;
fn main(){
    let mut name=String::new();
    let mut dob=String::new();
    println!("Enter name");
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Enter the DOB");
    io::stdin().read_line(&mut dob).
    expect("Failed to read input");
    println!("Hi {}",name);
    println!("Dob {}",dob);
}