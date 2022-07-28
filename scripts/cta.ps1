$bitness = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$tagName=""
$url="https://github.com/tauri-apps/binary-releases/releases/download/$tagName/create-tauri-app-$bitness-pc-windows-msvc.exe"
$outFile = "$Env:TEMP\create-tauri-app.exe"

Write-Output "$($PSStyle.Bold)info:$($PSStyle.Reset) downloading create-tauri-app"

$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri $url -OutFile $outFile
$ProgressPreference = 'Continue'

Start-Process -FilePath $outFile -Wait -NoNewWindow -ArgumentList $args

