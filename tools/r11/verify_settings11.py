"""Compare complete saved options against original 1.1 GameSettings, headlessly."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import uuid

ROOT = Path(__file__).resolve().parents[2]


def run(args):
    return subprocess.run(args, cwd=ROOT, check=True, capture_output=True, text=True)


def main():
    java = os.environ['ORACLE_JAVA']
    version = run([java, '-version'])
    assert '1.8.0_51' in version.stderr, 'Requires pinned original Java 8u51'
    build = run(['cargo', 'build', '--release', '-p', 'worldgen11', '--lib', '--message-format=json'])
    artifacts = [json.loads(line) for line in build.stdout.splitlines() if line.startswith('{')]
    artifact = next(a for a in artifacts if a.get('reason') == 'compiler-artifact'
                    and 'worldgen11' in a['package_id'] and a['target']['kind'] == ['lib'])
    rlib = next(Path(f) for f in artifact['filenames'] if f.endswith('.rlib'))
    destination = ROOT / 'target/r11/settings' / uuid.uuid4().hex
    destination.mkdir(parents=True)
    font = (ROOT / 'crates/client11/src/gui/font.rs').read_text(encoding='utf-8')
    start = font.index('pub fn java_split')
    end = font.index('\n}\n', start) + 3
    # Use the actual production split function, avoiding a hand-written test double.
    source = '\n'.join(
        f'#[path={json.dumps((ROOT / f"crates/client11/src/gui/{name}.rs").as_posix())}] mod {name};'
        for name in ['lang', 'settings'])
    source += '\nmod font {\n' + font[start:end] + '\n}\n'
    source += '''fn main() {
        let args: Vec<_> = std::env::args().collect();
        let settings = settings::GameSettings::load(std::path::Path::new(&args[1]));
        settings.save();
    }\n'''
    probe = destination / 'probe.rs'
    probe.write_text(source, encoding='utf-8')
    executable = destination / ('probe.exe' if os.name == 'nt' else 'probe')
    run(['rustc', '--edition=2021', str(probe), '-o', str(executable),
         '--extern', 'worldgen=' + str(rlib), '-L', 'dependency=' + str(ROOT / 'target/release/deps')])
    jar = ROOT / 'tools/retromcp-r11/minecraft/jars/deobfuscated.jar'
    run(['javac', '-cp', str(jar), '-d', str(destination),
         'tools/oracle11/probes/SettingsProbe11.java'])
    results = []
    for index, case in enumerate(['', 'lang:he_IL\nlang:\n', 'lang:ja_JP:ignored\n',
                                 'lang:\nlang\nlang:ar_SA\n', 'lang:made_up\n']):
        directories = [destination / f'{index}-{side}' for side in ['java', 'rust']]
        for directory in directories:
            directory.mkdir()
            (directory / 'options.txt').write_text(case, encoding='utf-8')
        run([java, '-cp', str(destination) + os.pathsep + str(jar),
             'net.minecraft.src.SettingsProbe11', str(directories[0])])
        run([str(executable), str(directories[1])])
        expected, actual = [(p / 'options.txt').read_bytes() for p in directories]
        assert expected == actual, f'Options mismatch for case {index}: {case!r}'
        results.append({'case': case, 'bytes': len(actual),
                        'sha256': hashlib.sha256(actual).hexdigest()})
    (destination / 'results.json').write_text(json.dumps(results, indent=2) + '\n', encoding='utf-8')
    print('5 SETTINGS ORIGINAL BYTECODE EXACT')
    print(destination / 'results.json')


if __name__ == '__main__':
    main()
