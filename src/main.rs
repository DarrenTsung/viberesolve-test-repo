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
    
    if args.verbose {
        eprintln!("[VERBOSE] Starting horoscope generation...");
        eprintln!("[VERBOSE] Parsing birth date: {}", args.birth_date);
    }
    
    let birth_date = match NaiveDate::parse_from_str(&args.birth_date, "%Y-%m-%d") {
        Ok(date) => {
            if args.verbose {
                eprintln!("[VERBOSE] Successfully parsed birth date: {}", date);
            }
            date
        },
        Err(_) => {
            eprintln!("Error: Please provide birth date in YYYY-MM-DD format (e.g., 1990-05-15)");
            std::process::exit(1);
        }
    };
    
    if args.verbose {
        eprintln!("[VERBOSE] Determining zodiac sign for {}", birth_date);
    }
    let zodiac_sign = determine_zodiac_sign(birth_date, args.verbose);
    if args.verbose {
        eprintln!("[VERBOSE] Zodiac sign determined: {}", zodiac_sign);
    }
    
    if args.verbose {
        eprintln!("[VERBOSE] Generating horoscope prediction...");
    }
    let horoscope = generate_horoscope(zodiac_sign, args.verbose);
    
    if args.verbose {
        eprintln!("[VERBOSE] Generating lucky numbers...");
    }
    let lucky_numbers = generate_lucky_numbers(zodiac_sign, args.verbose);
    
    if args.verbose {
        eprintln!("[VERBOSE] Generating lucky colors...");
    }
    let lucky_colors = generate_lucky_colors(zodiac_sign, args.verbose);
    
    if args.verbose {
        eprintln!("[VERBOSE] Retrieving ASCII art...");
    }
    let ascii_art = get_zodiac_ascii_art(zodiac_sign, args.verbose);
    
    if args.verbose {
        eprintln!("[VERBOSE] Displaying results...");
        eprintln!("[VERBOSE] Starting horoscope display output");
    }
    
    println!("🌟 Tomorrow's Horoscope for {} 🌟", zodiac_sign);
    if args.verbose {
        eprintln!("[VERBOSE] Displayed header with zodiac sign");
    }
    
    println!();
    println!("{}", ascii_art);
    if args.verbose {
        eprintln!("[VERBOSE] ASCII art displayed");
    }
    
    println!();
    println!("📜 PREDICTION:");
    println!("{}", horoscope);
    if args.verbose {
        eprintln!("[VERBOSE] Horoscope prediction displayed");
    }
    
    println!();
    let numbers_str = lucky_numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
    println!("🍀 LUCKY NUMBERS: {}", numbers_str);
    if args.verbose {
        eprintln!("[VERBOSE] Lucky numbers displayed: {}", numbers_str);
    }
    
    let colors_str = lucky_colors.join(", ");
    println!("🌈 LUCKY COLORS: {}", colors_str);
    if args.verbose {
        eprintln!("[VERBOSE] Lucky colors displayed: {}", colors_str);
    }
    
    println!();
    println!("✨ May the stars guide your path! ✨");
    if args.verbose {
        eprintln!("[VERBOSE] Footer message displayed");
    }
    
    if args.verbose {
        eprintln!("[VERBOSE] Horoscope generation completed successfully!");
    }
}