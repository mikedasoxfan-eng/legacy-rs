"""Capture or commit effects stages without rewriting the working tree.

Capture: python tools/stage_effects.py capture NUMBER TITLE PATH...
Commit:  python tools/stage_effects.py commit
Check:   python tools/stage_effects.py check
"""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
STAGES = ROOT / "target/effects-stages"
AUTHOR = "logiu"
EMAIL = "224190594+mikedasoxfan-eng@users.noreply.github.com"
TRAILERS = ("Co-Authored-By: Codex (gpt-6.1-sol) <noreply@openai.com>\n"
            "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>")


def git(*args, env=None):
    return subprocess.check_output(["git", *args], cwd=ROOT, env=env)


def stages():
    return [(p, json.loads(p.read_text())) for p in sorted(STAGES.glob("[0-9]*/manifest.json"))]


def validate(saved_stages):
    if not saved_stages:
        sys.exit("No captured effects stages.")
    latest = {name: manifest.parent / "files" / name
              for manifest, data in saved_stages for name in data["paths"]}
    changed = [name for name, saved in latest.items()
               if not (ROOT / name).is_file() or not saved.is_file()
               or (ROOT / name).read_bytes() != saved.read_bytes()]
    if changed:
        sys.exit("Uncaptured changes; capture another stage before committing:\n" + "\n".join(changed))
    staged = set(git("diff", "--cached", "--name-only", "-z").decode().strip("\0").split("\0"))
    overlap = staged.intersection(latest)
    if overlap:
        sys.exit("Captured paths also have staged edits; preserve those before committing:\n" + "\n".join(sorted(overlap)))
    for _, data in saved_stages:
        if data.get("commit"):
            git("merge-base", "--is-ancestor", data["commit"], "HEAD")
    return latest


def commit():
    saved_stages = stages()
    validate(saved_stages)
    # Start from HEAD, so unrelated staged work never enters these commits.
    fd, index = tempfile.mkstemp(prefix="effects-index-", dir=ROOT / "target")
    os.close(fd)
    os.unlink(index)
    env = os.environ.copy()
    env.update(GIT_INDEX_FILE=index, GIT_AUTHOR_NAME=AUTHOR, GIT_AUTHOR_EMAIL=EMAIL,
               GIT_COMMITTER_NAME=AUTHOR, GIT_COMMITTER_EMAIL=EMAIL)
    modes = {}
    for entry in git("ls-files", "--stage", "-z").decode().strip("\0").split("\0"):
        if entry:
            info, name = entry.split("\t", 1)
            modes[name] = info.split()[0]
    committed_paths = set()
    try:
        for manifest, data in saved_stages:
            if data.get("commit"):
                committed_paths.update(data["paths"])
                continue
            git("read-tree", "HEAD", env=env)
            for name in data["paths"]:
                saved = manifest.parent / "files" / name
                # --path applies the repository's filters to the captured bytes.
                blob = git("hash-object", "-w", "--path", name, str(saved)).decode().strip()
                git("update-index", "--add", "--cacheinfo", f"{modes.get(name, '100644')},{blob},{name}", env=env)
            git("commit", "-m", data["title"], "-m", TRAILERS, env=env)
            data["commit"] = git("rev-parse", "HEAD").decode().strip()
            committed_paths.update(data["paths"])
            manifest.write_text(json.dumps(data, indent=2) + "\n")
            print(data["commit"], data["title"])
    finally:
        # Reconcile only our committed paths in the real index. Working files and
        # unrelated staged entries remain untouched, including on partial failure.
        if committed_paths:
            git("reset", "-q", "HEAD", "--", *sorted(committed_paths))
        Path(index).unlink(missing_ok=True)


def main():
    command, *args = sys.argv[1:]
    if command == "capture":
        number, title, *paths = args
        if not number.isdigit() or not paths:
            sys.exit("Capture requires a numeric stage and explicit paths.")
        dest = STAGES / number
        if (dest / "manifest.json").exists() and json.loads((dest / "manifest.json").read_text()).get("commit"):
            sys.exit("Cannot replace a committed stage.")
        for name in paths:
            source = (ROOT / name).resolve()
            if not source.is_relative_to(ROOT) or not source.is_file():
                sys.exit("Capture path must name a workspace file: " + name)
            target = dest / "files" / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, target)
        (dest / "manifest.json").write_text(json.dumps({"title": title, "paths": paths}, indent=2) + "\n")
        script = ROOT / "target/commit-effects.sh"
        script.write_text('#!/usr/bin/env bash\nset -euo pipefail\ncd "$(dirname "$0")/.."\npython tools/stage_effects.py commit\n')
    elif command == "check":
        latest = validate(stages())
        print(f"Validated {len(stages())} stages and {len(latest)} captured paths; no Git writes.")
    elif command == "commit":
        commit()
    else:
        sys.exit("Expected capture, check or commit.")


if __name__ == "__main__":
    main()
