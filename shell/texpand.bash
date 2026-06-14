# texpand.bash — Terminal Expander Bash plugin
# Source this file from .bashrc:
#   source /path/to/texpand.bash
#
# Type :hello[space] → auto-expands inline
# Type te:hello[Enter] → runs te :hello (no space needed)
# Type te :ticket[Enter] → opens form TUI

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

# Called on Space — expand first, then insert space
_texpand_on_space() {
    _texpand_expand
    READLINE_LINE+=" "
    READLINE_POINT=${#READLINE_LINE}
}

# Allow te:hello (no space) to run te :hello
# Wraps command_not_found_handle — saves original, adds te: prefix handling
if [ -n "$(type -t command_not_found_handle 2>/dev/null)" ]; then
    _texpand_old_cnf=$(type command_not_found_handle 2>/dev/null | tail +3)
fi
command_not_found_handle() {
    if [[ "$1" == te:* ]]; then
        te ":${1#te:}"
        return $?
    fi
    if [ -n "$_texpand_old_cnf" ]; then
        eval "$_texpand_old_cnf" "$@"
    elif [ -x /usr/lib/command-not-found ]; then
        /usr/lib/command-not-found -- "$1"
    else
        echo "bash: $1: command not found" >&2
        return 127
    fi
}

# Bind space → expand, then insert space
bind -x '" ": _texpand_on_space'

# Bind Ctrl+T → manual expand
bind -x '"\C-t": _texpand_expand'
