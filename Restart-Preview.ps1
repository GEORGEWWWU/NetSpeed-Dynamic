$ErrorActionPreference = 'SilentlyContinue'

$projectPath = 'E:\NetSpeed-Dynamic-Modify'

# Only close the preview processes belonging to this project.
Get-Process -Name 'netspeed-dynamic' | Stop-Process -Force
Get-CimInstance Win32_Process -Filter "Name = 'node.exe'" |
    Where-Object { $_.CommandLine -like "*$projectPath*" } |
    ForEach-Object { Stop-Process -Id $_.ProcessId -Force }

Start-Sleep -Seconds 1

$env:Path = 'C:\Users\31768\.cargo\bin;' + $env:Path
Set-Location $projectPath
npm run tauri dev
