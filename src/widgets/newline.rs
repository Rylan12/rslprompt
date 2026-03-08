use crate::widgets::Widget;

/// Render a newline character.
pub struct Newline;

impl Widget for Newline {
    fn content(&self, _context: &crate::context::Context) -> Option<String> {
        Some("\n".to_string())
    }

    fn space_before(&self) -> bool {
        false
    }

    fn allow_space_after(&self) -> bool {
        false
    }
}
