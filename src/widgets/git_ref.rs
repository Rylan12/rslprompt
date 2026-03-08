use crate::{Context, widgets::Widget};

/// Display the current working directory, using `~` for the home directory if applicable.
pub struct GitRef;

impl Widget for GitRef {
    fn content(&self, context: &Context) -> Option<String> {
        if !context.git.is_git_repo() {
            return None;
        }

        let Some(head) = context.git.head() else {
            return Some("???".to_string());
        };

        Some(head.to_string())
    }
}
