use std::io;

fn main() {
    println!("Celcius to Fahrenheit");
    println!("{}", celcius_to_fahrenheit());

    let mut nth = String::new();

    loop {
        println!("Enter the nth Fibonacci number you want to calculate:");

        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line");

        match nth.trim().parse() {
            Ok(num) => {
                println!("Fibonacci number {}: {}", num, fibonacci(num));
                break; 
            }
            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
    }
}

fn celcius_to_fahrenheit() -> f64 {
    let mut celcius = String::new();
    
    println!("Enter your temperature in celcius: ");
    
    io::stdin()
        .read_line(&mut celcius)
        .expect("Failed to read line");

    let celcius: f64 = match celcius.trim().parse() {
        Ok(num) => num,
        Err(_) => celcius_to_fahrenheit(),
    };

    celcius * 9.0 / 5.0 + 32.0
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}