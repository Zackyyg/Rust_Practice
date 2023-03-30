fn main() {
    use std::io::{stdin, stdout, Write};
    loop {
        let _ = stdout().flush();
        println!("Please slecet an option:");
        println!("\t1. Add");
        println!("\t2. Subtract");
        println!("\t3. Multiply");
        println!("\t4. Divide");
        println!("\t5. Exit\n");

        let mut expression = String::new();
        stdin().read_line(&mut expression).expect("Failed to read line");

        match expression.trim() {
            "1" => {
                println!("Enter the first number:");
                let _ = stdout().flush();
                let mut first_number = String::new();
                stdin().read_line(&mut first_number).expect("Failed to read line");
                let first_number: f64 = first_number.trim().parse().expect("Please type a number!");

                println!("Enter the second number:");
                let _ = stdout().flush();
                let mut second_number = String::new();
                stdin().read_line(&mut second_number).expect("Failed to read line");
                let second_number: f64 = second_number.trim().parse().expect("Please type a number!");

                let result = first_number + second_number;
                println!("Result: {}", result);
            },
            "2" => {
                println!("Enter the first number:");
                let _ = stdout().flush();
                let mut first_number = String::new();
                stdin().read_line(&mut first_number).expect("Failed to read line");
                let first_number: f64 = first_number.trim().parse().expect("Please type a number!");

                println!("Enter the second number:");
                let _ = stdout().flush();
                let mut second_number = String::new();
                stdin().read_line(&mut second_number).expect("Failed to read line");
                let second_number: f64 = second_number.trim().parse().expect("Please type a number!");

                let result = first_number - second_number;
                println!("Result: {}", result);
            },
            "3" => {
                println!("Enter the first number:");
                let _ = stdout().flush();
                let mut first_number = String::new();
                stdin().read_line(&mut first_number).expect("Failed to read line");
                let first_number: f64 = first_number.trim().parse().expect("Please type a number!");

                println!("Enter the second number:");
                let _ = stdout().flush();
                let mut second_number = String::new();
                stdin().read_line(&mut second_number).expect("Failed to read line");
                let second_number: f64 = second_number.trim().parse().expect("Please type a number!");

                let result = first_number * second_number;
                println!("Result: {}", result);
            },
            "4" => {
                println!("Enter the first number:");
                let _ = stdout().flush();
                let mut first_number = String::new();
                stdin().read_line(&mut first_number).expect("Failed to read line");
                let first_number: f64 = first_number.trim().parse().expect("Please type a number!");

                println!("Enter the second number:");
                let mut second_number = String::new();
                stdin().read_line(&mut second_number).expect("Failed to read line");
                let second_number: f64 = second_number.trim().parse().expect("Please type a number!");

                let result = first_number / second_number;
                println!("Result: {}", result);
            },
            "5" => {
                println!("Goodbye!");
                break;
            },
            _ => {
                println!("Please enter a valid option");
            }
        }
    }

}

