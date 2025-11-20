fn main(){
    for i in 1..11{
        println!("{}", i);
    }
    let mut x=0;
    while x<10{
        println!("{}",x);
        x+=2;
    }
    loop{
        println!(" THis is an infinte loop!");
        break;
    }
}