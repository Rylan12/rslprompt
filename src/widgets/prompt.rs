use crate::{context::Context, formatting::Color, widgets::Widget};

/// Display the prompt symbol in a color that reflects the exit status of the last command.
pub struct Prompt;

impl Widget for Prompt {
    fn content(&self, context: &Context) -> Option<String> {
        match context.vi_mode() {
            Some(mode) if mode == "i" => Some("❯".to_string()),
            _ => Some("❮".to_string()),
        }
    }

    fn color(&self, context: &Context) -> Color {
        match context.exit_code() {
            Some(0) => Color::Green,
            _ => Color::Red,
        }
    }
}
