use chrono::{NaiveDate, Datelike};
use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ZodiacSign {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

impl fmt::Display for ZodiacSign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZodiacSign::Aries => write!(f, "Aries"),
            ZodiacSign::Taurus => write!(f, "Taurus"),
            ZodiacSign::Gemini => write!(f, "Gemini"),
            ZodiacSign::Cancer => write!(f, "Cancer"),
            ZodiacSign::Leo => write!(f, "Leo"),
            ZodiacSign::Virgo => write!(f, "Virgo"),
            ZodiacSign::Libra => write!(f, "Libra"),
            ZodiacSign::Scorpio => write!(f, "Scorpio"),
            ZodiacSign::Sagittarius => write!(f, "Sagittarius"),
            ZodiacSign::Capricorn => write!(f, "Capricorn"),
            ZodiacSign::Aquarius => write!(f, "Aquarius"),
            ZodiacSign::Pisces => write!(f, "Pisces"),
        }
    }
}

pub fn determine_zodiac_sign(birth_date: NaiveDate, verbose: bool) -> ZodiacSign {
    let month = birth_date.month();
    let day = birth_date.day();
    
    if verbose {
        eprintln!("[VERBOSE zodiac.rs] Determining zodiac sign for date: {}", birth_date);
        eprintln!("[VERBOSE zodiac.rs] Month: {}, Day: {}", month, day);
    }

    let sign = match (month, day) {
        (3, 21..=31) | (4, 1..=19) => ZodiacSign::Aries,
        (4, 20..=30) | (5, 1..=20) => ZodiacSign::Taurus,
        (5, 21..=31) | (6, 1..=20) => ZodiacSign::Gemini,
        (6, 21..=30) | (7, 1..=22) => ZodiacSign::Cancer,
        (7, 23..=31) | (8, 1..=22) => ZodiacSign::Leo,
        (8, 23..=31) | (9, 1..=22) => ZodiacSign::Virgo,
        (9, 23..=30) | (10, 1..=22) => ZodiacSign::Libra,
        (10, 23..=31) | (11, 1..=21) => ZodiacSign::Scorpio,
        (11, 22..=30) | (12, 1..=21) => ZodiacSign::Sagittarius,
        (12, 22..=31) | (1, 1..=19) => ZodiacSign::Capricorn,
        (1, 20..=31) | (2, 1..=18) => ZodiacSign::Aquarius,
        (2, 19..=29) | (3, 1..=20) => ZodiacSign::Pisces,
        _ => unreachable!("Invalid date"),
    };
    
    if verbose {
        eprintln!("[VERBOSE zodiac.rs] Zodiac sign determined: {}", sign);
    }
    
    sign
}

pub fn generate_lucky_numbers(sign: ZodiacSign, verbose: bool) -> Vec<u8> {
    if verbose {
        eprintln!("[VERBOSE zodiac.rs] Generating lucky numbers for {}", sign);
    }
    
    let mut rng = rand::thread_rng();
    
    let base_numbers = match sign {
        ZodiacSign::Aries => vec![1, 9, 17, 25, 33],
        ZodiacSign::Taurus => vec![2, 6, 14, 22, 30],
        ZodiacSign::Gemini => vec![3, 12, 21, 39, 48],
        ZodiacSign::Cancer => vec![2, 7, 11, 16, 20],
        ZodiacSign::Leo => vec![1, 3, 10, 19, 28],
        ZodiacSign::Virgo => vec![3, 15, 27, 35, 51],
        ZodiacSign::Libra => vec![6, 15, 24, 33, 42],
        ZodiacSign::Scorpio => vec![4, 13, 18, 22, 27],
        ZodiacSign::Sagittarius => vec![9, 18, 27, 36, 45],
        ZodiacSign::Capricorn => vec![8, 10, 26, 35, 44],
        ZodiacSign::Aquarius => vec![4, 8, 13, 17, 22],
        ZodiacSign::Pisces => vec![7, 12, 16, 21, 25],
    };
    
    if verbose {
        eprintln!("[VERBOSE zodiac.rs] Base numbers selected: {:?}", base_numbers);
    }
    
    let mut lucky_numbers = base_numbers;
    for (i, num) in lucky_numbers.iter_mut().enumerate() {
        let original = *num;
        *num = (*num + rng.gen_range(0..5)) % 50 + 1;
        if verbose {
            eprintln!("[VERBOSE zodiac.rs] Lucky number {}: {} -> {}", i + 1, original, *num);
        }
    }
    
    if verbose {
        eprintln!("[VERBOSE zodiac.rs] Final lucky numbers: {:?}", lucky_numbers);
    }
    
    lucky_numbers
}

pub fn generate_lucky_colors(sign: ZodiacSign, verbose: bool) -> Vec<&'static str> {
    if verbose {
        eprintln!("[VERBOSE zodiac.rs] Generating lucky colors for {}", sign);
    }
    
    let mut rng = rand::thread_rng();
    
    let colors = match sign {
        ZodiacSign::Aries => vec!["Red", "Orange", "Crimson", "Scarlet"],
        ZodiacSign::Taurus => vec!["Green", "Pink", "Emerald", "Rose Gold"],
        ZodiacSign::Gemini => vec!["Yellow", "Silver", "Bright Blue", "Turquoise"],
        ZodiacSign::Cancer => vec!["White", "Silver", "Sea Blue", "Moonstone"],
        ZodiacSign::Leo => vec!["Gold", "Orange", "Yellow", "Amber"],
        ZodiacSign::Virgo => vec!["Navy Blue", "Brown", "Forest Green", "Beige"],
        ZodiacSign::Libra => vec!["Blue", "Pink", "Lavender", "Pastel Green"],
        ZodiacSign::Scorpio => vec!["Deep Red", "Black", "Maroon", "Dark Purple"],
        ZodiacSign::Sagittarius => vec!["Purple", "Turquoise", "Royal Blue", "Violet"],
        ZodiacSign::Capricorn => vec!["Black", "Brown", "Dark Green", "Grey"],
        ZodiacSign::Aquarius => vec!["Electric Blue", "Silver", "Aqua", "Neon Green"],
        ZodiacSign::Pisces => vec!["Sea Green", "Lavender", "Aquamarine", "Soft Blue"],
    };
    
    if verbose {
        eprintln!("[VERBOSE zodiac.rs] Available colors for {}: {:?}", sign, colors);
    }
    
    let mut selected = vec![];
    let num_colors = rng.gen_range(2..=3);
    if verbose {
        eprintln!("[VERBOSE zodiac.rs] Selecting {} colors", num_colors);
    }
    
    let mut available_colors = colors;
    
    for i in 0..num_colors {
        if !available_colors.is_empty() {
            let idx = rng.gen_range(0..available_colors.len());
            let color = available_colors.remove(idx);
            if verbose {
                eprintln!("[VERBOSE zodiac.rs] Color {}: selected '{}'", i + 1, color);
            }
            selected.push(color);
        }
    }
    
    if verbose {
        eprintln!("[VERBOSE zodiac.rs] Final lucky colors: {:?}", selected);
    }
    
    selected
}

