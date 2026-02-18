use std::io;
use std::time::Instant;

fn main() {
    loop {
        let n = user_input(String::from("Please enter the value of n."));
        if n < 0 {
            println!("Exiting...");
            break
        }

        let start = Instant::now();
        println!("{n}th fibonacci number is {}", fibonacci(n as u32));
        println!("Time taken: {:?}", start.elapsed().as_secs());
    }
}

fn fibonacci(n: u32) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn user_input(msg: String) -> i32 {
    let mut user_input = String::new();

    println!("{msg}");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Invalid user input");

    user_input.trim().parse().expect("Unable to parse value")
}
