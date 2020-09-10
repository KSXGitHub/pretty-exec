#compdef pretty-exec

autoload -U is-at-least

_pretty-exec() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'--color=[When to use color]: :(auto never always)' \
'--skip-exec[Do not execute, print command only]' \
'--github-actions[Enable GitHub Action grouping]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':program -- Program to execute:_files' \
'::arguments -- Arguments to pass to program:_files' \
&& ret=0
    
}

(( $+functions[_pretty-exec_commands] )) ||
_pretty-exec_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'pretty-exec commands' commands "$@"
}

_pretty-exec "$@"