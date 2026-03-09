use crate::{
    Context,
    context::{AsyncValue, GitStatus},
    formatting::Color,
    widgets::Widget,
};

/// Display the current working directory, using `~` for the home directory if applicable.
pub struct GitRef;

impl Widget for GitRef {
    fn content(&self, context: &Context) -> Option<String> {
        if !context.git.is_git_repo() {
            return None;
        }

        if let Some(head_ref) = context.git.head_ref() {
            return Some(head_ref.to_string());
        }

        if let Some(head_sha) = context.git.head_sha() {
            return Some(head_sha.chars().take(7).collect());
        }

        Some("???".to_string())
    }

    fn color(&self, context: &Context) -> Color {
        match context.git.status() {
            AsyncValue::Ready(Some(GitStatus::Dirty)) => Color::Magenta,
            AsyncValue::Ready(Some(GitStatus::Clean)) => Color::Green,
            _ => Color::Default,
        }
    }
}
