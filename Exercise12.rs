use std::io;
fn main(){
    struct Employee{
        emp_id:String,
        working_hr:u32,
        sal_per_hr:u32,
    }
    let mut id=String::new();
    println!("Enter the employee_id");
    io::stdin().read_line(&mut id).expect("failed to read_line");
    let mut working_hr=String::new();
    io::stdin().read_line(&mut working_hr).expect("failed to read_line");
    let working_hr=working_hr.trim().parse().expect("invalid input");
    let mut sal_per_hr=String::new();
    io::stdin().read_line(&mut sal_per_hr).expect("failed to read input");
    let sal_per_hr=sal_per_hr.trim().parse().expect("invalid input");
    let empl1=Employee{
        emp_id:id,
        working_hr:working_hr,
        sal_per_hr:sal_per_hr,
    };
    println!("{:?}",empl1);
}