use crate::{Context, formatting::Color, widgets::Widget};

/// Display the exit code of the last command if it was non-zero.
pub struct ExitStatus;

impl Widget for ExitStatus {
    fn content(&self, context: &Context) -> Option<String> {
        context
            .exit_status()
            .filter(|&status| status != 0)
            .map(|status| status.to_string())
    }

    fn color(&self, _context: &Context) -> Color {
        Color::Red
    }
}
