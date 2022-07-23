use std::io;
fn main(){
    let mut radius=String::new();
    println!("Enter the Radius");
    io::stdin()
    .read_line(&mut radius)
    .expect("failed to read input");
    let radius:f64=radius.trim().parse().expect("invalid input");
    println!("Radius of Circle is {}",radius);
    let  perimeter=2.0*3.14*radius;
    
    println!("Perimeter of Circe is {} ",perimeter);
    let area=3.14*radius*radius;
    println!("Area of circle {}",area);
    
}