use chrono::{NaiveDate, Datelike};
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

pub fn determine_zodiac_sign(birth_date: NaiveDate) -> ZodiacSign {
    let month = birth_date.month();
    let day = birth_date.day();

    match (month, day) {
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
    }
}