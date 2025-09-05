// In rust the variable is unmutable so in order to make an variable mutable we have to use mut keyword
fn main(){
    let mut x=1; // cannot mutated the variable unless using mut if not it throws an error
    x+=2;
    assert_eq!(x,3);
    println!("Success!");
}