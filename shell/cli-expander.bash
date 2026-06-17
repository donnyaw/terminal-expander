# cli-expander.bash — CLI Expander Bash plugin
# Source this file from .bashrc:
#   source /path/to/cli-expander.bash
#
# Usage:
#   :hello[Space]   → expand inline into the current prompt
#   :find[Space]    → open form TUI, then insert generated command into prompt
#   Ctrl+T          → manually expand the current prompt buffer
#
# Space is the primary workflow. Press Enter only after the expansion is
# visible in the prompt and you are ready to execute it.

_cli_expander_cmd="ce"

_cli_expander_expand() {
    local input="$READLINE_LINE"
    local result
    _cli_expander_has_cursor=0
    result=$($_cli_expander_cmd "$input" 2>/dev/null)
    if [ $? -eq 0 ] && [ -n "$result" ]; then
        # Handle $|$ cursor marker — strip it and position cursor there
        if [[ "$result" == *'$|$'* ]]; then
            local before="${result%%\$|\$*}"
            local after="${result#*\$|\$}"
            READLINE_LINE="$before$after"
            READLINE_POINT=${#before}
            _cli_expander_has_cursor=1
        else
            READLINE_LINE="$result"
            READLINE_POINT=${#READLINE_LINE}
        fi
    fi
}

_cli_expander_on_space() {
    local cursor_before=$READLINE_POINT
    _cli_expander_has_cursor=0
    _cli_expander_expand
    # Insert space at cursor position only if no $|$ marker was handled
    if [ "$_cli_expander_has_cursor" -eq 0 ]; then
        READLINE_LINE="${READLINE_LINE:0:$READLINE_POINT} ${READLINE_LINE:$READLINE_POINT}"
        (( READLINE_POINT++ ))
    fi
}

# Avoid dumping generated command text above the prompt. Use Space/Ctrl+T so the
# expansion lands in the editable command line first.
command_not_found_handle() {
    if [[ "$1" == :* ]]; then
        printf 'cli-expander: use %s[Space] to expand into the prompt, then press Enter to run it.\n' "$1" >&2
        return 127
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

# Ctrl+F → fzf fuzzy search all triggers → select → expand inline
_cli_expander_fzf_search() {
    local selected
    selected=$(ce list --csv 2>/dev/null | fzf \
        --delimiter=',' \
        --with-nth=1,2 \
        --nth=1,2 \
        --preview='ce details {1} 2>/dev/null' \
        --preview-window=right:60%:wrap \
        --header='Enter: expand | Ctrl+T: toggle preview' \
        --bind='ctrl-t:change-preview-window(right,70%|down,5|hidden|)' \
        --height=50% \
        --min-height=10)

    if [ -n "$selected" ]; then
        local trigger
        trigger=$(echo "$selected" | cut -d',' -f1)
        READLINE_LINE="$trigger"
        READLINE_POINT=${#READLINE_LINE}
        _cli_expander_expand
    fi
}

bind -x '" ": _cli_expander_on_space'
bind -x '"\C-t": _cli_expander_expand'
bind -x '"\C-f": _cli_expander_fzf_search'
