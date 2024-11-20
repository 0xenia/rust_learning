mod calc;
use calc::Calc;

fn main() {
    println!("Please enter your operator.");

    let mut operator = String::new();

    std::io::stdin().read_line(&mut operator)
        .expect("Failed to operator");

    let operator = operator.trim();

    if operator.is_empty() {
        println!("No operators have been selected.");
        return;
    }

    println!("Please write numbers to calculate with spaces between.");

    let mut number_vector = String::new();

    std::io::stdin().read_line(&mut number_vector)
        .expect("Failed to numbers.");

    let numbers: Vec<i32> = number_vector
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Unknown number read: {}", i);
            std::process::exit(1);
        }))
        .collect();


    match operator {
        "+" => {
            let sum: i32 = Calc::add(&numbers);
            println!("Sum: {}", sum);
        }

        "-" => {
            let subtract = Calc::subtract(&numbers);
            println!("Sub: {}", subtract);
        }

        "*" | "x" => {
            let multiply: i32 = Calc::multiply(&numbers);
            println!("Mul: {}", multiply);
        }

        "/" => {
            let result = Calc::divide(&numbers);
            match result {
                Some(res) => println!("Div: {}", res),
                None => println!("You cannot divide by 0."),
            }
        }

        _ => {
            println!("Unknown operator. Only arithmetic operators are supported.");
        }
    }
}


