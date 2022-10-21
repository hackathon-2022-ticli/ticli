complete -c ticli -n "__fish_use_subcommand" -s h -l host -d 'TiKV PD server hostname' -r -f -a "(__fish_print_hostnames)"
complete -c ticli -n "__fish_use_subcommand" -s p -l port -d 'TiKV PD server port' -r
complete -c ticli -n "__fish_use_subcommand" -s m -l mode -d 'TiKV API mode' -r -f -a "{txn	,raw	}"
complete -c ticli -n "__fish_use_subcommand" -l help -d 'Print help information'
complete -c ticli -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c ticli -n "__fish_use_subcommand" -f -a "get" -d 'Get the value of key'
complete -c ticli -n "__fish_use_subcommand" -f -a "getb" -d 'Get the value of key in binary format'
complete -c ticli -n "__fish_use_subcommand" -f -a "set" -d 'Set key to hold the string value'
complete -c ticli -n "__fish_use_subcommand" -f -a "setb" -d 'Set key to hold the binary data from the file'
complete -c ticli -n "__fish_use_subcommand" -f -a "delete" -d 'Delete the specified key'
complete -c ticli -n "__fish_use_subcommand" -f -a "strlen" -d 'Get the length of the bytes stored at key'
complete -c ticli -n "__fish_use_subcommand" -f -a "scan" -d 'Scan keys between the range'
complete -c ticli -n "__fish_use_subcommand" -f -a "count" -d 'Count keys between the range'
complete -c ticli -n "__fish_use_subcommand" -f -a "source" -d 'Execute commands from file'
complete -c ticli -n "__fish_use_subcommand" -f -a "loadcsv" -d 'Load kv records from csv file'
complete -c ticli -n "__fish_use_subcommand" -f -a "ping" -d 'Return pong when connection is alive'
complete -c ticli -n "__fish_use_subcommand" -f -a "quit" -d 'Exit the program'
complete -c ticli -n "__fish_use_subcommand" -f -a "noop" -d 'No Operation'
complete -c ticli -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ticli -n "__fish_seen_subcommand_from get" -s h -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from getb" -s h -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from set" -s h -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from setb" -s h -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from delete" -s h -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from strlen" -s h -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from scan" -l from -d 'Start Key prefix (included)' -r
complete -c ticli -n "__fish_seen_subcommand_from scan" -l to -d 'End Key prefix (included)' -r
complete -c ticli -n "__fish_seen_subcommand_from scan" -s l -l limit -d 'Limit the number of records to scan' -r
complete -c ticli -n "__fish_seen_subcommand_from scan" -s o -l output -d 'Output format' -r -f -a "{auto	,table	,json	,csv	}"
complete -c ticli -n "__fish_seen_subcommand_from scan" -s h -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from count" -l from -d 'Start Key prefix (included)' -r
complete -c ticli -n "__fish_seen_subcommand_from count" -l to -d 'End Key prefix (included)' -r
complete -c ticli -n "__fish_seen_subcommand_from count" -s h -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from source" -s h -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from loadcsv" -s d -l delimiter -d 'Specify the field delimiter' -r
complete -c ticli -n "__fish_seen_subcommand_from loadcsv" -s b -l batch -d 'Specify how many records to write at once' -r
complete -c ticli -n "__fish_seen_subcommand_from loadcsv" -s h -l header -d 'Specify that the input has header row'
complete -c ticli -n "__fish_seen_subcommand_from loadcsv" -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from ping" -s h -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from quit" -s h -l help -d 'Print help information'
complete -c ticli -n "__fish_seen_subcommand_from noop" -s h -l help -d 'Print help information'
