
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'tkn-watch' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'tkn-watch'
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
        'tkn-watch' {
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'The namespace scope for this CLI request')
            [CompletionResult]::new('--namespace', 'namespace', [CompletionResultType]::ParameterName, 'The namespace scope for this CLI request')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'The number of seconds to wait between refreshes')
            [CompletionResult]::new('--refresh-seconds', 'refresh-seconds', [CompletionResultType]::ParameterName, 'The number of seconds to wait between refreshes')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'The json file to read the PipelineRun from instead of running one')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'The json file to read the PipelineRun from instead of running one')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Choose the last pipelinerun')
            [CompletionResult]::new('--last', 'last', [CompletionResultType]::ParameterName, 'Choose the last pipelinerun')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
