use crate::{context::Context, formatting::Color, widgets::Widget};

/// Render a prompt string from a list of widgets and the current context.
pub fn render_prompt(widgets: &[&dyn Widget], context: &Context) -> String {
    let mut output = String::new();
    let mut space_allowed = false;

    for widget in widgets {
        let mut rendered = render_widget(*widget, context);

        if space_allowed && widget.space_before() {
            rendered = format!(" {}", rendered);
        }

        output.push_str(&rendered);
        space_allowed = widget.allow_space_after();
    }

    format!("{}{} ", output, Color::reset())
}

fn render_widget(widget: &dyn Widget, context: &Context) -> String {
    let content = widget.content(context).unwrap_or_default();
    let color = widget.color(context);
    format!("{}{}", color.activate(), content)
}
