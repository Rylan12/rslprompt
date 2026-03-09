use crate::{context::Context, formatting::Color, widgets::Widget};

/// Display the current working directory, using `~` for the home directory if applicable.
pub struct Path;

impl Widget for Path {
    fn content(&self, context: &Context) -> Option<String> {
        // Worldpaths are already formatted correctly
        if let Some(world) = context.world() {
            return Some(world.path().to_string());
        }

        let cwd = context.cwd()?;

        if let Some(home) = context.home_dir()
            && let Ok(relative) = cwd.strip_prefix(home)
        {
            if relative.as_os_str().is_empty() {
                return Some("~".to_string());
            }
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
