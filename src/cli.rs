use crate::timer::start_timer;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "Pomato", version = "1.0", about = "A CLI Pomodoro Timer", long_about = None)]
pub struct Cli {
    #[clap(value_parser)]
    pub duration: u64,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        // start_timer(self.duration)?;
        start_timer(self.duration)?;
        Ok(())
    }
}
