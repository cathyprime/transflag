macro_rules! flag {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {{
        vec![$a, $b, $c, $d, $e]
    }};
}

macro_rules! _ansi {
    ($a:expr) => {
        Ansi::new("t4tt4tt4tt4tt4tt4tt4t".to_string(), $a)
    };
}

macro_rules! color {
    ($($el:expr) +) => {{
        let tup = ($($el),*);
        _ansi!(Color {
            red: tup.0 as u8,
            green: tup.1 as u8,
            blue: tup.2 as u8,
        })
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        _ansi!(Color {
            red: $a as u8,
            green: $b as u8,
            blue: $c as u8,
        })
    }};
}

fn main() {
    let transgender = flag!(
        color!(91 206 250),
        color!(245 169 184),
        color!(255 255 255),
        color!(245 169 184),
        color!(91 206 250)
    );
    let lesbian = flag!(
        color!(213, 45, 0),
        color!(239, 118, 39),
        color!(255 255 255),
        color!(181, 86, 144),
        color!(163, 2, 98)
    );
    transgender
        .into_iter()
        .zip(lesbian)
        .map(|(t, l)| format!("{}    {}", t, l))
        .for_each(|x| println!("{}", x));
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Ansi {
    text: String,
    color: Color,
}

impl std::fmt::Display for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let foreground = format!(
            "\x1b[38;2;{};{};{}m",
            self.color.red, self.color.green, self.color.blue
        );
        let background = format!(
            "\x1b[48;2;{};{};{}m",
            self.color.red, self.color.green, self.color.blue
        );
        write!(f, "{}", foreground + &background + &self.text + "\x1b[0m")
    }
}

impl Ansi {
    pub(crate) fn new(text: String, color: Color) -> Self {
        Ansi { text, color }
    }
}
