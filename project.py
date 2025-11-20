import os

# 扫描的允许文件后缀
ALLOW_EXT = {".rs", ".toml", ".md", ".txt"}
SPECIAL_FILES = {"Cargo.toml", "Cargo.lock", "build.rs"}

# 忽略的目录
IGNORE_DIRS = {".git", "target", "node_modules", ".idea", ".vscode", "dist", "build"}


def should_read_file(file_name: str) -> bool:
    if file_name in SPECIAL_FILES:
        return True
    _, ext = os.path.splitext(file_name)
    return ext in ALLOW_EXT


def read_rust_project(root_path: str) -> str:
    result = []

    for root, dirs, files in os.walk(root_path):
        # 过滤目录
        dirs[:] = [d for d in dirs if d not in IGNORE_DIRS]

        for f in files:
            if not should_read_file(f):
                continue

            full_path = os.path.join(root, f)

            try:
                with open(full_path, "r", encoding="utf-8") as fp:
                    content = fp.read()
            except Exception as e:
                content = f"<<无法读取文件: {e}>>"

            rel_path = os.path.relpath(full_path, root_path)
            result.append(f"===== FILE: {rel_path} =====\n{content}\n")

    return "\n".join(result)


if __name__ == "__main__":
    project_dir = input("请输入 Rust 项目根目录路径: ").strip()
    output = read_rust_project(project_dir)

    # 输出到文件
    out_file = "project_for_ai.txt"
    with open(out_file, "w", encoding="utf-8") as fp:
        fp.write(output)

    print(f"已写入：{out_file}")
