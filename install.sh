#!/bin/sh
#
# filedress-installer: A shell script to install the 'filedress' CLI tool.
#
# Usage:
#   curl --proto '=https' --tlsv1.2 -sSf https://.../install.sh | sh
#
# This script will:
# 1. Determine the user's OS and CPU architecture.
# 2. Fetch the latest release version from the GitHub API.
# 3. Download the correct binary for their system from GitHub Releases.
# 4. Unpack the binary and install it to ~/.local/bin.

set -e # Exit immediately if a command exits with a non-zero status.

# --- Configuration ---
GITHUB_REPO="Netajam/filedress"
INSTALL_DIR="${HOME}/.local/bin"
CMD_NAME="filedress"

# --- Helper Functions ---
echo_green() {
  printf "\033[0;32m%s\033[0m\n" "$1"
}

echo_error() {
  printf "\033[0;31mError: %s\033[0m\n" "$1" >&2
  exit 1
}

# --- Main Logic ---
# 1. Determine OS and Architecture
os=$(uname -s | tr '[:upper:]' '[:lower:]')
arch=$(uname -m)

case "$os" in
  linux)
    case "$arch" in
      x86_64) asset_arch="x86_64-unknown-linux-gnu" ;;
      *) echo_error "Unsupported architecture: $arch" ;;
    esac
    asset_os="linux"
    ;;
  darwin)
    case "$arch" in
      x86_64 | arm64) asset_arch="x86_64-apple-darwin" ;; # Note: Using Rosetta 2 for arm64 for now
      *) echo_error "Unsupported architecture: $arch" ;;
    esac
    asset_os="macos"
    ;;
  *)
    echo_error "Unsupported OS: $os"
    ;;
esac

# 2. Get the latest release version
echo "Fetching latest version of ${CMD_NAME}..."
latest_tag=$(curl -s "https://api.github.com/repos/${GITHUB_REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$latest_tag" ]; then
  echo_error "Could not fetch the latest release tag. Check repository name."
fi
echo_green "Latest version is ${latest_tag}"

# 3. Construct the download URL
asset_name="${CMD_NAME}-${asset_os}-x86_64.tar.gz" # Adjust if you create more specific arch builds
download_url="https://github.com/${GITHUB_REPO}/releases/download/${latest_tag}/${asset_name}"

echo "Downloading from ${download_url}..."

# 4. Download and install
temp_file=$(mktemp)
curl -L --progress-bar -o "$temp_file" "$download_url"

echo "Installing to ${INSTALL_DIR}..."
mkdir -p "$INSTALL_DIR"
tar -xzf "$temp_file" -C "$INSTALL_DIR" "${CMD_NAME}"
rm "$temp_file"

# 5. Verify and finish
if ! command -v "${CMD_NAME}" >/dev/null; then
    echo "Warning: '${INSTALL_DIR}' is not in your PATH."
    echo "Please add the following line to your ~/.bashrc or ~/.zshrc:"
    echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
fi

echo_green "${CMD_NAME} installed successfully!"
"${INSTALL_DIR}/${CMD_NAME}" --version