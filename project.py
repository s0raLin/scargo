import os

# 允许包含的文件扩展名
ALLOWED_EXT = {
    ".rs", ".toml", ".lock", ".md", ".json",
    ".yaml", ".yml", ".txt"
}

# 需要排除的目录
EXCLUDED_DIRS = {
    "target", ".git", "node_modules", "dist", "build",
    ".idea", ".vscode", "__pycache__", ".pytest_cache"
}

OUTPUT_FILE = "project_tokens.txt"


def is_allowed_file(path: str) -> bool:
    _, ext = os.path.splitext(path)
    return ext in ALLOWED_EXT


def should_skip_dir(dir_name: str) -> bool:
    return dir_name in EXCLUDED_DIRS


def collect_files(root: str):
    collected = []

    for dirpath, dirnames, filenames in os.walk(root):
        # 移除不必要目录
        dirnames[:] = [d for d in dirnames if not should_skip_dir(d)]

        for filename in filenames:
            full_path = os.path.join(dirpath, filename)
            if is_allowed_file(full_path):
                collected.append(full_path)

    return collected


def read_file(path: str) -> str:
    try:
        with open(path, "r", encoding="utf-8") as f:
            return f.read()
    except UnicodeDecodeError:
        return "/// [Binary or Non-UTF8 File Skipped]"


def pack_project(root: str):
    files = collect_files(root)
    files.sort()

    with open(OUTPUT_FILE, "w", encoding="utf-8") as out:
        for file_path in files:
            rel_path = os.path.relpath(file_path, root)

            out.write(f"/// FILE: {rel_path}\n\n")
            out.write(read_file(file_path))
            out.write("\n\n\n")

            print(f"已处理：{rel_path}")

    print(f"\n🎉 完成！输出文件：{OUTPUT_FILE}")


if __name__ == "__main__":
    import sys

    if len(sys.argv) < 2:
        print("用法：python project_packer.py <Rust 项目路径>")
        exit(1)

    project_root = sys.argv[1]
    pack_project(project_root)
