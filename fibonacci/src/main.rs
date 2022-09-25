fn main() {
    println!("{}", fibonacci(1));
    println!("{}", fibonacci(2));
    println!("{}", fibonacci(3));
    println!("{}", fibonacci(4));
    println!("{}", fibonacci(5));
}

fn fibonacci(number: u32) -> u32 {
    match number {
        0 => 0,
        1 | 2 => 1,
        _ => fibonacci(number - 1) + fibonacci(number - 2)
    }
}