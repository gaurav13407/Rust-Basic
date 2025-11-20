enum Op {
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
}

fn calc(op: Op) {
    match op {
        Op::Add(a, b) => println!("Sum = {}", a + b),
        Op::Sub(a, b) => println!("Diff = {}", a - b),
        Op::Mul(a, b) => println!("Mul = {}", a * b),
    }
}
fn main() {
    let add_op = Op::Add(10, 5);
    let sub_op = Op::Sub(10, 5);
    let mul_op = Op::Mul(10, 5);

    calc(add_op);
    calc(sub_op);
    calc(mul_op);
}