# Copyright 2019-2022 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

$bitness = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$__TAG_NAME__ = "create-tauri-app-v2.6.2"
# $url="https://github.com/tauri-apps/create-tauri-app/releases/download/$__TAG_NAME__/create-tauri-app-$bitness-pc-windows-msvc.exe"
$url="https://create.tauri.app/download/bin?tag=$__TAG_NAME__&arch=$bitness-pc-windows-msvc&ext=.exe"
$outFile = "$Env:TEMP\create-tauri-app.exe"

Write-Output "$($PSStyle.Bold)$($PSStyle.Foreground.Green)info:$($PSStyle.Reset) downloading create-tauri-app"

$oldProgressPreference = $ProgressPreference
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri $url -OutFile $outFile
$ProgressPreference = $oldProgressPreference

Start-Process -FilePath $outFile -Wait -NoNewWindow
