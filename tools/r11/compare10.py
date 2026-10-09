"""Run an inherited 1.0 Rust checker with isolated diagnostic output paths."""
import hashlib
import json
import os
import subprocess
import sys
import shutil
from pathlib import Path

from diagnose_dump10 import diagnose

ROOT = Path(__file__).resolve().parents[2]


def text_difference(java, rust):
    left=java.read_text(errors='replace').splitlines();right=rust.read_text(errors='replace').splitlines()
    for line,(j,r) in enumerate(zip(left,right)):
        if j==r:continue
        for token,(a,b) in enumerate(zip(j.split(),r.split())):
            if a!=b:return f'text line {line}, {j.split()[0]} field[{token}]: Java {a!r}, Rust {b!r}'
        return f'text line {line}.fieldCount: Java {len(j.split())}, Rust {len(r.split())}'
    return f'text line count: Java {len(left)}, Rust {len(right)}'


def main():
    binary = Path(sys.argv[1]).resolve()
    if not binary.is_file() and binary.with_suffix('.exe').is_file():
        binary = binary.with_suffix('.exe')
    args = sys.argv[2:]
    args = [str((ROOT / a).resolve()) if a.replace('\\','/').startswith(('target/', 'tools/', '1.1/')) else a for a in args]
    key = hashlib.sha256(json.dumps([str(binary),args]).encode()).hexdigest()[:20]
    work = ROOT / 'target/r11/compare' / key
    (work / 'target').mkdir(parents=True, exist_ok=True)
    candidates = [Path(a) for a in args if a.endswith(('.bin','.txt')) and Path(a).is_file()]
    oracle_arguments = None
    snapshot = None
    if candidates:
        original = candidates[-1]
        snapshot = work / 'target/oracle-java.bin'
        shutil.copyfile(original, snapshot)
        sidecar=Path(str(original)+'.oracle11.json')
        if sidecar.exists(): oracle_arguments=json.loads(sidecar.read_text())['arguments']
        stale = list((work/'target').glob('*rust.bin')) + list((work/'target').glob('*rust.txt')) + [original.with_suffix('.rust'+original.suffix)]
        stale += [p for p in candidates[:-1] if 'rust' in p.name]
        for path in stale:
            resolved=path.resolve()
            if path.exists() and any(resolved.is_relative_to(base) for base in [ROOT/'target/r11',ROOT/'tools/oracle11/out']):
                path.unlink()
    env = dict(os.environ)
    if binary.stem == 'terrain_compare11': env['TERRAIN_COMPARE_DUMP'] = str(work / 'target/checker-rust.bin')
    result = subprocess.run([str(binary), *args], cwd=work, env=env, capture_output=True, text=True, errors='replace')
    # This Rust fixture deliberately writes files consumed by the next oracle
    # invocation. Publish them to the release-only location used by its script.
    if binary.stem == 'compare11' and args and args[0] == 'mapfiles':
        generated=work/'target/map-rust-save'
        if generated.exists(): shutil.copytree(generated,ROOT/'target/r11/map-rust-save',dirs_exist_ok=True)
    print(result.stdout, end='');print(result.stderr, end='', file=sys.stderr)
    metadata = {'binary': binary.name, 'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),
                'arguments':args, 'returncode':result.returncode, 'output':result.stdout+result.stderr}
    if snapshot: metadata['java_snapshot']=str(snapshot)
    if oracle_arguments: metadata['oracle_arguments']=oracle_arguments
    candidates = [Path(a) for a in args if a.endswith(('.bin','.txt')) and Path(a).is_file()]
    if result.returncode and candidates:
        java = candidates[-1]
        rust = java.with_suffix('.rust'+java.suffix)
        supplied = [p for p in candidates[:-1] if 'rust' in p.name]
        if not rust.exists() and supplied: rust = supplied[-1]
        if not rust.exists():
            dumped = list((work / 'target').glob('*rust'+java.suffix))
            if len(dumped) == 1: rust = dumped[0]
        if rust.exists():
            try:
                detail = diagnose(snapshot or java,rust,oracle_arguments) if java.suffix=='.bin' else text_difference(snapshot or java,rust)
                metadata['first_difference'] = detail
                print('FIRST DIFFERENCE: ' + detail, file=sys.stderr)
            except Exception as error:
                metadata['diagnostic_error'] = type(error).__name__ + ': ' + str(error)
    (work / 'comparison.json').write_text(json.dumps(metadata,indent=2))
    return result.returncode


if __name__ == '__main__': raise SystemExit(main())
