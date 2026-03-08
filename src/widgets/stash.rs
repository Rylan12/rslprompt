use crate::{
    Context,
    formatting::{Color, to_superscript},
    widgets::Widget,
};

pub struct Stash;

impl Widget for Stash {
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
