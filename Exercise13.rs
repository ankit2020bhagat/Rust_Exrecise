use std ::io;
fn main(){
    let mut num1=String::new();
    println!("Enter the first Number");
    io::stdin().read_line(&mut num1).expect("failed to read");
    let num1:i32=num1.trim().parse().expect("invlid input");
    let mut num2=String::new();
    println!("Enter the Second Number");
    io::stdin().read_line(&mut num2).expect("failed to read");
    let num2:i32=num2.trim().parse().expect("invalid input");
    println!("Enter the Third Number");
    let mut num3=String::new();
    io::stdin().read_line(&mut num3).expect("failed to read");
    let num3:i32=num3.trim().parse().expect("invalid input");
    if num1>num2{
        if num1>num3{
            println!("Num1 is the largest number {}",num1);
        }
        else{
            println!("Num3 is the largest number {}",num3);
        }
    }
    else{
        if num2>num3{
            println!("Num2 is the largest number {}",num2);
        }
        else{
            println!("Num3 is the largest number {}",num3);
        }
    }
        
    
}