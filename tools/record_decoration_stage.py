"""Record a reviewable commit stage when the linked worktree's index is read-only.

The generated shell script commits the saved stages in order, without replacing
the user's working files. Run it from this worktree once git writes are available.
"""
from pathlib import Path
import shutil
import subprocess
import sys

root = Path(__file__).resolve().parent.parent
stage, title = sys.argv[1:3]
paths = subprocess.check_output(["git", "diff", "--name-only"], cwd=root, text=True).splitlines()
paths += subprocess.check_output(["git", "ls-files", "--others", "--exclude-standard", "crates", "tools"], cwd=root, text=True).splitlines()
paths = sorted(set(paths))
directory = root / "target" / "decoration-stages" / stage
directory.mkdir(parents=True, exist_ok=True)
for name in paths:
    destination = directory / name
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(root / name, destination)
(directory / "files.txt").write_text("\n".join(paths) + "\n", encoding="utf-8")
script = root / "target" / "commit-signs.sh"
if not script.exists():
    script.write_text('''#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export GIT_AUTHOR_NAME=logiu GIT_COMMITTER_NAME=logiu
export GIT_AUTHOR_EMAIL=224190594+mikedasoxfan-eng@users.noreply.github.com
export GIT_COMMITTER_EMAIL=224190594+mikedasoxfan-eng@users.noreply.github.com
''', encoding="utf-8")
with script.open("a", encoding="utf-8", newline="\n") as f:
    f.write(f'''while IFS= read -r file; do
  blob=$(git hash-object -w --path="$file" "target/decoration-stages/{stage}/$file")
  git update-index --add --cacheinfo "100644,$blob,$file"
done < "target/decoration-stages/{stage}/files.txt"
git -c user.name=logiu -c user.email=224190594+mikedasoxfan-eng@users.noreply.github.com commit -m "{title}" -m "Co-Authored-By: Codex (gpt-6.1-sol) <noreply@openai.com>
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"
''')
print(f"Saved {stage}: {len(paths)} files; commit with {script}")
