# Coursier 可执行文件

此目录用于存放打包的 `coursier` 可执行文件。

## 如何添加 coursier 可执行文件

### Linux (x86_64)
```bash
curl -fL https://github.com/coursier/coursier/releases/latest/download/cs-x86_64-pc-linux.gz | gzip -d > coursier
chmod +x coursier
```

### macOS (x86_64)
```bash
curl -fL https://github.com/coursier/coursier/releases/latest/download/cs-x86_64-apple-darwin.gz | gzip -d > coursier
chmod +x coursier
```

### macOS (ARM64)
```bash
curl -fL https://github.com/coursier/coursier/releases/latest/download/cs-aarch64-apple-darwin.gz | gzip -d > coursier
chmod +x coursier
```

### Windows
下载 `cs-x86_64-pc-win32.zip`，解压后将 `cs.exe` 重命名为 `coursier.exe` 并放入此目录。

## 快速下载

使用提供的脚本自动下载：

```bash
cd crates/sinter-core/bin
./download-coursier.sh
```

## 使用说明

如果此目录中存在 `coursier`（或 `coursier.exe`），sinter 将优先使用打包的版本，而不是系统安装的版本。

查找顺序：
1. **打包的版本**：`<可执行文件目录>/bin/coursier` 或开发时的 `CARGO_MANIFEST_DIR/bin/coursier`
2. **系统命令**：PATH 中的 `coursier` 命令
3. **回退方案**：如果两者都不存在，sinter 会回退到使用 `scala-cli` 进行依赖管理

## 构建时包含

在构建发布版本时，确保将 `bin/coursier` 复制到可执行文件目录的 `bin/` 子目录中，这样打包的版本才能被找到。

例如，如果可执行文件在 `target/release/sinter`，则 coursier 应该在 `target/release/bin/coursier`。

