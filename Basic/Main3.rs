use std ::io;
fn main(){
    let mut array:[String;3];
    let mut str1=String::new();
  
    println!("Enter the character");
    for i in 1..4 {
        io::stdin().read_line(&mut str1).expect("Failed to read input");
        array[i]=str1;
    }
    // for i in array {
    //     print!("{}",i);
    // }
}