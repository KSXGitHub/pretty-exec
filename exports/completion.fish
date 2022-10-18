complete -c pretty-exec -l prompt -d 'Customize the prompt before the command' -r
complete -c pretty-exec -l color -d 'When to use color' -r -f -a "{auto	,never	,always	}"
complete -c pretty-exec -l skip-exec -d 'Do not execute, print command only'
complete -c pretty-exec -l github-actions -d 'Enable GitHub Action grouping'
complete -c pretty-exec -s h -l help -d 'Print help information'
complete -c pretty-exec -s V -l version -d 'Print version information'
