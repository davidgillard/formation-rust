use std::io;

fn main() {
    println!("Fibonacci generator");
    // println!("{}", fibonacci(1));
    // println!("{}", fibonacci(3));
    // println!("{}", fibonacci(5));
    println!("Type \"quit\" to end the program");
    loop {
        let mut n = String::new();
        println!("\nEnter a positive integer:");
        io::stdin().read_line(&mut n).expect("Failed to read line");
        if n.trim() == "quit" {
            break;
        }
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", fibonacci(n));
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
