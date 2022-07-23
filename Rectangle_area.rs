use std::io;
fn main() {
    let mut length = String::new();
    println!("Enter the length");
    io::stdin()
        .read_line(&mut length)
        .expect("failed to read input.");
    let length: i32 = length.trim().parse().expect("invalid input");
    println!("{:?}", length);
    println!("Enter the width");
    let mut width=String::new();
    io::stdin()
    .read_line(&mut width)
    .expect("failed to read input.");
    let width:i32 = width.trim().parse().expect("invalid input");
    println!("{:?}", width);
    println!(" Area of Rectangle{}",length*width);
}