
use builtin;
use str;

set edit:completion:arg-completer[ticli] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'ticli'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'ticli'= {
            cand -h 'TiKV PD server hostname'
            cand --host 'TiKV PD server hostname'
            cand -p 'TiKV PD server port'
            cand --port 'TiKV PD server port'
            cand -m 'TiKV API mode'
            cand --mode 'TiKV API mode'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand get 'Get the value of key'
            cand set 'Set key to hold the string value'
            cand delete 'Delete the specified key'
            cand strlen 'Get the length of the bytes stored at key'
            cand scan 'Scan keys between the range'
            cand count 'Count keys between the range'
            cand source 'Execute commands from file'
            cand ping 'Return pong when connection is alive'
            cand quit 'Exit the program'
            cand noop 'No Operation'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'ticli;get'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;set'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;delete'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;strlen'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;scan'= {
            cand --from 'Start Key prefix (included)'
            cand --to 'End Key prefix (included)'
            cand -l 'Limit the number of records to scan'
            cand --limit 'Limit the number of records to scan'
            cand -o 'Output format'
            cand --output 'Output format'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;count'= {
            cand --from 'Start Key prefix (included)'
            cand --to 'End Key prefix (included)'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;source'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;ping'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;quit'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;noop'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;help'= {
        }
    ]
    $completions[$command]
}
