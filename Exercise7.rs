use std::io;
fn main(){
     let mut char1=String::new();
     println!("Enter the character");
     io::stdin().read_line(&mut char1).expect("failed to read input");
     print!("{}",char1);
     let num: i32 = char1.trim().parse().expect("invalid input");
     println!("{}",num);
     let mut num1=String::new();
     println!("Enter the number");
     io::stdin().read_line(&mut num1).expect("Failed to read input");
     let num1:i32=num1.trim().parse().expect("invalid input");
     println!("{}",num1);
     println!("{}",num+num1);
} 