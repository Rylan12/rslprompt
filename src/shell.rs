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

# Track how long the previous command took to run
zmodload zsh/datetime
__ps1_cmd_start=
__ps1_cmd_duration_ms=
function __ps1_cmd_timer_start() {{
    __ps1_cmd_start=$EPOCHREALTIME
}}
function __ps1_cmd_timer_stop() {{
    if [[ -n \"$__ps1_cmd_start\" ]]; then
        local -i elapsed_ms=$(( (EPOCHREALTIME - __ps1_cmd_start) * 1000 ))
        __ps1_cmd_duration_ms=$elapsed_ms
    else
        # No command was run (e.g. an empty line), so don't show a stale duration.
        __ps1_cmd_duration_ms=
    fi
    __ps1_cmd_start=
}}
preexec_functions+=(__ps1_cmd_timer_start)
precmd_functions+=(__ps1_cmd_timer_stop)

# Enable async prompt updates via SIGALRM
TRAPALRM() {{
    # Only reset when ZLE is active (i.e. not while commands are running).
    if [[ -n \"$ZLE_STATE\" ]]; then
        zle reset-prompt
    fi
}}

PROMPT='$(PS1_EXEC_NO=$__ps1_exec_no CMD_DURATION_MS=$__ps1_cmd_duration_ms VI_MODE=$ZVM_MODE EXIT_STATUS=$? SHELL_PID=$$ {})'",
        exec_path()
    )
}
