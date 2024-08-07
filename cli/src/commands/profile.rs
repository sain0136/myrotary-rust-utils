use crate::utils::terminal::clear_screen;
use cfonts::{say, Align, BgColors, Colors, Env, Fonts, Options};
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind};
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
struct SystemOwner {
    first_name: String,
    address: String,
    age: i8,
    gender: String,
    theme: ThemeColors,
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
                std::process::exit(0);
            }
            _ => {
                break;
            }
        }
    }
}

pub fn personal_information() {
    let os_mode = "win";
    let owner_config: Result<SystemOwner, Box<dyn std::error::Error>>;
    let file_path: &Path;
    match os_mode {
        "win" => {
            file_path = Path::new("utils-config.toml");
            owner_config = read_config(file_path);
        }
        "linux" => {
            file_path = Path::new("/etc/utils-config.toml");
            owner_config = read_config(file_path);
        }
        _ => {
            println!("Invalid OS Mode");
            println!("Exiting...");
            panic!()
        }
    }
    match owner_config {
        Ok(file) => {
            // Handle the case where the file is successfully opened
            println!("Personal Information File Loaded Successfully {:?}", file);
            let mut input: String = String::new();
            println!("Would you like to change your personal information?(y/n)");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if input.trim() == "y" || input.trim() == "yes" {
                        set_personal(file_path);
                    } else if input.trim() == "n" || input.trim() == "no" {
                        clear_screen();
                        profile_menu();
                    }
                }
                Err(_) => {
                    eprintln!("Failed to read line");
                    profile_menu();
                }
            }
        }
        Err(ref e)
            if e.downcast_ref::<io::Error>()
                .map_or(false, |e| e.kind() == ErrorKind::NotFound) =>
        {
            // Handle the specific case where the file is not found
            println!("Error: Could not find personal information file");
            let mut input: String = String::new();
            println!("Would you like to set your personal information?(y/n)");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if input.trim() == "y" || input.trim() == "yes" {
                        set_personal(file_path);
                    } else if input.trim() == "n" || input.trim() == "no" {
                        clear_screen();
                        profile_menu();
                    }
                }
                Err(_) => {
                    eprintln!("Failed to read line");
                    profile_menu();
                }
            }
        }
        Err(e) => {
            // Handle all other errors
            println!("Error: {:?}", e);
            profile_menu();
        }
    }
}

fn read_config(file_path: &Path) -> Result<SystemOwner, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let config: SystemOwner = toml::from_str(&content)?;
    Ok(config)
}

// This function writes a SystemOwner configuration to a file.
//
// Parameters:
// - file_path: A reference to a Path that specifies the file location.
// - system_owner: An instance of SystemOwner that contains the configuration data.
//
// The function performs the following steps:
// 1. Serializes the system_owner instance to a TOML string using the `toml::to_string` function.
// 2. Writes the serialized string to the specified file using the `fs::write` function.
// 3. Returns an Ok(()) if the operation is successful, or an error wrapped in a Box if any step fails.
fn write_config(
    file_path: &Path,
    system_owner: &SystemOwner,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = toml::to_string(&system_owner)?;
    fs::write(file_path, content)?;
    Ok(())
}

fn set_personal(file_path: &Path) {
    let mut owner = SystemOwner {
        first_name: String::new(),
        address: String::new(),
        age: 0,
        gender: String::new(),
        theme: ThemeColors::System,
    };

    loop {
        let options = &["Name", "Age", "Address", "Gender", "View/Save", "Exit"];
        let selection: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Set your personal information")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();
        match selection {
            0 => {
                owner.first_name = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter your first name")
                    .interact_text()
                    .unwrap();
            }
            1 => {
                owner.age = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter your age")
                    .interact_text()
                    .unwrap();
            }
            2 => {
                owner.address = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter your address")
                    .interact_text()
                    .unwrap();
            }
            3 => {
                owner.gender = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter your gender")
                    .interact_text()
                    .unwrap();
            }
            4 => {
                println!("Personal Information: {:?}", owner);
                println!("Would you like to save your personal information?(y/n)");
                let mut input: String = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        if input.trim() == "y" || input.trim() == "yes" {
                            let write_operation = write_config(file_path, &owner);
                            match write_operation {
                                Ok(_) => {
                                    println!("Personal Information Saved Successfully");
                                    profile_menu();
                                }
                                Err(e) => {
                                    println!("Error: {:?}\n", e);
                                    profile_menu();
                                }
                            }
                        } else if input.trim() == "n" || input.trim() == "no" {
                            continue;
                        }
                    }
                    Err(_) => {
                        eprintln!("Failed to read line");
                        profile_menu();
                    }
                }
            }
            5 => {
                println!("Returning to profile menu...");
                profile_menu()
            }
            _ => {
                break;
            }
        }
    }
}

pub fn change_theme() {}
