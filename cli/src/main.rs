mod commands;
use clap::Parser;

const VERSION: &str = "1.0.0";
const ABOUT: &str = "My Rotary Server Rust CLI";
const LONG_ABOUT: &str = "My Rotary Server Rust CLI -- An user-friendly CLI for server administration and file management.";
// use commands::game;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version = VERSION, about = ABOUT, long_about = LONG_ABOUT)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from("Nameless"))]
    name: String,

    /// Number of times to greet
    #[arg(short = 'k', long, default_value_t = 1)]
    count: u8,

    /// Play a game
    #[arg(short, long)]
    game: bool,

    /// Run the calculator
    #[arg(short, long)]
    calculator: bool,
}

fn main() {
    let args = Args::parse();

    if args.game {
        commands::game::guessing_game("Sebastien");
    } else if args.calculator {
        commands::calculator::calculator();
    } else {
        println!(
            "Hello,{} you called this program {} times!",
            args.name, args.count
        );
    }
}
