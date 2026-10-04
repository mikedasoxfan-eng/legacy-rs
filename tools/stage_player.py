"""Preserve a finished stage and append its commit to target/commit-player.sh.

Only named files are captured; unrelated client work is never staged.
Usage: python tools/stage_player.py stage-number title paths...
"""
import json
from pathlib import Path
import shutil
import subprocess
import sys
root = Path(__file__).resolve().parents[1]
if sys.argv[1:] == ['--commit']:
    for manifest in sorted((root / 'target/player-stages').glob('*/manifest.json')):
        data = json.loads(manifest.read_text())
        for name in data['files']:
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(manifest.parent / name, target)
        subprocess.run(['git', 'add', '--', *data['files']], cwd=root, check=True)
        subprocess.run(['git', '-c', 'user.name=logiu', '-c', 'user.email=224190594+mikedasoxfan-eng@users.noreply.github.com', 'commit', '-m', data['title'], '-m', 'Co-Authored-By: Codex (gpt-6.1-sol) <noreply@openai.com>'], cwd=root, check=True)
    sys.exit(0)
stage, title, *paths = sys.argv[1:]
dest = root / 'target/player-stages' / stage
files = []
for name in paths:
    p = root / name
    for file in sorted(p.rglob('*')) if p.is_dir() else [p]:
        if file.is_file():
            relative = file.relative_to(root).as_posix()
            target = dest / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(file, target)
            files.append(relative)
dest.mkdir(parents=True, exist_ok=True)
(dest / 'manifest.json').write_text(json.dumps({'title': title, 'files': files}, indent=2))
script = root / 'target/commit-player.sh'
if not script.exists():
    script.write_text('''#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
python tools/stage_player.py --commit
''')
