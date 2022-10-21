
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
            cand scan 'Scan keys between the range'
            cand count 'Count keys between the range'
            cand ping 'Return pong when connection is alive'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'ticli;get'= {
        }
        &'ticli;set'= {
        }
        &'ticli;delete'= {
        }
        &'ticli;scan'= {
            cand --from 'Start key'
            cand --to 'End Key (included)'
            cand -l 'Limit the number of records to scan'
            cand --limit 'Limit the number of records to scan'
        }
        &'ticli;count'= {
            cand --from 'Start key'
            cand --to 'End Key (included)'
        }
        &'ticli;ping'= {
        }
        &'ticli;help'= {
            cand get 'Get the value of key'
            cand set 'Set key to hold the string value'
            cand delete 'Delete the specified key'
            cand scan 'Scan keys between the range'
            cand count 'Count keys between the range'
            cand ping 'Return pong when connection is alive'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'ticli;help;get'= {
        }
        &'ticli;help;set'= {
        }
        &'ticli;help;delete'= {
        }
        &'ticli;help;scan'= {
        }
        &'ticli;help;count'= {
        }
        &'ticli;help;ping'= {
        }
        &'ticli;help;help'= {
        }
    ]
    $completions[$command]
}
