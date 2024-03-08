// Generate ASCII art for the given number
pub fn generate_large_number(num: u64) -> Vec<String> {
    let big_digits = vec![
        vec![" __ ", "|  |", "|__|"], // 0
        vec!["  . ", "  | ", "  | "], // 1
        vec![" __ ", " __|", "|__ "], // 2
        vec![" __ ", " __|", " __|"], // 3
        vec![".   ", "|__|", "   |"], // 4
        vec![" __ ", "|__ ", " __|"], // 5
        vec![" __ ", "|__ ", "|__|"], // 6
        vec![" __ ", "   |", "   |"], // 7
        vec![" __ ", "|__|", "|__|"], // 8
        vec![" __ ", "|__|", " __|"], // 9
    ];

    let num_str = num.to_string();
    let num_digits = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let digit_height = big_digits[0].len();
    let mut output_lines = vec![String::new(); digit_height];

    for line in 0..digit_height {
        for &digit in &num_digits {
            let segment = big_digits[digit][line];
            output_lines[line].push_str(segment);
            output_lines[line].push(' '); // Add space between digits
        }
    }

    output_lines
}

// Combine the ASCII art for minutes and seconds to create the timer display
pub fn print_large_timer(minutes: u64, seconds: u64) {
    let minutes_tens = generate_large_number(minutes / 10);
    let minutes_ones = generate_large_number(minutes % 10);
    let seconds_tens = generate_large_number(seconds / 10);
    let seconds_ones = generate_large_number(seconds % 10);

    for i in 0..minutes_tens.len() {
        let separator = match i {
            0 => ' ',
            1 => '.',
            _ => '\u{02D9}',
        };
        let out_time = format!(
            "{}{} {} {}{}",
            minutes_tens[i], minutes_ones[i], separator, seconds_tens[i], seconds_ones[i]
        );
        println!("\x1b[1m{}\x1b[0m", out_time);
    }
}
