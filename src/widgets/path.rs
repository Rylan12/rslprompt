use crate::{Context, formatting::Color, widgets::Widget};

/// Display the current working directory, using `~` for the home directory if applicable.
pub struct Path;

impl Widget for Path {
    fn content(&self, context: &Context) -> Option<String> {
        let cwd = context.cwd()?;

        if let Some(home) = context.home_dir()
            && let Ok(relative) = cwd.strip_prefix(home)
        {
            return Some(format!("~/{}", relative.display()));
        }

        Some(cwd.display().to_string())
    }

    fn color(&self, context: &Context) -> Color {
        if context.ssh_connection() {
            Color::Green
        } else {
            Color::Blue
        }
    }
}
