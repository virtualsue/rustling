fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Fibonacci generator");
    let mut i = 0;
    loop {
        println!("{}", fibonacci(i));
        i += 1;
    }
}
