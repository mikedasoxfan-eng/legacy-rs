"""Reproduce the mechanical 1.0 harness copy without modifying Beta fixtures.

Run once before hand-fixing the 1.0 API deltas. The inferred rename file contains
local variables and changed types as well as renames; only unambiguous numbered
MCP identifiers and explicit class mappings are safe across arbitrary receivers.
"""
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def apply_renames(text, names):
    # Leave comments and strings intact: reflection names are handled separately
    # below, while fixture text and diagnostic labels must remain reproducible.
    pattern = re.compile(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|//[^\n]*|/\*[\s\S]*?\*/|\b[A-Za-z_$][\w$]*\b')
    return pattern.sub(lambda m: names.get(m[0], m[0]), text)


def main():
    renames = json.loads((ROOT / 'tools/r10/renames.json').read_text())
    names = {k: v for k, v in renames['global'].items()
             if re.fullmatch(r'(?:func|field)_\d+_\w+', k)}
    names.update(renames['classes'])
    names['ChunkProviderSky'] = 'ChunkProviderEnd'
    source = ROOT / 'tools/oracle181'
    dest = ROOT / 'tools/oracle10'
    for path in source.rglob('*'):
        if not path.is_file() or any(p in {'build', 'build_try', 'out', 'generated', '__pycache__'} for p in path.relative_to(source).parts):
            continue
        target = dest / str(path.relative_to(source)).replace('181', '10')
        target.parent.mkdir(parents=True, exist_ok=True)
        if path.suffix not in {'.java', '.sh', '.py', '.txt'}:
            target.write_bytes(path.read_bytes())
            continue
        text = path.read_text(encoding='utf-8')
        text = text.replace('retromcp-b181', 'retromcp-r10').replace('b1.8.1', '1.0')
        # Version suffixes change; MCP numbers, RNG seeds and binary magic do not.
        text = re.sub(r'\b([A-Za-z_][A-Za-z_0-9]*?)181\b', r'\g<1>10', text)
        text = text.replace('oracle181', 'oracle10')
        for suffix in ('.trace181','.draws181'):
            text=text.replace(suffix.replace('181','10'),suffix)
        # Legacy ModelShot has a stale oracle (b1.7.3) path in the Beta copy.
        text = text.replace('$root/tools/oracle"', '$root/tools/oracle10"')
        if path.suffix == '.java':
            text = apply_renames(text, names)
            for old, new in names.items():
                text = text.replace('"' + old + '"', '"' + new + '"')
        target.write_text(text, encoding='utf-8', newline='\n')
    # Remove obsolete filenames from an initial filesystem copy.
    for path in dest.rglob('*181*'):
        if path.is_file():
            path.unlink()
    print('Copied and renamed oracle sources; use port_verify.py for the suites')


if __name__ == '__main__':
    main()
