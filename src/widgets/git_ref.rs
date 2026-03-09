use crate::{
    context::{AsyncValue, Context, GitStatus},
    formatting::Color,
    widgets::Widget,
};

/// Display the current working directory, using `~` for the home directory if applicable.
pub struct GitRef;

impl Widget for GitRef {
    fn content(&self, context: &Context) -> Option<String> {
        let git = context.git()?;

        if let Some(head_ref) = git.head_ref() {
            return Some(head_ref.to_string());
        }

        if let Some(head_sha) = git.head_sha() {
            return Some(head_sha.chars().take(7).collect());
        }

        Some("???".to_string())
    }

    fn color(&self, context: &Context) -> Color {
        let Some(git) = context.git() else {
            return Color::Default;
        };

        match git.status() {
            AsyncValue::Ready(Some(GitStatus::Dirty)) => Color::Magenta,
            AsyncValue::Ready(Some(GitStatus::Clean)) => Color::Green,
            _ => Color::Default,
        }
    }
}
