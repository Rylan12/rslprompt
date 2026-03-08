use std::fmt::Display;

const ZERO_WIDTH_BEGIN: &str = "%{";
const ZERO_WIDTH_END: &str = "%}";
const SGR_RESET: &str = "\x1b[0m";

const SUPERSCRIPT_DIGITS: &str = "⁰¹²³⁴⁵⁶⁷⁸⁹";
const SUPERSCRIPT_CAPITALS: &str = "ᴬᴮᶜᴰᴱꟳᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾꟴᴿˢᵀᵁⱽᵂ   ";
const SUPERSCRIPT_LOWERCASE: &str = "ᵃᵇᶜᵈᵉᶠᵍʰⁱʲᵏˡᵐⁿᵒᵖ ʳˢᵗᵘᵛʷˣʸᶻ";

pub fn to_superscript(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_digit() {
                SUPERSCRIPT_DIGITS
                    .chars()
                    .nth(c.to_digit(10).unwrap() as usize)
                    .unwrap_or(c)
            } else if c.is_ascii_uppercase() {
                SUPERSCRIPT_CAPITALS
                    .chars()
                    .nth((c as u8 - b'A') as usize)
                    .unwrap_or(c)
            } else if c.is_ascii_lowercase() {
                SUPERSCRIPT_LOWERCASE
                    .chars()
                    .nth((c as u8 - b'a') as usize)
                    .unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}

/// A color that can be applied to widget content.
pub enum Color {
    Default,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    /// Don't apply any color codes to inherit the previous color settings.
    None,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Color::Default => SGR_RESET,
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::None => "",
        };
        write!(f, "{}", code)
    }
}

impl Color {
    /// Return the ANSI escape codes to activate the given color.
    pub fn activate(&self) -> String {
        match self {
            Color::None => String::new(),
            _ => format!("{}{}{}", ZERO_WIDTH_BEGIN, self, ZERO_WIDTH_END),
        }
    }
}
