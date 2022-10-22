
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
            cand -s 'Specify the output table style'
            cand --style 'Specify the output table style'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand get 'Get the value of key'
            cand getb 'Get the value of key in binary format'
            cand set 'Set key to hold the string value'
            cand setb 'Set key to hold the binary data from the file'
            cand incr 'Increments the number stored at key by one'
            cand incrby 'Increments the number stored at key by increment'
            cand decr 'Decrements the number stored at key by one'
            cand decrby 'Decrements the number stored at key by decrement'
            cand delete 'Delete the specified key'
            cand strlen 'Get the length of the bytes stored at key'
            cand exists 'Returns if key exists'
            cand scan 'Scan keys between the range'
            cand count 'Count keys between the range'
            cand source 'Execute commands from file'
            cand loadcsv 'Load kv records from csv file'
            cand flushall 'Remove all keys from tikv'
            cand ping 'Return pong when connection is alive'
            cand style 'Specify the output table style'
            cand quit 'Exit the program'
            cand noop 'No Operation'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'ticli;get'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;getb'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;set'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;setb'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;incr'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;incrby'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;decr'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;decrby'= {
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
        &'ticli;exists'= {
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
        &'ticli;loadcsv'= {
            cand -d 'Specify the field delimiter'
            cand --delimiter 'Specify the field delimiter'
            cand -b 'Specify how many records to write at once'
            cand --batch 'Specify how many records to write at once'
            cand -h 'Specify that the input has header row'
            cand --header 'Specify that the input has header row'
            cand --help 'Print help information'
        }
        &'ticli;flushall'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;ping'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ticli;style'= {
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
