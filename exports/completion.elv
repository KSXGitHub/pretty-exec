
use builtin;
use str;

set edit:completion:arg-completer[pretty-exec] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'pretty-exec'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'pretty-exec'= {
            cand --prompt 'Customize the prompt before the command'
            cand --color 'When to use color'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand --skip-exec 'Do not execute, print command only'
            cand --github-actions 'Enable GitHub Action grouping'
        }
    ]
    $completions[$command]
}
