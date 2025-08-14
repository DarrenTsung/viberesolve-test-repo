use crate::zodiac::ZodiacSign;

pub fn get_zodiac_ascii_art(sign: ZodiacSign, verbose: bool) -> &'static str {
    if verbose {
        eprintln!("[VERBOSE ascii_art.rs] Retrieving ASCII art for {}", sign);
    }
    
    let art = match sign {
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
    };
    
    if verbose {
        eprintln!("[VERBOSE ascii_art.rs] ASCII art retrieved successfully");
        let lines_count = art.lines().count();
        eprintln!("[VERBOSE ascii_art.rs] Art contains {} lines", lines_count);
    }
    
    art
}