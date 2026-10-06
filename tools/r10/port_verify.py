"""Copy Beta suites with 1.0 paths, preserving the inherited Rust CLI dialect.

worldgen10 renamed its executable targets, but its fixture-mode arguments still
have Beta names until area workers port them. Oracle.java accepts both aliases.
"""
import re
import os
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    beta = tomllib.loads((ROOT / 'crates/worldgen181/Cargo.toml').read_text())
    release = tomllib.loads((ROOT / 'crates/worldgen10/Cargo.toml').read_text())
    by_source = {b['path']: b['name'] for b in release['bin']}
    binaries = {b['name']: by_source[b['path']] for b in beta['bin']}
    for source, name in by_source.items():
        binaries.setdefault(Path(source).stem, name)
    paths = list((ROOT / 'tools').glob('verify*181.sh'))
    for path in paths:
        text = path.read_text()
        text = text.replace('b1.8.1', '1.0').replace('worldgen181', 'worldgen10')
        text = text.replace('oracle181', 'oracle10').replace('shots181', 'shots10')
        text = text.replace('ingame181', 'ingame10')
        text = text.replace('VERIFY181', 'VERIFY10').replace('verify181', 'verify10')
        for suite in paths:
            text = text.replace(suite.name, suite.name.replace('181', '10'))
        text = re.sub(r'(?<=release/)[A-Za-z0-9_]+', lambda m: binaries.get(m[0], m[0]), text)
        text = text.replace('target/release/${mode}181', 'target/release/${mode}10').replace('target/release/$mode\"', 'target/release/${mode/181/10}\"')
        text = re.sub(r'(?<=--bin )[A-Za-z0-9_]+', lambda m: binaries.get(m[0], m[0]), text)
        text = text.replace('\"target/release/${mode/181/10}\"', 'r10_compare \"target/release/${mode/181/10}\"')
        variables = re.findall(r'(?m)^(\w+)="\$root/target/release/([^"\n]+)"', text)
        for variable, executable in variables:
            if executable.removesuffix('.exe') in by_source.values():
                text = text.replace('"$'+variable+'"', 'r10_compare "$'+variable+'"')
        names = '|'.join(map(re.escape, sorted(set(by_source.values()), key=len, reverse=True)))
        command = re.compile(r'(?m)(^|[\s(;])("(?:\$root/)?target/release/(?:'+names+r')(?:\.exe)?"|(?:\$root/)?target/release/(?:'+names+r')(?:\.exe)?)(?=[\s;]|$)')
        text = command.sub(r'\1r10_compare \2', text)
        text = re.sub(r'oracle10/[^\s"\';)]+', lambda m: m[0] if m[0].startswith('oracle10/out/') else m[0].replace('181', '10'), text)
        # Prefix even legacy unversioned outputs to isolate the two suites.
        text = re.sub(r'(target/)(?!release/|debug/|r10/)([^\s"\';)]+)', r'\1r10/\2', text)
        text = text.replace('mkdir -p "$root/target"', 'mkdir -p "$root/target/r10"')
        text = text.replace('mkdir -p target', 'mkdir -p target/r10')
        # Initial top-level suites use paths relative to root but do not chdir.
        text = text.replace('root="$(cd "$(dirname "$0")/.." && pwd)"', 'root="$(cd "$(dirname "$0")/.." && pwd)"\ncd "$root"\nmkdir -p "$root/target/r10" "$root/tools/oracle10/out"')
        text = re.sub(r'(?m)^(\s*)rm -f ', r'\1[ "${VERIFY10_KEEP:-}" = 1 ] || rm -f ', text)
        text = text.replace('mkdir -p "$root/target/r10" "$root/tools/oracle10/out"', 'mkdir -p "$root/target/r10" "$root/tools/oracle10/out"\nsource "$root/tools/r10/verify_common10.sh"', 1)
        text = re.sub(r'(\"(?:\$root/)?target/release/beta\"|target/release/beta|\"\$beta\") --mc 1\.0', r'r10_client \1 --mc 1.0', text)
        text = text.replace('tools/b181/gui_sections.py', 'tools/r10/gui_sections10.py')
        # GUI capture comparison reports each image and its first changed pixel.
        text = re.sub(r'^.*--guidiff "\$java_out" "\$rust_out".*$', 'python "$root/tools/oracle10/compare_gui_strict.py" "$root/tools/oracle10/gui/scenes.txt" "$java_out" "$rust_out" "$root/shots10/gui_diff" || fail=1', text, flags=re.M)
        text = re.sub(r'^.*--guidiff "\$java" "\$rust".*$', '  python "$root/tools/oracle10/compare_gui_strict.py" "$root/tools/oracle10/gui/$f-scenes.txt" "$java" "$rust" "$root/shots10/${f}_diff" || fail=1', text, flags=re.M)
        text = re.sub(r'^  python .*gui_sections10.py.*\n', '', text, flags=re.M)
        if path.name == 'verify_containers181.sh':
            text = text.replace('set -euo pipefail','set -uo pipefail')
            text = re.sub(r'cargo run --release -q -p worldgen10 --bin (\w+) -- ', r'r10_compare "$root/target/release/\1" ', text)
            text = text.replace('python tools/oracle10/container_cases.py','fail=0\npython tools/oracle10/container_cases.py || exit 1\necho "== containers10: clicks and recipes"')
            text = text.replace('tools/oracle10/oracle.sh containerreplay', 'if tools/oracle10/oracle.sh containerreplay')
            text = text.replace(' container-replay.bin\nr10_compare', ' container-replay.bin; then\nr10_compare')
            text = text.replace('tools/oracle10/out/container-replay.bin\n', 'tools/oracle10/out/container-replay.bin || fail=1\nelse echo "ORACLE FAILED: container replay"; fail=1; fi\necho "== containers10: furnace and dispenser tiles"\n')
            text = text.replace('tools/oracle10/oracle.sh containertiles', 'if tools/oracle10/oracle.sh containertiles')
            text = text.replace(' container-tiles.bin\nr10_compare', ' container-tiles.bin; then\nr10_compare')
            text = text.replace('tools/oracle10/out/container-tiles.bin\n', 'tools/oracle10/out/container-tiles.bin || fail=1\nelse echo "ORACLE FAILED: container tiles"; fail=1; fi\nexit "$fail"\n')
        text = text.replace('"$oracle"', 'r10_oracle "$oracle"')
        text = text.replace('bash r10_oracle ', 'r10_oracle bash ')
        text = re.sub(r'(?m)^(\s*echo "==[^\n]+)', lambda m:m[0].replace('181','10'), text)
        if path.name == 'verify_player_gui181.sh':
            text=text.replace('  echo \"RECIPE CATALOG DIFFERS\"; fail=1', '  echo \"RECIPE CATALOG DIFFERS\"; python \"$root/tools/r10/diagnose_dump10.py\" tools/oracle10/out/player-gui3-recipes.bin crates/worldgen10/src/container/recipes.bin; fail=1')
            text=text.replace('    sys.exit(1)', "    sys.path.insert(0,str(Path('tools/r10').resolve()))\n    from diagnose_dump10 import diagnose\n    print('FIRST DIFFERENCE: '+diagnose(Path(java),Path(rust)))\n    sys.exit(1)")
        if path.name == 'verify_playertick181.sh':
            text = text.replace('set -euo pipefail','set -uo pipefail')
            text = text.replace('for kind in ', 'fail=0\nfor kind in ', 1)
            text = text.replace('  bash tools/oracle10/oracle.sh ', '  echo "== playertick10: $kind, $ticks ticks"\n  if r10_oracle bash tools/oracle10/oracle.sh ')
            text = text.replace(' "$java"\n  r10_compare', ' "$java"; then\n  r10_compare')
            text = text.replace(' "$rust" "$java"\ndone', ' "$rust" "$java" || fail=1\n  else fail=1; fi\ndone\nexit "$fail"')
        if path.name == 'verify_statsync181.sh':
            text = text.replace('{ tail -5 \"$fixture/rust.log\"; exit 1; }','{ tail -5 \"$fixture/rust.log\"; python \"$root/tools/r10/diagnose_text10.py\" \"$fixture/expected.txt\" \"$fixture/actual.txt\"; exit 1; }')
            text = text.replace('fixture=$(mktemp', 'echo "== statsync10: receive, send and file writer"\nfixture=$(mktemp')
        if path.name == 'verify_terrain181.sh':
            text = text.replace('echo "== $label";', 'echo "== $label seed $1";')
        if path.name == 'verify_gui181.sh':
            text = text.replace('bash "$root/tools/oracle10/guishot.sh" "$root/tools/oracle10/gui/scenes.txt"', 'echo "== gui10: oracle capture and HUD brightness hints"\nbash "$root/tools/oracle10/guishot.sh" "$root/tools/oracle10/gui/scenes.txt"', 1)
            text = text.replace('"$root/target/release/beta" --mc 1.0 --texturecheck', 'echo "== gui10: texture animation bytes"\n"$root/target/release/beta" --mc 1.0 --texturecheck')
        destination = Path(os.environ.get('VERIFY10_SCRIPT_DIR', path.parent))
        destination.mkdir(parents=True, exist_ok=True)
        (destination / path.name.replace('181', '10')).write_text(text, newline='\n')
    print('Ported', len(paths), 'suites and', len(binaries), 'binary names; retained Rust fixture mode arguments')


if __name__ == '__main__':
    main()
