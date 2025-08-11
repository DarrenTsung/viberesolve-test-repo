use chrono::NaiveDate;
use clap::Parser;

mod zodiac;
mod horoscope;
mod cli;

use zodiac::determine_zodiac_sign;
use horoscope::generate_horoscope;
use cli::Args;

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
    
    println!("🌟 Tomorrow's Horoscope for {} 🌟", zodiac_sign);
    println!();
    println!("{}", horoscope);
    println!();
    println!("✨ May the stars guide your path! ✨");
}