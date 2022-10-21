
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
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set key to hold the string value')
            [CompletionResult]::new('delete', 'delete', [CompletionResultType]::ParameterValue, 'Delete the specified key')
            [CompletionResult]::new('strlen', 'strlen', [CompletionResultType]::ParameterValue, 'Get the length of the bytes stored at key')
            [CompletionResult]::new('scan', 'scan', [CompletionResultType]::ParameterValue, 'Scan keys between the range')
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'Count keys between the range')
            [CompletionResult]::new('source', 'source', [CompletionResultType]::ParameterValue, 'Execute commands from file')
            [CompletionResult]::new('ping', 'ping', [CompletionResultType]::ParameterValue, 'Return pong when connection is alive')
            [CompletionResult]::new('noop', 'noop', [CompletionResultType]::ParameterValue, 'No Operation')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'ticli;get' {
            break
        }
        'ticli;set' {
            break
        }
        'ticli;delete' {
            break
        }
        'ticli;strlen' {
            break
        }
        'ticli;scan' {
            [CompletionResult]::new('--from', 'from', [CompletionResultType]::ParameterName, 'Start key')
            [CompletionResult]::new('--to', 'to', [CompletionResultType]::ParameterName, 'End Key (included)')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Limit the number of records to scan')
            [CompletionResult]::new('--limit', 'limit', [CompletionResultType]::ParameterName, 'Limit the number of records to scan')
            break
        }
        'ticli;count' {
            [CompletionResult]::new('--from', 'from', [CompletionResultType]::ParameterName, 'Start key')
            [CompletionResult]::new('--to', 'to', [CompletionResultType]::ParameterName, 'End Key (included)')
            break
        }
        'ticli;source' {
            break
        }
        'ticli;ping' {
            break
        }
        'ticli;noop' {
            break
        }
        'ticli;help' {
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Get the value of key')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set key to hold the string value')
            [CompletionResult]::new('delete', 'delete', [CompletionResultType]::ParameterValue, 'Delete the specified key')
            [CompletionResult]::new('strlen', 'strlen', [CompletionResultType]::ParameterValue, 'Get the length of the bytes stored at key')
            [CompletionResult]::new('scan', 'scan', [CompletionResultType]::ParameterValue, 'Scan keys between the range')
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'Count keys between the range')
            [CompletionResult]::new('source', 'source', [CompletionResultType]::ParameterValue, 'Execute commands from file')
            [CompletionResult]::new('ping', 'ping', [CompletionResultType]::ParameterValue, 'Return pong when connection is alive')
            [CompletionResult]::new('noop', 'noop', [CompletionResultType]::ParameterValue, 'No Operation')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'ticli;help;get' {
            break
        }
        'ticli;help;set' {
            break
        }
        'ticli;help;delete' {
            break
        }
        'ticli;help;strlen' {
            break
        }
        'ticli;help;scan' {
            break
        }
        'ticli;help;count' {
            break
        }
        'ticli;help;source' {
            break
        }
        'ticli;help;ping' {
            break
        }
        'ticli;help;noop' {
            break
        }
        'ticli;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
