use crate::{Context, formatting::Color, widgets::Widget};

/// Indicate if the local Git branch is out of sync with the remote.
pub struct GitSync;

impl Widget for GitSync {
    fn content(&self, context: &Context) -> Option<String> {
        if context.git.head_sha()? == context.git.remote_head_sha()? {
            return None;
        }

        Some("⇵".to_string())
    }

    fn color(&self, _context: &Context) -> Color {
        Color::Cyan
    }
}
