"""Compile every 1.0 oracle entry point, including GUI and server fixtures."""
import argparse
import os
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
MCP = HERE.parent / 'retromcp-r11'


def compile_all(group='main'):
    jar = MCP / 'minecraft/jars/deobfuscated.jar'
    libs = [p for p in (MCP / 'libraries').rglob('*.jar') if 'sources' not in p.name and 'natives' not in p.name]
    build = HERE / 'build'
    build.mkdir(exist_ok=True)
    groups = {
        'main': ([p for p in (HERE / 'src').rglob('*.java') if p.name != 'ModelShot.java']
                 + list((HERE / 'headless').rglob('*.java')) + list((HERE / 'headless-input').rglob('*.java')), build),
        'visual': ([HERE / 'src/net/minecraft/src/ModelShot.java'], HERE / 'visual_build'),
        'gui': (list((HERE / 'gui').glob('*.java')), HERE / 'gui/build'),
        'server': (list((HERE / 'server/src').rglob('*.java')), HERE / 'server/build'),
    }
    sources, dest = groups[group]
    dest.mkdir(parents=True, exist_ok=True)
    if group == 'server':
        jar = MCP / 'minecraft_server/jars/deobfuscated.jar'
    cp = os.pathsep.join(map(str, [jar] + libs))
    args = ['-nowarn', '-Xmaxerrs', '2000', '-cp', cp, '-d', str(dest)] + list(map(str, sources))
    argfile = build / ('compile-' + group + '.args')
    argfile.write_text('\n'.join('"' + a.replace('\\', '/') + '"' for a in args))
    result = subprocess.run(['javac', '@' + str(argfile)], capture_output=True, text=True)
    log = ROOT / 'target' / ('javac11-' + group + '.log')
    log.parent.mkdir(exist_ok=True)
    log.write_text(result.stdout + result.stderr)
    errors = [l.replace(str(HERE) + os.sep, '') for l in result.stderr.splitlines() if ': error:' in l]
    print(group + ': ' + str(len(errors)) + ' errors (' + str(log) + ')')
    print('\n'.join(errors))
    return result.returncode


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('group', choices=['main', 'visual', 'gui', 'server'], default='main', nargs='?')
    raise SystemExit(compile_all(parser.parse_args().group))
