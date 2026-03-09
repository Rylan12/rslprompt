use crate::{context::Context, formatting::Color, widgets::Widget};

/// Render a prompt string from a list of widgets and the current context.
pub fn render_prompt(widgets: &[&dyn Widget], context: &Context) -> String {
    let mut output = String::new();
    let mut space_allowed = false;

    for widget in widgets {
        let Some(content) = widget.content(context) else {
            continue;
        };

        let color = widget.color(context);
        let rendered = format!("{}{}", color.activate(), content);

        if space_allowed && widget.space_before() {
            output.push(' ');
        }
        output.push_str(&rendered);
        space_allowed = widget.allow_space_after();
    }

    format!("{}{} ", output, Color::reset())
}
