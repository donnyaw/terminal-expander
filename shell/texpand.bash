# texpand.bash — Terminal Expander Bash plugin
# Source this file from .bashrc:
#   source /path/to/texpand.bash
#
# Usage:
#   :hello[space]   → auto-expands inline (text triggers only)
#   :ticket[Enter]  → opens form TUI (colon triggers command_not_found_handle)
#   te:ticket[Enter] → same as above
#   te :ticket[Enter] → same as above

_texpand_cmd="te"

_texpand_expand() {
    local input="$READLINE_LINE"
    local result
    result=$($_texpand_cmd "$input" 2>/dev/null)
    if [ $? -eq 0 ] && [ -n "$result" ]; then
        READLINE_LINE="$result"
        READLINE_POINT=${#READLINE_LINE}
    fi
}

_texpand_on_space() {
    _texpand_expand
    READLINE_LINE+=" "
    READLINE_POINT=${#READLINE_LINE}
}

# Catch :trigger[Enter] — runs in subshell to isolate Cursive's terminal changes
command_not_found_handle() {
    if [[ "$1" == :* ]]; then
        (te "$1")
        return $?
    fi
    if [ -x /usr/lib/command-not-found ]; then
        /usr/lib/command-not-found -- "$1"
        return $?
    fi
    if [ -x /usr/share/command-not-found/command-not-found ]; then
        /usr/share/command-not-found/command-not-found -- "$1"
        return $?
    fi
    echo "bash: $1: command not found" >&2
    return 127
}

bind -x '" ": _texpand_on_space'
bind -x '"\C-t": _texpand_expand'
