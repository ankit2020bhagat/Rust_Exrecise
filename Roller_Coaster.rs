use std::io;
fn main(){
    let mut test=String::new();
    io::stdin().read_line(&mut test).expect("failed to read");
    let test:i32=test.trim().parse().expect("invalud input");
    let mut x=String::new();
    let mut h=String::new();
    for i in (0..test){
       io::stdin().read_line(&mut x).expect("failed to read");
       io::stdin().read_line(&mut h).expect("failed to read");
       let x :i32=x.trim().parse().expect("invalid input");
       let h :i32=h.trim().parse().expect("invalid input");
       if x>=h {
           println!("YES");
       }
       else{
           println!("NO");
       }
    }

}