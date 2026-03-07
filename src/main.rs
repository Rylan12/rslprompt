use rprompt::{Context, DEFAULT_WIDGETS, render_prompt};

fn main() {
    let context = Context::new();
    let output = render_prompt(DEFAULT_WIDGETS, &context);
    println!("{}", output);
}
