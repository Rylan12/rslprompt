use std::fmt::Display;

const ZERO_WIDTH_BEGIN: &str = "%{";
const ZERO_WIDTH_END: &str = "%}";
const SGR_RESET: &str = "\x1b[0m";

/// A color that can be applied to widget content.
pub enum Color {
    Default,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    White,
    Gray,
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
            Color::White => "\x1b[37m",
            Color::Gray => "\x1b[38;5;240m",
            Color::None => "",
        };
        write!(f, "{}", code)
    }
}

impl Color {
    /// Wraps the given text with zero-width markers and ANSI color escape codes.
    pub fn wrap(&self, text: &str) -> String {
        format!("{}{}{}", self.apply(), text, self.reset())
    }

    fn apply(&self) -> String {
        match self {
            Color::None => String::new(),
            _ => format!("{}{}{}", ZERO_WIDTH_BEGIN, self, ZERO_WIDTH_END),
        }
    }

    fn reset(&self) -> String {
        match self {
            Color::None => String::new(),
            _ => format!("{}{}{}", ZERO_WIDTH_BEGIN, SGR_RESET, ZERO_WIDTH_END),
        }
    }
}
