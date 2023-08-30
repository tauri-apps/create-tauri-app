# Copyright 2019-2022 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

$bitness = if ([System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture -eq "X64") {
    "x86_64"
} elseif ([System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture -eq "Arm64") {
    "aarch64"
} else {
    "i686"
}
$__TAG_NAME__ = "create-tauri-app-v3.7.3"
# $url="https://github.com/tauri-apps/create-tauri-app/releases/download/$__TAG_NAME__/create-tauri-app-$bitness-pc-windows-msvc.exe"
$url="https://create.tauri.app/download/bin?tag=$__TAG_NAME__&arch=$bitness-pc-windows-msvc&ext=.exe"
$outFile = "$Env:TEMP\create-tauri-app.exe"

Write-Output "$($PSStyle.Bold)$($PSStyle.Foreground.Green)info:$($PSStyle.Reset) downloading create-tauri-app"

$oldProgressPreference = $ProgressPreference
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri $url -OutFile $outFile
$ProgressPreference = $oldProgressPreference

if ($Env:CTA_ARGS) {
    Start-Process -FilePath $outFile -Wait -NoNewWindow -ArgumentList "$Env:CTA_ARGS"
} else {
    Start-Process -FilePath $outFile -Wait -NoNewWindow
}
