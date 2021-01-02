fn factorial(num: u128) -> u128 {
    return (1..=num).product();
}

fn main() {
    println!("Factorials ");
    println!("{}", factorial(1));
    println!("{}", factorial(7));
    println!("{}", factorial(10));
    println!("{}", factorial(20));
}
