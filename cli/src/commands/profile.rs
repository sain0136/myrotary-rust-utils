use crate::utils::terminal::clear_screen;
use cfonts::{say, Align, BgColors, Colors, Env, Fonts, Options};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;

const TITLE: &str = "Profile Settings";
const GREETING: &str = "Welcome to the Profile Settings Menu - Please select an option\nAll settings will be written to a file";

#[derive(Serialize, Deserialize, Debug)]
enum ThemeColors {
    System,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    RedBright,
    GreenBright,
    YellowBright,
    BlueBright,
    MagentaBright,
    CyanBright,
    WhiteBright,
}

#[derive(Serialize, Deserialize, Debug)]
enum SystemOwner {
    FirstName(String),
    Lastname(String),
    Address(String),
    Age(i8),
    Theme(ThemeColors), // import and use thier enum colors
}

pub fn profile_menu() {
    say(Options {
        text: String::from(TITLE),
        font: Fonts::FontSimple3d,
        colors: vec![Colors::CyanBright],
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
    loop {
        let options = &["Personal Information", "Change Theme", "Exit"];
        let selection: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();
        match selection {
            0 => {
                personal_information();
            }
            1 => {
                change_theme();
            }
            2 => {
                clear_screen();
                println!("Exiting...");
                break;
            }
            _ => {
                break;
            }
        }
    }
}

pub fn personal_information() {
    let os_mode = "win";
    let owner_config: Result<Vec<SystemOwner>, Box<dyn std::error::Error>>;
    match os_mode {
        "win" => {
            let file_path = Path::new("utils-config.toml");
            owner_config = read_config(file_path);
        }
        "linux" => {
            let file_path = Path::new("/etc/utils-config.toml");
            owner_config = read_config(file_path);
        }
        _ => {
            println!("Invalid OS Mode");
            println!("Exiting...");
            panic!()
        }
    }
    println!("Config: {:?}", owner_config);
}

fn read_config(file_path: &Path) -> Result<Vec<SystemOwner>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let config: Vec<SystemOwner> = toml::from_str(&content)?;
    Ok(config)
}

pub fn change_theme() {}
