use std::io;
// fn main(){
// let positive = 4.0_f64;
// let negative = -4.0_f64;
// let negative_zero = -0.0_f64;

// let abs_difference = (positive.sqrt() ).abs();
// println!("{}",abs_difference);
// assert!(abs_difference < 1e-10);
// assert!(negative.sqrt().is_nan());
// assert!(negative_zero.sqrt() == negative_zero);
//}
fn main(){
    let mut x1=String::new();
    println!("enter the X1 Coardinate");
    io::stdin().read_line(&mut x1).expect("failed to read");
    let x1:f32=x1.trim().parse().expect("invalid input");
    println!("enter the X2 Coardinate");
    let mut x2=String::new();
    io::stdin().read_line(&mut x2).expect("failed to read");
    let x2:f32=x2.trim().parse().expect("invalid input");
    println!("enter the Y1 Coardinate");
    let mut y1 =String::new();
    io::stdin().read_line(&mut y1).expect("failed to read");
    let y1:f32=y1.trim().parse().expect("invalid input");
    println!("enter the Y2 Coardinate");
    let mut y2=String::new();
    io::stdin().read_line(&mut y2).expect("failed to rerad");
    let y2:f32=y2.trim().parse().expect("invlaid input");
   
    let  mut dist:f32;

    dist=(((x1-x2)*(x1-x2))+((y1-y2)*(y1-y2))).sqrt();
    println!("{}",dist);
}