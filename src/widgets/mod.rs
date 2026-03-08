use crate::{context::Context, formatting::Color};

mod exit_status;
mod head_ref;
mod newline;
mod path;
mod prompt;
mod stash;

/// A widget is a component of the prompt that can display content and have a color.
/// Widgets are rendered in order to create the final prompt string.
pub trait Widget {
    /// Returns the content to display for this widget.
    fn content(&self, context: &Context) -> Option<String>;

    /// Returns the color to use for this widget.
    fn color(&self, _context: &Context) -> Color {
        Color::Default
    }

    /// Whether to request a space before this widget when rendering.
    /// Note: a space will only be added if the previous widget sets `allow_space_after()`.
    fn space_before(&self) -> bool {
        true
    }

    /// Whether to allow a space to be placed after this widget when rendering.
    fn allow_space_after(&self) -> bool {
        true
    }
}

pub const DEFAULT_WIDGETS: &[&dyn Widget] = &[
    &newline::Newline,
    &path::Path,
    &head_ref::HeadRef,
    &stash::Stash,
    &exit_status::ExitStatus,
    &newline::Newline,
    &prompt::Prompt,
];
