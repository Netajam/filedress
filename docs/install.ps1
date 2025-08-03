# filedress-installer: A PowerShell script to install the 'filedress' CLI tool on Windows.
#
# Usage (in PowerShell):
# iwr https://Netajam.github.io/filedress/install.ps1 -useb | iex

# --- Configuration ---
$GithubRepo = "Netajam/filedress"
$CmdName = "filedress"
# Install to a user-specific directory to avoid needing admin rights.
$InstallDir = "$env:LOCALAPPDATA\Programs\$CmdName"

# --- Main Logic ---
Write-Host "Fetching latest version of $($CmdName)..."

# 1. Get the latest release version from the GitHub API
try {
    $releaseInfo = Invoke-RestMethod -Uri "https://api.github.com/repos/$GithubRepo/releases/latest"
    $latestTag = $releaseInfo.tag_name
    Write-Host -ForegroundColor Green "Latest version is $($latestTag)"
}
catch {
    Write-Error "Could not fetch the latest release tag. Check repository name and ensure a release exists."
    exit 1
}

# 2. Construct the download URL for the Windows asset
$assetName = "$($CmdName)-windows-x86_64.zip"
$downloadUrl = "https://github.com/$GithubRepo/releases/download/$latestTag/$assetName"

Write-Host "Downloading from $($downloadUrl)..."

# 3. Download and unpack the archive
$tempFile = "$env:TEMP\filedress.zip"
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $tempFile -UseBasicParsing
}
catch {
    Write-Error "Failed to download the asset. Please check the URL and your connection."
    exit 1
}

Write-Host "Installing to $($InstallDir)..."
# Ensure the installation directory exists
if (-not (Test-Path -Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

# Unpack the ZIP archive
Expand-Archive -Path $tempFile -DestinationPath $InstallDir -Force

# 4. Add the installation directory to the user's PATH
Write-Host "Adding $($InstallDir) to your PATH..."
try {
    # Get the current user's PATH environment variable
    $currentUserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")

    # Add the new path only if it's not already there
    if (-not ($currentUserPath -split ';' -contains $InstallDir)) {
        $newPath = "$currentUserPath;$InstallDir"
        [System.Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        Write-Host -ForegroundColor Green "PATH updated. Please restart your terminal for the change to take effect."
    } else {
        Write-Host "PATH already contains the installation directory."
    }
}
catch {
    Write-Warning "Could not automatically update your PATH. Please add '$($InstallDir)' to your PATH environment variable manually."
}

# 5. Clean up the temporary file
Remove-Item -Path $tempFile

Write-Host -ForegroundColor Green "`n$($CmdName) installed successfully!"
Write-Host "Please open a new PowerShell or Command Prompt window to use the 'filedress' command."