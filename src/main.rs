use chrono::NaiveDate;
use clap::Parser;

mod zodiac;
mod horoscope;
mod cli;
mod ascii_art;

use zodiac::{determine_zodiac_sign, generate_lucky_numbers, generate_lucky_colors};
use horoscope::generate_horoscope;
use cli::Args;
use ascii_art::get_zodiac_ascii_art;

fn main() {
    let args = Args::parse();
    
    let birth_date = match NaiveDate::parse_from_str(&args.birth_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            eprintln!("Error: Please provide birth date in YYYY-MM-DD format (e.g., 1990-05-15)");
            std::process::exit(1);
        }
    };
    
    let zodiac_sign = determine_zodiac_sign(birth_date);
    let horoscope = generate_horoscope(zodiac_sign);
    let lucky_numbers = generate_lucky_numbers(zodiac_sign);
    let lucky_colors = generate_lucky_colors(zodiac_sign);
    let ascii_art = get_zodiac_ascii_art(zodiac_sign);
    
    println!("🌟 Tomorrow's Horoscope for {} 🌟", zodiac_sign);
    println!();
    println!("{}", ascii_art);
    println!();
    println!("📜 PREDICTION:");
    println!("{}", horoscope);
    println!();
    println!("🍀 LUCKY NUMBERS: {}", 
        lucky_numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "));
    println!("🌈 LUCKY COLORS: {}", lucky_colors.join(", "));
    println!();
    println!("✨ May the stars guide your path! ✨");
}