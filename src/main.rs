use chrono::{NaiveDate, Datelike};
use clap::Parser;
use rand::Rng;
use std::fmt;

#[derive(Parser)]
#[command(name = "horoscope")]
#[command(about = "Generate tomorrow's horoscope based on your birth date")]
struct Args {
    #[arg(help = "Your birth date in YYYY-MM-DD format")]
    birth_date: String,
}

#[derive(Debug, Clone, Copy)]
enum ZodiacSign {
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

fn determine_zodiac_sign(birth_date: NaiveDate) -> ZodiacSign {
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

fn generate_lucky_numbers(sign: ZodiacSign) -> Vec<u8> {
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
    
    let mut lucky_numbers = base_numbers;
    for num in lucky_numbers.iter_mut() {
        *num = (*num + rng.gen_range(0..5)) % 50 + 1;
    }
    
    lucky_numbers
}

fn generate_lucky_colors(sign: ZodiacSign) -> Vec<&'static str> {
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
    
    let mut selected = vec![];
    let num_colors = rng.gen_range(2..=3);
    let mut available_colors = colors;
    
    for _ in 0..num_colors {
        if !available_colors.is_empty() {
            let idx = rng.gen_range(0..available_colors.len());
            selected.push(available_colors.remove(idx));
        }
    }
    
    selected
}

fn get_zodiac_ascii_art(sign: ZodiacSign) -> &'static str {
    match sign {
        ZodiacSign::Aries => r#"    /|   /|  
   (  @v@  )
    |  _  |
    -'---'-
      RAM"#,
        ZodiacSign::Taurus => r#"  (   )   (   )
   ) _   '-'   _ (
  ( |o|       |o| )
 _)  |  .---. |  (_
(  `'|_/ \_/ \|'`  )
 `-._/ BULL  \_.-'"#,
        ZodiacSign::Gemini => r#"  .-''-.  .-''-.
 /  _    \/    _ \
| (o)\  /\  /(o) |
 \    /  \/  \    /
  `'-|  TWINS |-'`
     \  ____  /
      `------'"#,
        ZodiacSign::Cancer => r#"    .--.
   /    \
  | (oo) |
   \    /
 .-'    '-.
/  CANCER  \
\  _    _  /
 `'------'`"#,
        ZodiacSign::Leo => r#"    ,-'''-,
   /  ___  \
  /  (   )  \
 |    \_/    |
  \   LEO   /
   `.     .'
     `---'"#,
        ZodiacSign::Virgo => r#"      .-.   .-.
     /   \ /   \
    | (M) | (V) |
     \   / \   /
      `-'   `-'
      VIRGO"#,
        ZodiacSign::Libra => r#"    ___===___
   /           \
  |    LIBRA    |
  |   -------   |
   \___________/
      ||||||||
      --------"#,
        ZodiacSign::Scorpio => r#"      /~\
     /   \
    |  o  |
    |  _  |
 /~\ \___/ /~\
(   ) === (   )
 \~/SCORPIO\~/
    \_____/"#,
        ZodiacSign::Sagittarius => r#"    \
     \
      \  * 
  -----\--> *
        \
         \
    SAGITTARIUS"#,
        ZodiacSign::Capricorn => r#"      /^\
     /   \
    | (o) |
    |  v  |
 /~\_|   |_/~\
(   CAPRI-   )
 \_CORN___/"#,
        ZodiacSign::Aquarius => r#"   ~~~~~~
  ~AQUARIUS~
   ~~~~~~~~
     |||||
     |||||
     ~~~~~"#,
        ZodiacSign::Pisces => r#"    ><>      ><>
   /   \    /   \
  < (o) >< (o) >
   \___/    \___/
     PISCES"#,
    }
}

fn generate_horoscope(sign: ZodiacSign) -> String {
    let mut rng = rand::thread_rng();
    
    let predictions = match sign {
        ZodiacSign::Aries => vec![
            "Tomorrow brings unexpected opportunities in your career. Trust your instincts and take bold action.",
            "A conversation with an old friend will spark new inspiration. Your energy will be contagious.",
            "Financial matters require careful attention tomorrow. A small investment could yield surprising returns.",
            "Your leadership qualities will shine through a challenging situation. Others will look to you for guidance.",
            "Adventure calls tomorrow! Say yes to spontaneous plans that come your way.",
        ],
        ZodiacSign::Taurus => vec![
            "Stability and comfort are your themes tomorrow. Focus on building lasting foundations.",
            "A practical solution to a longstanding problem will present itself. Trust your methodical approach.",
            "Romance may bloom in unexpected places. Keep your heart open to new connections.",
            "Your patience will be rewarded with tangible results. Slow and steady wins the race.",
            "Material pleasures bring joy tomorrow. Treat yourself to something beautiful.",
        ],
        ZodiacSign::Gemini => vec![
            "Communication is key tomorrow. Your words will have more impact than usual.",
            "Multiple opportunities present themselves. Your adaptability will be your greatest asset.",
            "Learning something new will open doors you didn't know existed. Stay curious!",
            "Social connections flourish tomorrow. A networking opportunity could change everything.",
            "Your wit and charm will help you navigate a tricky situation with ease.",
        ],
        ZodiacSign::Cancer => vec![
            "Family and home take center stage tomorrow. Nurturing others brings unexpected rewards.",
            "Your intuition is especially strong. Trust those gut feelings about people and situations.",
            "Emotional healing occurs through creative expression. Don't be afraid to show vulnerability.",
            "A childhood memory resurfaces with new meaning. Past experiences illuminate present choices.",
            "Caring for others' needs also fulfills your own emotional well-being tomorrow.",
        ],
        ZodiacSign::Leo => vec![
            "The spotlight finds you tomorrow, whether you seek it or not. Embrace your moment to shine.",
            "Creative projects receive recognition and praise. Your artistic vision inspires others.",
            "Generosity of spirit brings unexpected returns. What you give freely comes back tenfold.",
            "Leadership opportunities arise naturally. Others recognize your natural charisma and ability.",
            "Romance and passion ignite tomorrow. Your heart leads the way to beautiful experiences.",
        ],
        ZodiacSign::Virgo => vec![
            "Attention to detail pays off handsomely tomorrow. Your meticulous work gets noticed.",
            "Health and wellness improvements show real results. Your disciplined approach is working.",
            "Organization brings clarity to a confusing situation. Your systematic method solves problems.",
            "Service to others fills your heart with purpose. Small acts of kindness create big ripples.",
            "Practical wisdom guides you to make the right choices. Trust your analytical mind.",
        ],
        ZodiacSign::Libra => vec![
            "Balance and harmony restore themselves in a key relationship. Peace replaces conflict.",
            "Beauty surrounds you tomorrow in unexpected forms. Your aesthetic sense guides important decisions.",
            "Diplomacy and fairness help you mediate a dispute successfully. Justice prevails through your efforts.",
            "Partnership opportunities present themselves. Collaboration leads to mutual success and satisfaction.",
            "Your natural charm opens doors that seemed permanently closed. Grace under pressure impresses.",
        ],
        ZodiacSign::Scorpio => vec![
            "Hidden truths come to light tomorrow, revealing important information you needed to know.",
            "Transformation begins with a small but significant change. Embrace the process of renewal.",
            "Your magnetic personality draws influential people into your orbit. Powerful alliances form.",
            "Intuitive insights guide you to make profitable decisions. Trust your deeper knowing.",
            "Passion projects take on new life and energy. Your intensity becomes your superpower.",
        ],
        ZodiacSign::Sagittarius => vec![
            "Adventure beckons from distant horizons. Travel or learning expands your worldview dramatically.",
            "Your optimism is infectious and inspires others to dream bigger. Share your vision freely.",
            "Higher education or spiritual pursuits offer profound insights. Seek wisdom from unexpected sources.",
            "Freedom from old restrictions allows you to explore new territories. Break free from limitations.",
            "Your philosophical approach to problems reveals elegant solutions others missed completely.",
        ],
        ZodiacSign::Capricorn => vec![
            "Hard work and discipline finally pay off with tangible recognition. Your persistence was worth it.",
            "Authority figures take notice of your reliability and competence. Advancement opportunities appear.",
            "Long-term planning shows its wisdom as events unfold exactly as you predicted.",
            "Tradition and innovation find perfect balance in your approach. Respect the past, embrace the future.",
            "Your reputation for excellence opens doors to prestigious opportunities tomorrow.",
        ],
        ZodiacSign::Aquarius => vec![
            "Innovation and originality set you apart from the crowd. Your unique perspective is valued.",
            "Humanitarian causes capture your attention and energy. Making a difference feels deeply fulfilling.",
            "Technology plays a surprising role in solving a personal problem. Embrace digital solutions.",
            "Friendship evolves into something deeper and more meaningful. Social connections transform your life.",
            "Your vision of the future inspires others to join your cause. Revolutionary ideas gain momentum.",
        ],
        ZodiacSign::Pisces => vec![
            "Intuition and dreams provide guidance for important decisions. Pay attention to subtle messages.",
            "Compassion and empathy create healing in damaged relationships. Your sensitivity is a gift.",
            "Creative inspiration flows freely tomorrow. Artistic pursuits bring both joy and potential profit.",
            "Spiritual practices offer profound comfort and direction. Connect with your deeper nature.",
            "Your ability to see the best in others transforms a difficult situation into something beautiful.",
        ],
    };
    
    predictions[rng.gen_range(0..predictions.len())].to_string()
}

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