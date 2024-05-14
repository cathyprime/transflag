macro_rules! color {
    ($a:expr) => {
        Ansi::new("t4tt4tt4tt4tt4tt4tt4t".to_string(), $a)
    };
    ($($el:expr) +) => {{
        let tup = ($($el),*);
        Color {
            red: tup.0 as u8,
            green: tup.1 as u8,
            blue: tup.2 as u8,
        }
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        Color {
            red: $a as u8,
            green: $b as u8,
            blue: $c as u8,
        }
    }};
}

fn main() {
    let pink_line        = color!(color!(245 169 184));
    let blue_line        = color!(color!(91 206 250));
    let white_line       = color!(color!(255 255 255));
    let dark_orange_line = color!(color!(213, 45, 0));
    let orange_line      = color!(color!(239, 118, 39));
    let dust_pink_line   = color!(color!(181, 86, 144));
    let dark_rose_line   = color!(color!(163, 2, 98));
    println!("{}  {}", blue_line, dark_orange_line);
    println!("{}  {}", pink_line, orange_line);
    println!("{}  {}", white_line, white_line);
    println!("{}  {}", pink_line, dust_pink_line);
    println!("{}  {}", blue_line, dark_rose_line);
}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct Ansi {
    text: String,
    color: Color
}

impl std::fmt::Display for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let foreground = format!("\x1b[38;2;{};{};{}m", self.color.red, self.color.green, self.color.blue);
        let background = format!("\x1b[48;2;{};{};{}m", self.color.red, self.color.green, self.color.blue);
        write!(f, "{}", foreground + &background + &self.text + "\x1b[0m")
    }
}

impl Ansi {
    pub(crate) fn new(text: String, color: Color) -> Self {
        Ansi { text, color }
    }
}
