use crate::{Context, formatting::Color, widgets::Widget};

/// Display the current worktree in the world monorepo if applicable.
pub struct WorldWorktree;

impl Widget for WorldWorktree {
    fn content(&self, context: &Context) -> Option<String> {
        Some(format!("+{}", context.world()?.tree()))
    }

    fn color(&self, _context: &Context) -> Color {
        Color::Green
    }

    fn allow_space_after(&self) -> bool {
        false
    }
}
