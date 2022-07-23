use std ::io;
fn main(){
    let mut p=String::new();
    println!("Enter the even value for P");
    io::stdin().read_line(&mut p).expect("Failed to read");
    let p:i32=p.trim().parse().expect("invalid input");
    let mut q=String::new();
    println!("Enter the value of Q");
    io::stdin().read_line(&mut q).expect("Failed to read");
    let q:i32=q.trim().parse().expect("invalid input");
    let mut r=String::new();
    println!("Enter the value of R");
    io::stdin().read_line(&mut r).expect("Failed to read");
    let r:i32=r.trim().parse().expect("invalid input");
    let mut s=String::new();
    println!("Enter the value of S");
    io::stdin().read_line(&mut s).expect("Failed to read");
    let s:i32=s.trim().parse().expect("invalid input");
    if q>r && s>p{
        if s+r >p+q {
            println!("Correct_Values");
        }
        else{
            println!("Wrong_Values");
        }
    }

}