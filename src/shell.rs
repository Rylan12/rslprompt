use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
pub enum Shell {
    Zsh,
}

impl Shell {
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
PROMPT='$(PS1_EXEC_NO=$__ps1_exec_no VI_MODE=$ZVM_MODE EXIT_STATUS=$? SHELL_PID=$$ {})'
function __ps1_exec_incr() {{
    __ps1_exec_no=$((__ps1_exec_no+1))
}}
precmd_functions+=(__ps1_exec_incr)
TRAPALRM() {{
    if [[ -n \"$WIDGET\" ]]; then
        zle reset-prompt
    fi
}}",
        exec_path()
    )
}
