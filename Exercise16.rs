use std ::io;
fn main(){
    let mut num1=String::new();
    println!("Enter the number");
    io::stdin().read_line(&mut num1).expect("failed to read");
     let mut num1:i32=num1.trim().parse().expect("invalid input");
    let _100th=num1/100;
    num1=num1%100;
    let _10th=num1/10;
    num1=num1%10;
    let _5th=num1/5;
   
        
         
        
         
    
    println!("100_th note {} 10_th note {} 5_th npte {}",_100th,_10th,_5th);
}