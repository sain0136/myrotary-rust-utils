use crate::utils::terminal::clear_screen;
use cfonts::{say, Align, BgColors, Colors, Env, Fonts, Options};
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
            "Server Status",
            "Nginx Logs",
            "Update Server",
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
                echo_test();
            }
            1 => {
                list_pm2_processes();
            }
            2 => {
                show_nginx_logs();
            }
            3 => {
                update_server();
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

pub fn list_pm2_processes() {
    let output = Command::new("pm2").arg("list").output();
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
            eprintln!("Failed to execute pm2 list command: {}", e);
        }
    }
    let frontend_status = Command::new("curl")
        .arg("-I")
        .arg("http://localhost/frontend")
        .output();
    match frontend_status {
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
            eprintln!("Failed to execute curl command: {}", e);
        }
    }
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

pub fn update_server() {
    let output = Command::new("sudo")
        .arg("apt-get")
        .arg("update")
        .arg("&&")
        .arg("sudo")
        .arg("apt-get")
        .arg("upgrade")
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
            eprintln!(
                "Failed to execute apt-get update and upgrade command: {}",
                e
            );
        }
    }
}
