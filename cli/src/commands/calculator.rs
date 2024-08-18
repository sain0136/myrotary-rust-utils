use crate::utils::terminal;
use cfonts::{say, Align, BgColors, Colors, Env, Fonts, Options};
use dialoguer::{theme::ColorfulTheme, Select};
use std::io;
const GREETING: &str = "Welcome to the Calculator Version 0.0.1";

struct NumberSet {
    num1: i32,
    num2: i32,
}

impl NumberSet {
    fn find_largest(&self) -> i32 {
        match self.num1.cmp(&self.num2) {
            std::cmp::Ordering::Greater => self.num1,
            std::cmp::Ordering::Less => self.num2,
            std::cmp::Ordering::Equal => self.num1, // or self.num2, since they are equal
        }
    }
}

enum CalcResult {
    Success(i32),
    Error(String),
}

pub fn calculator() {
    say(Options {
        text: String::from("Rotary Calculator"),
        font: Fonts::FontBlock,
        colors: vec![Colors::System],
        background: BgColors::Transparent,
        align: Align::Left,
        letter_spacing: 1,
        line_height: 1,
        spaceless: false,
        max_length: 0,
        gradient: Vec::new(),
        independent_gradient: false,
        transition_gradient: false,
        raw_mode: false,
        env: Env::Cli,
        ..Options::default()
    });

    println!("{}\n", GREETING);
    let mut number_set = NumberSet { num1: 0, num2: 0 };
    prompt(&mut number_set);
    loop {
        let options = &[
            "Add",
            "Subtract",
            "Multiply",
            "Divide",
            "Exponentiate",
            "Reset",
            "Exit",
        ];
        let selection: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        match selection {
            0 => {
                println!("You chose to Add");
                add(&number_set.num1, &number_set.num2);
            }
            1 => {
                println!("You chose to Subtract");
                subtract(&number_set.num1, &number_set.num2);
            }
            2 => {
                println!("You chose to Multiply");
                multiply(&number_set.num1, &number_set.num2);
            }
            3 => {
                println!("You chose to Divide");
                match divide(&number_set.num1, &number_set.num2) {
                    CalcResult::Success(i) => print_result(i),
                    CalcResult::Error(err) => print!("{}", err),
                }
            }
            4 => {
                println!("You chose to Exponentiate");
                match exponentiate(&number_set.num1, &number_set.num2) {
                    CalcResult::Success(i) => print_result(i),
                    CalcResult::Error(err) => print!("{}", err),
                }
            }
            5 => {
                println!("You chose to Reset the numbers");
                terminal::clear_screen();
                prompt(&mut number_set);
            }
            6 => {
                println!("Exiting...");
                return;
            }
            _ => unreachable!(),
        }
        println!("\n\n");
    }
}

fn prompt(set: &mut NumberSet) {
    loop {
        let mut input: String = String::new();
        println!("Enter two numbers you would like to operate on separated by a space.");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut iter = input.split_whitespace();
                let num1: i32 = match iter.next() {
                    Some(num) => match num.parse() {
                        Ok(n) => n,
                        Err(_) => {
                            eprintln!("Failed to parse the first number");
                            continue;
                        }
                    },
                    None => {
                        eprintln!("No first number provided");
                        continue;
                    }
                };
                let num2: i32 = match iter.next() {
                    Some(num) => match num.parse() {
                        Ok(n) => n,
                        Err(_) => {
                            eprintln!("Failed to parse the second number");
                            continue;
                        }
                    },
                    None => {
                        eprintln!("No second number provided");
                        continue;
                    }
                };
                println!("You entered: {} and {}", num1, num2);
                set.num1 = num1;
                set.num2 = num2;
                println!("The largest number is: {}", set.find_largest());
                break;
            }
            Err(_) => {
                eprintln!("Failed to read line");
                continue;
            }
        }
    }
}

fn add(num1: &i32, num2: &i32) {
    let sum = num1 + num2;
    print_result(sum);
}

fn subtract(num1: &i32, num2: &i32) {
    let difference = num1 - num2;
    print_result(difference);
}

fn multiply(num1: &i32, num2: &i32) {
    let product = num1 * num2;
    print_result(product);
}

fn divide(num1: &i32, num2: &i32) -> CalcResult {
    if *num2 == 0 {
        return CalcResult::Error(String::from("Cant devide by zero"));
    }
    let quotient = num1 / num2;
    CalcResult::Success(quotient)
}

fn exponentiate(base: &i32, exponent: &i32) -> CalcResult {
    if *base == 0 && *exponent < 0 {
        return CalcResult::Error(String::from("Cannot raise zero to a negative power"));
    }

    let result = (*base as i64).checked_pow(*exponent as u32);

    // Check for overflow
    match result {
        Some(value) => return CalcResult::Success(value as i32),
        None => return CalcResult::Error(String::from("Overflow occurred")),
    };
}

fn print_result(result: i32) {
    println!("The result is: {}", result);
}
