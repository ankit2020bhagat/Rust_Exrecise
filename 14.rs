use std::io;
fn main(){
    let mut dist=String::new();
    println!("enter the distance covered ");
    io::stdin().read_line(&mut dist).expect("failed to read");
    let dist:f32=dist.trim().parse().expect("invalid input");
    println!("enter the speed of bike");
    let mut speed = String::new();
    io::stdin().read_line(&mut speed).expect("failed to read");
    let speed :f32=speed.trim().parse().expect("invalid input");
   
    let avg:f32;
    avg=dist/speed;
    println!("Average is {}",avg);
}