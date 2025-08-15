use clap::Parser;

#[derive(Parser)]
#[command(name = "horoscope")]
#[command(about = "Generate tomorrow's horoscope based on your birth date")]
pub struct Args {
    #[arg(help = "Your birth date in YYYY-MM-DD format")]
    pub birth_date: String,
    
    #[arg(short, long, help = "Enable verbose logging output")]
    pub verbose: bool,
}