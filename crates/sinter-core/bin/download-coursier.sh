#!/bin/bash
# 下载coursier可执行文件到bin目录

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="$SCRIPT_DIR"

# 检测平台
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case "$os" in
        linux)
            if [ "$arch" = "x86_64" ]; then
                echo "x86_64-pc-linux"
            elif [ "$arch" = "aarch64" ] || [ "$arch" = "arm64" ]; then
                echo "aarch64-pc-linux"
            else
                echo "unsupported"
            fi
            ;;
        darwin)
            if [ "$arch" = "x86_64" ]; then
                echo "x86_64-apple-darwin"
            elif [ "$arch" = "aarch64" ] || [ "$arch" = "arm64" ]; then
                echo "aarch64-apple-darwin"
            else
                echo "unsupported"
            fi
            ;;
        *)
            echo "unsupported"
            ;;
    esac
}

PLATFORM=$(detect_platform)

if [ "$PLATFORM" = "unsupported" ]; then
    echo "Error: Unsupported platform: $(uname -s) $(uname -m)"
    exit 1
fi

echo "Detected platform: $PLATFORM"
echo "Downloading coursier to $BIN_DIR/coursier..."

# 下载coursier
curl -fL "https://github.com/coursier/coursier/releases/latest/download/cs-${PLATFORM}.gz" | \
    gzip -d > "$BIN_DIR/coursier"

# 设置执行权限
chmod +x "$BIN_DIR/coursier"

echo "Successfully downloaded coursier to $BIN_DIR/coursier"
echo "Version:"
"$BIN_DIR/coursier" --version

