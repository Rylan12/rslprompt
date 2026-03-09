use clap::{Parser, Subcommand};

use rslprompt::{Context, DEFAULT_WIDGETS, Shell, render_prompt};

#[derive(Parser)]
#[command(name = "rslprompt")]
#[command(about = "Rylan Polster's shell prompt", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize shell configuration.
    Init {
        /// Shell type to configure.
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { shell }) => println!("{}", shell.init_config()),
        None => {
            let context = Context::new();
            let output = render_prompt(DEFAULT_WIDGETS, &context);
            println!("{}", output);
        }
    }
}
