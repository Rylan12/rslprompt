use crate::{Context, widgets::Widget};

/// Display the current working directory, using `~` for the home directory if applicable.
pub struct HeadRef;

impl Widget for HeadRef {
    fn content(&self, context: &Context) -> Option<String> {
        Some(context.git.head()?.to_string())
    }
}
