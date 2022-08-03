# Copyright 2019-2022 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

$bitness = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$__TAG_NAME__ = "create-tauri-app-v2.0.0-alpha.7"
$url="https://github.com/tauri-apps/create-tauri-app/releases/download/$__TAG_NAME__/create-tauri-app-$bitness-pc-windows-msvc.exe"
$outFile = "$Env:TEMP\create-tauri-app.exe"

Write-Output "$($PSStyle.Bold)info:$($PSStyle.Reset) downloading create-tauri-app"

$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri $url -OutFile $outFile
$ProgressPreference = 'Continue'

Start-Process -FilePath $outFile -Wait -NoNewWindow -ArgumentList $args
