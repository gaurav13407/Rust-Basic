fn main(){
    let  v=vec![1,2,3,4,5];
    for (i,val) in v.iter().enumerate(){
        println!("Index: {}, Value: {}", i, val);
    }

    for i in 1..15{
        if i%2==0{
            continue;
        }
        println!("Odd Numers:{}",i);
    }
}