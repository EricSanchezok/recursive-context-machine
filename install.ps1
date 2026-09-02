# RCM Windows Installer
# Usage: irm https://raw.githubusercontent.com/EricSanchezok/recursive-context-machine/main/install.ps1 | iex

param(
  [string]$Version = ""
)

$ErrorActionPreference = "Stop"

$Repo = "EricSanchezok/recursive-context-machine"
$BaseUrl = "https://github.com/$Repo/releases"
$BinName = "accelerate"

if (-not $Version) {
  $ReleaseUrl = "https://api.github.com/repos/$Repo/releases/latest"
  $Release = Invoke-RestMethod -Uri $ReleaseUrl -ErrorAction Stop
  $Version = $Release.tag_name
}

Write-Host "Installing $BinName $Version for Windows x86_64 ..."

$ArchiveName = "${BinName}-x86_64-windows.zip"
$DownloadUrl = "${BaseUrl}/download/${Version}/${ArchiveName}"
$TempDir = Join-Path $env:TEMP "rcm-install-$([System.Guid]::NewGuid().ToString('N').Substring(0, 8))"

New-Item -ItemType Directory -Path $TempDir -Force | Out-Null
$ArchivePath = Join-Path $TempDir $ArchiveName

Write-Host "Downloading from $DownloadUrl ..."
Invoke-WebRequest -Uri $DownloadUrl -OutFile $ArchivePath

Expand-Archive -Path $ArchivePath -DestinationPath $TempDir -Force

$InstallDir = if ($env:RCM_INSTALL_DIR) { $env:RCM_INSTALL_DIR } else { "$env:USERPROFILE\.rcm\bin" }
New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
Copy-Item -Path "$TempDir\${BinName}.exe" -Destination "$InstallDir\${BinName}.exe" -Force

$env:Path = "$InstallDir;" + $env:Path
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
  [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
}

Remove-Item -Recurse -Force $TempDir

Write-Host ""
Write-Host "Installed ${BinName}.exe -> $InstallDir"
Write-Host ""
Write-Host "Run 'accelerate --help' to get started."
