use std::io;
fn main(){
    let mut weight=String::new();
    println!("Enter the Weight");
    io::stdin().read_line(&mut weight).expect("failed to read input");
    let weight:f64=weight.trim().parse().expect("invalid input");
    println!("enter the no of item");
    let mut no_of_item=String::new();
    io::stdin().read_line(&mut no_of_item).expect("failed to read input");
    let no_of_item:f64=no_of_item.trim().parse().expect("invalid input");

    let mut weight2=String::new();
    println!("Enter the weight of second item");
    io::stdin().read_line(&mut weight2).expect("failed to read input");
    let weight2:f64=weight2.trim().parse().expect("invalid input");
    println!("enter the no of item");
    let mut no_of_item2=String::new();
    io::stdin().read_line(&mut no_of_item2).expect("failed to read line");
    let no_of_item2:f64=no_of_item2.trim().parse().expect("invalid input"); 
    let avg =((weight*no_of_item)+(weight2*no_of_item2))/(no_of_item+no_of_item2);
    println!("{}",avg);

}