
use builtin;
use str;

set edit:completion:arg-completer[tkn-watch] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'tkn-watch'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'tkn-watch'= {
            cand -n 'The namespace scope for this CLI request'
            cand --namespace 'The namespace scope for this CLI request'
            cand -r 'The number of seconds to wait between refreshes'
            cand --refresh-seconds 'The number of seconds to wait between refreshes'
            cand -f 'The json file to read the PipelineRun from instead of running one'
            cand --file 'The json file to read the PipelineRun from instead of running one'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand -l 'Choose the last pipelinerun'
            cand --last 'Choose the last pipelinerun'
        }
    ]
    $completions[$command]
}
