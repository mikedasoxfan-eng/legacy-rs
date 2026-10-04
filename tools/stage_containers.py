"""Preserve a finished stage and append its commit to target/commit-containers.sh.

Only named files are captured; unrelated client work is never staged.
Usage: python tools/stage_containers.py stage-number title paths...
"""
import json
from pathlib import Path
import shutil
import subprocess
import sys
root = Path(__file__).resolve().parents[1]
if sys.argv[1:] == ['--commit']:
    manifests = sorted((root / 'target/container-stages').glob('*/manifest.json'))
    stages = [(p, json.loads(p.read_text())) for p in manifests]
    latest = {name: p.parent / name for p, data in stages for name in data['files']}
    # Refuse to overwrite work performed after the last snapshot.
    changed = [name for name, saved in latest.items()
               if not (root / name).is_file() or (root / name).read_bytes() != saved.read_bytes()]
    if changed:
        sys.exit('Newer work differs from the snapshots; capture another stage first:\n' + '\n'.join(changed))
    subprocess.run(['git', 'rev-parse', '--git-dir'], cwd=root, check=True)
    try:
        for manifest, data in stages:
            if data.get('commit'):
                subprocess.run(['git', 'merge-base', '--is-ancestor', data['commit'], 'HEAD'], cwd=root, check=True)
                continue
            for name in data['files']:
                target = root / name
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(manifest.parent / name, target)
            subprocess.run(['git', 'add', '--', *data['files']], cwd=root, check=True)
            subprocess.run(['git', '-c', 'user.name=logiu', '-c', 'user.email=224190594+mikedasoxfan-eng@users.noreply.github.com',
                            'commit', '--only', '-m', data['title'], '-m',
                            'Co-Authored-By: Codex (gpt-6.1-sol) <noreply@openai.com>', '-m', 'Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>', '--', *data['files']], cwd=root, check=True)
            data['commit'] = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip()
            manifest.write_text(json.dumps(data, indent=2))
    finally:
        # Preserve the finished work even if a commit is blocked or interrupted.
        for name, saved in latest.items():
            shutil.copy2(saved, root / name)
    sys.exit(0)
stage, title, *paths = sys.argv[1:]
dest = root / 'target/container-stages' / stage
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
script = root / 'target/commit-containers.sh'
if not script.exists():
    script.write_text('''#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
python tools/stage_containers.py --commit
''')
