complete -c ticli -n "__fish_use_subcommand" -s h -l host -d 'TiKV PD server hostname' -r -f -a "(__fish_print_hostnames)"
complete -c ticli -n "__fish_use_subcommand" -s p -l port -d 'TiKV PD server port' -r
complete -c ticli -n "__fish_use_subcommand" -s m -l mode -d 'TiKV API mode' -r -f -a "{txn	,raw	}"
complete -c ticli -n "__fish_use_subcommand" -l help -d 'Print help information'
complete -c ticli -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c ticli -n "__fish_use_subcommand" -f -a "get" -d 'Get the value of key'
complete -c ticli -n "__fish_use_subcommand" -f -a "set" -d 'Set key to hold the string value'
complete -c ticli -n "__fish_use_subcommand" -f -a "delete" -d 'Delete the specified key'
complete -c ticli -n "__fish_use_subcommand" -f -a "scan" -d 'Scan keys between the range'
complete -c ticli -n "__fish_use_subcommand" -f -a "count" -d 'Count keys between the range'
complete -c ticli -n "__fish_use_subcommand" -f -a "ping" -d 'Return pong when connection is alive'
complete -c ticli -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ticli -n "__fish_seen_subcommand_from scan" -l from -d 'Start key' -r
complete -c ticli -n "__fish_seen_subcommand_from scan" -l to -d 'End Key (included)' -r
complete -c ticli -n "__fish_seen_subcommand_from scan" -s l -l limit -d 'Limit the number of records to scan' -r
complete -c ticli -n "__fish_seen_subcommand_from count" -l from -d 'Start key' -r
complete -c ticli -n "__fish_seen_subcommand_from count" -l to -d 'End Key (included)' -r
complete -c ticli -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from delete; and not __fish_seen_subcommand_from scan; and not __fish_seen_subcommand_from count; and not __fish_seen_subcommand_from ping; and not __fish_seen_subcommand_from help" -f -a "get" -d 'Get the value of key'
complete -c ticli -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from delete; and not __fish_seen_subcommand_from scan; and not __fish_seen_subcommand_from count; and not __fish_seen_subcommand_from ping; and not __fish_seen_subcommand_from help" -f -a "set" -d 'Set key to hold the string value'
complete -c ticli -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from delete; and not __fish_seen_subcommand_from scan; and not __fish_seen_subcommand_from count; and not __fish_seen_subcommand_from ping; and not __fish_seen_subcommand_from help" -f -a "delete" -d 'Delete the specified key'
complete -c ticli -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from delete; and not __fish_seen_subcommand_from scan; and not __fish_seen_subcommand_from count; and not __fish_seen_subcommand_from ping; and not __fish_seen_subcommand_from help" -f -a "scan" -d 'Scan keys between the range'
complete -c ticli -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from delete; and not __fish_seen_subcommand_from scan; and not __fish_seen_subcommand_from count; and not __fish_seen_subcommand_from ping; and not __fish_seen_subcommand_from help" -f -a "count" -d 'Count keys between the range'
complete -c ticli -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from delete; and not __fish_seen_subcommand_from scan; and not __fish_seen_subcommand_from count; and not __fish_seen_subcommand_from ping; and not __fish_seen_subcommand_from help" -f -a "ping" -d 'Return pong when connection is alive'
complete -c ticli -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from delete; and not __fish_seen_subcommand_from scan; and not __fish_seen_subcommand_from count; and not __fish_seen_subcommand_from ping; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
