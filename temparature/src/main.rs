use std::io;

fn main() {
    loop {
        println!("Enter 1 to convert from Celsius to Fahrenheit, \nEnter 2 to convert from Fahrenheit to Celsius, \nEnter 0 to exit:");
        match take_user_input() {
            1f32 => {
                println!("Enter the temperature in Celsius:");
                let celsius = take_user_input();
                let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
                println!("{}°C is equal to {}°F", celsius, fahrenheit);
                println!();
                },
            2f32 => {
                println!("Enter the temperature in Fahrenheit:");
                let fahrenheit = take_user_input();
                let celsius = (fahrenheit - 32.0) * 5.0;
                println!("{}°F is equal to {}°C", fahrenheit, celsius);
            },
            0f32 => {
                println!("Exiting the program. Goodbye!");
                break;
            },
            _ => {
                println!("Please enter a valid option (1 or 2).");
            }

        }
    }
}

fn take_user_input() -> f32 {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim().parse().expect("Please type a number!")
}
