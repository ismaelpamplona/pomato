use crate::timer::start_timer;
use clap::{Arg, Command};

pub fn run() {
    let matches = Command::new("pomato")
        .arg(
            Arg::new("DURATION")
                .help("Sets the duration in minutes for the Pomodoro timer")
                .required(true)
                .index(1),
        )
        .get_matches();

    let duration_str = matches
        .get_one::<String>("DURATION")
        .expect("DURATION argument not found");

    let duration_min: u64 = duration_str.parse().expect("Duration needs to be a number");

    start_timer(duration_min);
}
