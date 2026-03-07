use crate::{Context, formatting::Color, widgets::Widget};

/// Display the current working directory, using `~` for the home directory if applicable.
pub struct Path;

impl Widget for Path {
    fn content(&self, context: &Context) -> Option<String> {
        let Some(cwd) = context.cwd() else {
            return Some(String::from("?"));
        };

        let Some(home) = context.home_dir() else {
            return Some(cwd);
        };

        if home.is_empty() || !cwd.starts_with(&home) {
            return Some(cwd);
        }

        Some(format!("~{}", &cwd[home.len()..]))
    }

    fn color(&self, context: &Context) -> Color {
        if context.ssh_connection() {
            Color::Green
        } else {
            Color::Blue
        }
    }
}
