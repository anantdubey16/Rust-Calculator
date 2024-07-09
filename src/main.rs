use std::io;
fn main() {
    //creating menu where we choose addition, division etc.
    loop {
        println!("Menu");       
        println!("1. Addition");
        println!("2. Substraction");
        println!("3. multiplication");
        println!("4. Division");
        println!("5. Exit");

        println!("Enter your choice:");

        //input of choice
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        if choice == 5 {
            println!("Exiting Calculator");
            break;
        }

        //input of two numbers

        println!("Enter num1");
        let mut input1 = String::new();
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read line");
        let a: f64 = match input1.trim().parse() {
            Ok(a) => a,
            Err(_) => {
                println!("Failed to parse num1. Please enter a valid integer.");
                return;
            }
        };

        println!("Enter num2");
        let mut input2 = String::new();
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read line");
        let b: f64 = match input2.trim().parse() {
            Ok(b) => b,
            Err(_) => {
                println!("Failed to parse num1. Please enter a valid integer.");
                return;
            }
        };

        match choice {
            1 => println!("Result: {}", a + b),
            2 => println!("Result: {}", a - b),
            3 => println!("Result: {}", a * b),
            4 => {  
                if b == 0.0 {
                    println!("error");
                } else {
                    println!("Result; {}", a / b);
                }
            }
            _ => println!("invalid choice pls enter btw 1-5"),
        }
    }
}
