use rand::Rng;
use crate::zodiac::ZodiacSign;

pub fn generate_horoscope(sign: ZodiacSign, verbose: bool) -> String {
    if verbose {
        eprintln!("[VERBOSE horoscope.rs] Generating horoscope for sign: {}", sign);
    }
    
    let mut rng = rand::thread_rng();
    if verbose {
        eprintln!("[VERBOSE horoscope.rs] Random number generator initialized");
    }
    
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
    
    if verbose {
        eprintln!("[VERBOSE horoscope.rs] Found {} predictions for {}", predictions.len(), sign);
    }
    
    let selected_index = rng.gen_range(0..predictions.len());
    if verbose {
        eprintln!("[VERBOSE horoscope.rs] Selected prediction index: {}", selected_index);
    }
    
    let selected_prediction = predictions[selected_index].to_string();
    if verbose {
        eprintln!("[VERBOSE horoscope.rs] Prediction selected successfully");
        eprintln!("[VERBOSE horoscope.rs] Prediction length: {} characters", selected_prediction.len());
    }
    
    selected_prediction
}