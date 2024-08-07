mod commands;
mod utils;
use crate::utils::terminal::clear_screen;
use clap::Parser;
pub mod prod;
use prod::prod;

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

    /// Server Actions Menu
    #[arg(short, long)]
    server: bool,

    // Personal Information
    #[arg(short, long)]
    profile: bool,
}

fn main() {
    clear_screen();
    let args = Args::parse();
    prod();

    match args {
        Args { game: true, .. } => commands::game::guessing_game("Sebastien"),
        Args {
            calculator: true, ..
        } => commands::calculator::calculator(),
        Args { server: true, .. } => commands::server::server_menu(),
        Args { profile : true, ..} => commands::profile::profile_menu(),
        _ => {
            println!(
                "Hello,{} you called this program {} times!",
                args.name, args.count
            );
        }
    }
}
