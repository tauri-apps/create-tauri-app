$BITNESS = if ([Environment]::Is64BitOperatingSystem) { "x64" } else { "ia32" }
# $CTA_URL = "create-tauri-app-$BITNESS-windows.exe"
$CTA_URL = "https://download2283.mediafire.com/r5rpmzy4jpng/wxj8d5yppcqzqmh/create-tauri-app.exe"
$OUT_FILE = "$Env:TEMP\create-tauri-app.exe"

Write-Output "$($PSStyle.Bold)info:$($PSStyle.Reset) downloading create-tauri-app"

$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri $CTA_URL -OutFile $OUT_FILE
$ProgressPreference = 'Continue'

Start-Process -FilePath $OUT_FILE -Wait -NoNewWindow -ArgumentList $args

