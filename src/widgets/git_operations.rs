use crate::{
    Context,
    formatting::{Color, to_superscript},
    widgets::Widget,
};

/// Indicate ongoing Git operations (e.g., merge, rebase) using symbolic letters.
pub struct GitOperations;

impl Widget for GitOperations {
    fn content(&self, context: &Context) -> Option<String> {
        if context.git.operations().is_empty() {
            return None;
        }

        let output = context
            .git
            .operations()
            .iter()
            .map(|op| to_superscript(op.symbolic_letter()))
            .collect();

        Some(output)
    }

    fn color(&self, _context: &Context) -> Color {
        Color::Red
    }

    fn allow_space_after(&self) -> bool {
        false
    }
}
