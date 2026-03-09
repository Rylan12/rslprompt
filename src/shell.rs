use clap::ValueEnum;

/// Supported shell types for prompt configuration.
#[derive(Clone, ValueEnum)]
pub enum Shell {
    Zsh,
}

impl Shell {
    /// Generate the initialization script for this shell type.
    pub fn init_config(&self) -> String {
        match self {
            Shell::Zsh => zsh_config(),
        }
    }
}

fn exec_path() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(String::from))
        .unwrap_or_else(|| "rslprompt".to_string())
}

fn zsh_config() -> String {
    format!(
        "# rslprompt zsh setup
set -o promptsubst

# Track command execution count
__ps1_exec_no=0
function __ps1_exec_incr() {{
    __ps1_exec_no=$((__ps1_exec_no+1))
}}
precmd_functions+=(__ps1_exec_incr)

# Enable async prompt updates via SIGALRM
TRAPALRM() {{
    # Only reset when ZLE is active (i.e. not while commands are running).
    if [[ -n \"$ZLE_STATE\" ]]; then
        zle reset-prompt
    fi
}}

PROMPT='$(PS1_EXEC_NO=$__ps1_exec_no VI_MODE=$ZVM_MODE EXIT_STATUS=$? SHELL_PID=$$ {})'",
        exec_path()
    )
}
