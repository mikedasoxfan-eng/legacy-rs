"""Run new modes using the merged oracle, or a read-only sibling build.

All compilation and fixture output stays in this worktree. This does not create
or replace the harness worker's Oracle, hooks, or original-game classes.
"""
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]

def main():
    candidates = [ROOT / 'tools/oracle10', ROOT.parent / 'wt181-r10harness/tools/oracle10']
    if os.environ.get('ORACLE10_HOME'):
        candidates.insert(0, Path(os.environ['ORACLE10_HOME']))
    oracle = next((p for p in candidates if (p / 'build/net/minecraft/src/Mode_entities10.class').exists()), None)
    if oracle is None:
        raise RuntimeError('Build tools/oracle10 first, or set ORACLE10_HOME to an existing 1.0 harness build')
    mcp = oracle.parent / 'retromcp-r10'
    if not mcp.exists():
        mcp = ROOT.parent / 'beta-rs/tools/retromcp-r10'
    jar = mcp / 'minecraft/jars/deobfuscated.jar'
    build = ROOT / 'target/r10entities2-oracle'
    build.mkdir(parents=True, exist_ok=True)
    libs = [p for p in (mcp / 'libraries').rglob('*.jar') if 'sources' not in p.name and 'natives' not in p.name]
    unsealed = oracle / 'build/lwjgl-unsealed.jar'
    libs = [unsealed if p.name.startswith('lwjgl-2') and unsealed.exists() else p for p in libs]
    cp = os.pathsep.join(map(str, [build, oracle / 'build', jar] + libs))
    source = ROOT / 'tools/oracle10/src/net/minecraft/src/Mode_r10entities2.java'
    target = build / 'net/minecraft/src/Mode_r10entities2.class'
    if not target.exists() or source.stat().st_mtime > target.stat().st_mtime:
        result = subprocess.run(['javac', '-nowarn', '-cp', cp, '-d', str(build), str(source)])
        if result.returncode:
            return result.returncode
    if sys.argv[1] == '--compile':
        return 0
    runtimes = list((Path(os.environ.get('LOCALAPPDATA', '')) / 'Packages').glob('Microsoft.4297127D64EC6_*/LocalCache/Local/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe'))
    java = os.environ.get('ORACLE_JAVA', str(runtimes[0]) if runtimes else 'java')
    version = subprocess.run([java, '-version'], capture_output=True, text=True)
    if '1.8.0_51' not in version.stderr:
        raise RuntimeError('Original-bytecode comparisons require the launcher Java 8u51')
    output = Path(sys.argv[2]).resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    # Oracle.newWorld creates a throwaway save; keep it under target as well.
    return subprocess.run([java, '-cp', cp, 'net.minecraft.src.Mode_r10entities2', sys.argv[1], str(output)], cwd=build).returncode

if __name__ == '__main__':
    raise SystemExit(main())
