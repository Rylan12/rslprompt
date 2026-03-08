use crate::{
    Context,
    formatting::{Color, to_superscript},
    widgets::Widget,
};

/// Display the number of stashed changes in the current Git repository.
pub struct GitStash;

impl Widget for GitStash {
    fn content(&self, context: &Context) -> Option<String> {
        match context.git.num_stashes() {
            0 => None,
            n => Some(to_superscript(&n.to_string())),
        }
    }

    fn color(&self, _context: &Context) -> Color {
        Color::White
    }

    fn space_before(&self) -> bool {
        false
    }
}
