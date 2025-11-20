struct User{
    userName:String,
    email:String,
    is_admin:bool,
    age:i32,
}
fn main(){
    let user1=User{
        userName:String::from("Gaurav"),
        email:String::from("gaurav@example.com"),
        age:22,
        is_admin:true,
    };
    println!("USerName :{}",user1.userName);
    println!("Email :{}",user1.email);
    println!("Age :{}",user1.age);
    println!("Is Admin :{}",user1.is_admin);

}