use std::io;

fn return_name(s: &str){
    println!("My Name is :{}", s);
}

fn main(){
    let mut name=String::new();
    println!("Enter Your Name :");
    io::stdin().read_line(&mut name).expect("Enter the name :");
    return_name(&name);
}