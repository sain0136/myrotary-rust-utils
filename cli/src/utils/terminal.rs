use std::env;
use std::error::Error;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn load_env(key: &str) -> Result<String, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let target = env::var(key)?;
    Ok(target)
}

pub fn load_env_var(var_name: &str) -> String {
    match load_env(var_name) {
        Ok(value) => value,
        Err(e) => {
            println!("Failed to load {} env with error {:?}", var_name, e);
            println!("Exiting...");
            panic!();
        }
    }
}
