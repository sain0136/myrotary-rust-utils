use crate::utils::terminal::clear_screen;
use cfonts::{say, Align, BgColors, Colors, Env, Fonts, Options};
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;
use std::str;

const TITLE: &str = "Rotary Server Admin";
const GREETING: &str = "Welcome to the Server Administration Menu";

pub fn server_menu() {
    say(Options {
        text: String::from(TITLE),
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
    loop {
        let options = &[
            "Test Echo",
            "System Info",
            "Server Status",
            "Nginx Logs",
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
                clear_screen();
                echo_test();
            }
            1 => {
                clear_screen();
                display_system_info();
            }
            2 => {
                clear_screen();
                display_servers_status();
            }
            3 => {
                clear_screen();
                show_nginx_logs();
            }
            4 => {
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

pub fn echo_test() {
    let output = Command::new("echo").arg("Testing Echo!").output();
    match output {
        Ok(output) => {
            if output.status.success() {
                println!(
                    "{}",
                    str::from_utf8(&output.stdout).expect("Invalid UTF-8 sequence")
                );
            } else {
                eprintln!("Command executed with failing error code");
                eprintln!(
                    "stderr: {}",
                    str::from_utf8(&output.stderr).expect("Invalid UTF-8 sequence")
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to execute echo command: {}", e);
            {
                let output = Command::new("echo").arg("Testing Echo!").output();
                match output {
                    Ok(output) => {
                        if output.status.success() {
                            println!(
                                "{}",
                                str::from_utf8(&output.stdout).expect("Invalid UTF-8 sequence")
                            );
                        } else {
                            eprintln!("Command executed with failing error code");
                            eprintln!(
                                "stderr: {}",
                                str::from_utf8(&output.stderr).expect("Invalid UTF-8 sequence")
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to execute echo command. Attempting ipconfig command: {}",
                            e
                        );
                        {
                            let output = Command::new("ipconfig").output();
                            match output {
                                Ok(output) => {
                                    if output.status.success() {
                                        println!(
                                            "{}",
                                            str::from_utf8(&output.stdout)
                                                .expect("Invalid UTF-8 sequence")
                                        );
                                    } else {
                                        eprintln!("Command executed with failing error code");
                                        eprintln!(
                                            "stderr: {}",
                                            str::from_utf8(&output.stderr)
                                                .expect("Invalid UTF-8 sequence")
                                        );
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to execute ipconfig command: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn display_servers_status() {
    fn execute_and_display(command: &str, args: &[&str], title: &str) {
        let output = Command::new(command).args(args).output();
        match output {
            Ok(output) => {
                display_status(title);
                if output.status.success() {
                    println!(
                        "{}",
                        str::from_utf8(&output.stdout).expect("Invalid UTF-8 sequence")
                    );
                } else {
                    eprintln!("Command executed with failing error code");
                    eprintln!(
                        "stderr: {}",
                        str::from_utf8(&output.stderr).expect("Invalid UTF-8 sequence")
                    );
                }
            }
            Err(e) => {
                eprintln!("Failed to execute {} command: {}", command, e);
            }
        }
    }

    execute_and_display("pm2", &["list"], "BACKEND STATUS");
    execute_and_display(
        "curl",
        &["-Ik", "https://myrotaryprojects.org/"],
        "FRONTEND STATUS",
    );
    execute_and_display(
        "systemctl",
        &["status", "rotaryanalytic.service"],
        "ROTARYANALYTIC SERVER GOLANG STATUS",
    );
}

pub fn show_nginx_logs() {
    let output = Command::new("tail")
        .arg("-n")
        .arg("100")
        .arg("/var/log/nginx/error.log")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!(
                    "{}",
                    str::from_utf8(&output.stdout).expect("Invalid UTF-8 sequence")
                );
            } else {
                eprintln!("Command executed with failing error code");
                eprintln!(
                    "stderr: {}",
                    str::from_utf8(&output.stderr).expect("Invalid UTF-8 sequence")
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to execute tail command: {}", e);
        }
    }
}

pub fn display_system_info() {
    display_status("SYSTEM INFORMATION");

    // List all installed packages
    println!("Linux Distribution Information:");
    execute_command("lsb_release", &["-a"]);

    // Display disk usage
    println!("\nDisk Usage:");
    execute_command("df", &["-h"]);

    // Show system memory usage
    println!("\nMemory Usage:");
    execute_command("free", &["-h"]);

    // Check for available updates
    println!("\nAvailable Updates:");
    execute_command("apt-get", &["-s", "upgrade"]);
}

pub fn display_status(title: &str) {
    let term = Term::stdout();
    term.write_line("========================================")
        .unwrap();
    term.write_line(&format!("          {}          ", title))
        .unwrap();
    term.write_line("========================================")
        .unwrap();
}

// Function to execute a command and print its output
fn execute_command(command: &str, args: &[&str]) {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!(
            "{}",
            str::from_utf8(&output.stdout).expect("Invalid UTF-8 sequence")
        );
    } else {
        eprintln!("Command executed with failing error code");
        eprintln!(
            "stderr: {}",
            str::from_utf8(&output.stderr).expect("Invalid UTF-8 sequence")
        );
    }
}
