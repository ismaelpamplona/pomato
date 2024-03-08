use crate::ascii_art::generate_large_number;
use crate::utils::{clear_screen, print_header};
use anyhow::Result;
use rodio::{source::Source, OutputStream};
use std::{
    thread,
    time::{Duration, Instant},
};

fn print_large_timer(minutes: u64, seconds: u64) -> Result<()> {
    let minutes_tens = generate_large_number(minutes / 10);
    let minutes_ones = generate_large_number(minutes % 10);
    let seconds_tens = generate_large_number(seconds / 10);
    let seconds_ones = generate_large_number(seconds % 10);

    print_header();
    for i in 0..minutes_tens.len() {
        let separator = if i == 1 { ':' } else { ' ' };
        let out_time = format!(
            "{}{}{}{}{}",
            minutes_tens[i], minutes_ones[i], separator, seconds_tens[i], seconds_ones[i]
        );
        println!("\x1b[1m{}\x1b[0m", out_time);
    }
    println!("\n");
    Ok(())
}

pub fn start_timer(minutes: u64) -> Result<()> {
    let total_duration = Duration::new(minutes * 60, 0);
    let start_time = Instant::now();
    let end_time = start_time + total_duration;

    // Initialize the audio system
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    while Instant::now() < end_time {
        let remaining_time = end_time - Instant::now();
        clear_screen()?;
        print_large_timer(remaining_time.as_secs() / 60, remaining_time.as_secs() % 60)?;
        thread::sleep(Duration::new(1, 0)); // Sleep for 1 second
    }

    play_alarm_sound(&stream_handle)?;

    println!("Timer complete!");
    println!("\n");
    Ok(())
}

fn play_alarm_sound(stream_handle: &rodio::OutputStreamHandle) -> Result<()> {
    let file = std::fs::File::open(
        "/home/isma/workspace/cwnt/repos/me/pomato/mixkit-clock-countdown-bleeps-916.wav",
    )?;
    let source = rodio::Decoder::new(std::io::BufReader::new(file))?;
    if let Err(err) = stream_handle.play_raw(source.convert_samples()) {
        eprintln!("Error playing sound: {}", err);
    }
    Ok(())
}
