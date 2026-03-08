use crate::{Context, widgets::Widget};

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
}
