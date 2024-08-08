use std::env;
use std::error::Error;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn load_env(key: &str) -> Result<String, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let target = env::var(key)?;
    Ok(target)
}