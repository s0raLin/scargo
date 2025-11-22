#!/bin/bash
# 下载scala-cli可执行文件到bin目录

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
echo "Downloading scala-cli to $BIN_DIR/scala-cli..."

# 创建临时文件
TEMP_FILE=$(mktemp)

# 下载scala-cli到临时文件
curl -fL "https://github.com/VirtusLab/scala-cli/releases/latest/download/scala-cli-${PLATFORM}.gz" | \
    gzip -d > "$TEMP_FILE"

# 移动到最终位置
mv "$TEMP_FILE" "$BIN_DIR/scala-cli"

# 设置执行权限
chmod +x "$BIN_DIR/scala-cli"

echo "Successfully downloaded scala-cli to $BIN_DIR/scala-cli"
echo "Version:"
"$BIN_DIR/scala-cli" --version