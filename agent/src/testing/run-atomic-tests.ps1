

Import-Module "C:\AtomicRedTeam\invoke-atomicredteam\Invoke-AtomicRedTeam.psd1" -Force

# Test Parameters
$T1136001Args = @{ "password" = "Chiapet1" }
$T1505003Args = @{ "web_shell_path" = "C:\inetpub\wwwroot"; "web_shells" = "C:\AtomicRedTeam\atomics\T1505.003\src" }
$T1546007Args = @{ "helper_file" = "C:\AtomicRedTeam\atomics\T1134.004\bin\calc.dll" }
$T1546010Args = @{ "registry_file" = "C:\AtomicRedTeam\atomics\T1546.010\src\T1546.010.reg" }

Invoke-AtomicTest T1037.001 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1053.005 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1055 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1136.001 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv' -InputArgs $T1136001Args
Invoke-AtomicTest T1505.003 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv' -InputArgs $T1505003Args
Invoke-AtomicTest T1543.003 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1546.007 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv' -InputArgs $T1546007Args
Invoke-AtomicTest T1546.008 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1546.010 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv' -InputArgs $T1546010Args
Invoke-AtomicTest T1546.011 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1546.012 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1546.015 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1547.001 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1547.004 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1547.005 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1562.004 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'
Invoke-AtomicTest T1569.002 -ExecutionLogPath 'C:\Users\user\Desktop\ICSHunter\AtomicTestsResults.csv'