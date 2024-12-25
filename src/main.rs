use std::io;

fn main() {
    let mut lang_str = String::new();

    let mut num1_str = String::new();
    let mut num2_str = String::new();

    let mut action_str = String::new();

    println!("Choose your language:(ru or en)");
    // Lang choose
    match io::stdin().read_line(&mut lang_str) {
        Ok(_) => {
            lang_str = lang_str.trim().to_string();
            if lang_str == "en" {
                println!("Choose first number:");
                // Num1 choose
                match io::stdin().read_line(&mut num1_str) {
                    Ok(_) => {
                        println!("Choose second number:");
                        // Num2 choose
                        match io::stdin().read_line(&mut num2_str) {
                            Ok(_) => {
                                println!("Choose action:");
                                match io::stdin().read_line(&mut action_str) {
                                    Ok(_) => {},
                                    Err(_) => println!("Failed to read <action_str>")
                                }
                            }
                            Err(_) => println!("Failed to read <num2_str>")
                        }
                    }
                    Err(_) => println!("Failed to read <num1_str>")
                }
            } else {
                println!("Выберите первое число:");
                // Num1 choose
                match io::stdin().read_line(&mut num1_str) {
                    Ok(_) => {
                        println!("Выберите второе число:");
                        // Num2 choose
                        match io::stdin().read_line(&mut num2_str) {
                            Ok(_) => {
                                println!("Выберите действие:");
                                match io::stdin().read_line(&mut action_str) {
                                    Ok(_) => {},
                                    Err(_) => println!("Failed to read <action_str>")
                                }
                            }
                            Err(_) => println!("Failed to read <num2_str>")
                        }
                    }
                    Err(_) => println!("Failed to read <num1_str>")
                }
            }
        }
        Err(_) => println!("Failed to read <lang_str>")
    }



let num1: f64 = num1_str.trim().parse().expect("Failed to parse number");
    let num2: f64 = num2_str.trim().parse().expect("Failed to parse number");
    let action: char = action_str.chars().next().expect("No symbol found");


    match action {
        '+' => {
            let result = num1 + num2;
            println!("{} + {} = {}", num1, num2, result);
        },
        '-' => {
            let result = num1 - num2;
            println!("{} - {} = {}", num1, num2, result);
        },
        '*' => {
            let result = num1 * num2;
            println!("{} * {} = {}", num1, num2, result);
        },
        '/' => {
            if num2 == 0 as f64 {
                if lang_str == "en" {
                    println!("Division by zero is undefined.");
                } else {
                    println!("На ноль делить нельзя.");
                }
            } else {
                let result = num1 / num2;
                println!("{} / {} = {}", num1, num2, result);
            }
        }
        ':' => {
            if num2 == 0 as f64 {
                if lang_str == "en" {
                    println!("Division by zero is undefined.");
                } else {
                    println!("На ноль делить нельзя.");
                }
            } else {
                let result = num1 / num2;
                println!("{} : {} = {}", num1, num2, result);
            }
        }
        _ => println!("Invalid operation"),
    }

}
