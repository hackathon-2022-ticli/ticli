
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'ticli' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'ticli'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'ticli' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'TiKV PD server hostname')
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'TiKV PD server hostname')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'TiKV PD server port')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'TiKV PD server port')
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, 'TiKV API mode')
            [CompletionResult]::new('--mode', 'mode', [CompletionResultType]::ParameterName, 'TiKV API mode')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Get the value of key')
            [CompletionResult]::new('getb', 'getb', [CompletionResultType]::ParameterValue, 'Get the value of key in binary format')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set key to hold the string value')
            [CompletionResult]::new('setb', 'setb', [CompletionResultType]::ParameterValue, 'Set key to hold the binary data from the file')
            [CompletionResult]::new('delete', 'delete', [CompletionResultType]::ParameterValue, 'Delete the specified key')
            [CompletionResult]::new('strlen', 'strlen', [CompletionResultType]::ParameterValue, 'Get the length of the bytes stored at key')
            [CompletionResult]::new('exists', 'exists', [CompletionResultType]::ParameterValue, 'Returns if key exists')
            [CompletionResult]::new('scan', 'scan', [CompletionResultType]::ParameterValue, 'Scan keys between the range')
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'Count keys between the range')
            [CompletionResult]::new('source', 'source', [CompletionResultType]::ParameterValue, 'Execute commands from file')
            [CompletionResult]::new('loadcsv', 'loadcsv', [CompletionResultType]::ParameterValue, 'Load kv records from csv file')
            [CompletionResult]::new('flushall', 'flushall', [CompletionResultType]::ParameterValue, 'Remove all keys from tikv')
            [CompletionResult]::new('ping', 'ping', [CompletionResultType]::ParameterValue, 'Return pong when connection is alive')
            [CompletionResult]::new('quit', 'quit', [CompletionResultType]::ParameterValue, 'Exit the program')
            [CompletionResult]::new('noop', 'noop', [CompletionResultType]::ParameterValue, 'No Operation')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'ticli;get' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;getb' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;set' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;setb' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;delete' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;strlen' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;exists' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;scan' {
            [CompletionResult]::new('--from', 'from', [CompletionResultType]::ParameterName, 'Start Key prefix (included)')
            [CompletionResult]::new('--to', 'to', [CompletionResultType]::ParameterName, 'End Key prefix (included)')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Limit the number of records to scan')
            [CompletionResult]::new('--limit', 'limit', [CompletionResultType]::ParameterName, 'Limit the number of records to scan')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;count' {
            [CompletionResult]::new('--from', 'from', [CompletionResultType]::ParameterName, 'Start Key prefix (included)')
            [CompletionResult]::new('--to', 'to', [CompletionResultType]::ParameterName, 'End Key prefix (included)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;source' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;loadcsv' {
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Specify the field delimiter')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'Specify the field delimiter')
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'Specify how many records to write at once')
            [CompletionResult]::new('--batch', 'batch', [CompletionResultType]::ParameterName, 'Specify how many records to write at once')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Specify that the input has header row')
            [CompletionResult]::new('--header', 'header', [CompletionResultType]::ParameterName, 'Specify that the input has header row')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;flushall' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;ping' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;quit' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;noop' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ticli;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
