use anyhow::Result;
use std::io::{self, Write};

pub fn clear_screen() -> Result<()> {
    // Clear the screen in a cross-platform way
    // This method would vary depending on the target OS
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush()?;
    Ok(())
}

pub fn print_header() {
    println!("\x1b[1mPomato App | ⌐■-■ \x1b[0m");
    println!("┌──────────────────────────────────────────────────┐");
    println!("│ The timer has begun! Stay focused and productive!│");
    println!("└──────────────────────────────────────────────────┘");
    println!("                                     https://cwnt.io");
}
