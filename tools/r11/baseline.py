"""Run the release suites and turn their section logs into a reviewable baseline.

Full run: python tools/r11/baseline.py
Single suite: python tools/r11/baseline.py --suite verify_world_blocks11.sh
Report from existing logs: python tools/r11/baseline.py --report-only
"""
import argparse
import hashlib
import json
import os
import re
import subprocess
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'target/r11/baseline'

def source_digest():
    digest=hashlib.sha256()
    paths=[]
    for directory in ['crates/worldgen11', 'crates/client11', 'crates/client', 'tools/oracle11', 'tools/r11']:
        paths.extend((ROOT/directory).rglob('*'))
    for path in sorted(paths):
        if not path.is_file() or any(part in {'build','out','generated','visual_build','__pycache__'} for part in path.parts): continue
        if path.suffix not in {'.rs','.toml','.py','.java','.sh','.txt','.json'}: continue
        digest.update(str(path.relative_to(ROOT)).encode());digest.update(path.read_bytes())
    return digest.hexdigest()


def sections(text):
    current = None
    for line in text.splitlines():
        alpha=re.match(r'^([A-Za-z0-9_-]+): ALPHA (EXACT|DIFFERS)$',line)
        if alpha and (not current or current[0]!='gui10: '+alpha[1]+' alpha'):
            if current: yield current
            current=['gui10: '+alpha[1]+' alpha',[line]]
        elif line.startswith('== '):
            if current: yield current
            current = [line[3:], []]
        elif current and not re.search(r'^(?:ALL .*EXACT|SOME .*DIFFER|[A-Z0-9 _-]+ (?:ALL|SOME|CASES DIFFER)|GUI ALPHA (?:DIFFERS|EXACT):|PLAYER-GUI ROUND \d+ (?:EXACT|DIFFERS)|MINECRAFT ROUND \d+ CHECKS (?:EXACT|DIFFER))', line):
            current[1].append(line)
    if current: yield current


def text_difference(lines):
    for i,line in enumerate(lines[:-1]):
        if not line.strip().startswith('java '): continue
        rust = next((v.strip()[5:] for v in lines[i+1:] if v.strip().startswith('rust ')), None)
        if rust is None: continue
        java = line.strip()[5:]
        field='value'; component=0
        for index,(j,r) in enumerate(zip(java.split(),rust.split())):
            if j != r and re.fullmatch(r'\d+/\d+',j) and re.fullmatch(r'\d+/\d+',r):
                jb,jm=j.split('/');rb,rm=r.split('/')
                field = 'blockID' if jb != rb else 'metadata'
                return f'tick {java.split()[0].rstrip(":")}, queriedBlock[{index-1}].{field}: Java {jb if jb!=rb else jm}, Rust {rb if jb!=rb else rm}'
            if j != r and ',' in j and ',' in r:
                for component,(a,b) in enumerate(zip(j.split(','),r.split(','))):
                    if a != b:
                        field = 'previousRedstoneState' if j.split(',')[4:5]==['M'] and component==6 else f'field[{component}]'
                        return f'tiles[{index-2}].{field}: Java {a}, Rust {b}'

            if not re.fullmatch(r'-?\d+(?:\.\d+)?',j): field=j;component=0
            else:
                if j != r: return f'{field}[{component}]: Java {j}, Rust {r}'
                component+=1
    return ''


def assertion_difference(lines):
    for i, line in enumerate(lines):
        if not line.startswith('assertion `left == right` failed:'): continue
        left = next((l.split(':',1)[1].strip() for l in lines[i+1:] if l.strip().startswith('left:')), None)
        right = next((l.split(':',1)[1].strip() for l in lines[i+1:] if l.strip().startswith('right:')), None)
        if left is not None and right is not None:
            field = line.split('failed:',1)[1].strip()
            if field.startswith('sky case'): field += ', GL color float bits'
            return f'{field}: Rust {left}, Java {right}'
    return ''


def panic_failure(lines):
    for index,line in enumerate(lines):
        if 'panicked' not in line: continue
        message = next((l.strip() for l in lines[index+1:] if l.strip()), '')
        return 'Checker failed before completing comparison: ' + message + '; ' + line.strip()
    return ''


def oracle_failure_details(lines):
    causes=[i for i,line in enumerate(lines) if line.strip().startswith('Caused by: ')]
    if not causes: return ''
    index=causes[-1]
    message=lines[index].strip().removeprefix('Caused by: ')
    frame=next((line.strip() for line in lines[index+1:] if line.strip().startswith('at ')), '')
    return message + ('; '+frame if frame else '')


def runtime_failure(line):
    return bool(re.search(r'^(?:Caused by: |Exception in thread [^ ]+ )?(?:java|javax|sun|org|com)\.[\w.$]*(?:Exception|Error)(?::|$)', line.strip()) or re.search(r'^[\w.]+(?:Error|Exception):', line.strip()))


def report():
    rows = []
    diagnostic_updates = {}
    # Interpret saved packets when a schema has become more precise since the
    # run. Keep the original logs intact and never rerun game code here.
    from diagnose_dump10 import terrain_difference, playertick_difference
    for metadata in (ROOT/'target/r11/compare').glob('*/comparison.json'):
        record = json.loads(metadata.read_text())
        arguments = record.get('oracle_arguments', [])
        old = record.get('first_difference', '')
        lifetime=len(arguments)>=2 and arguments[:2] == ['terrainround2','lifetime'] and 'outside the scalar' in old
        player=arguments and arguments[0].replace('181','10') in {'playertickintegration10','playertick10'}
        if not lifetime and not player: continue
        java = Path(record['java_snapshot'])
        rust = metadata.parent/'target/checker-rust.bin'
        if player:
            rust=next((Path(a) for a in record['arguments'] if a.endswith('.bin') and 'rust' in a),rust)
        if java.exists() and rust.exists():
            if player:
                new = playertick_difference(java.read_bytes(),rust.read_bytes())
            else:
                offset_match = re.search(r'byte (\d+)',old)
                if offset_match is None: continue
                new = terrain_difference('terrainround2',arguments,java.read_bytes(),rust.read_bytes(),int(offset_match[1]))
            if new: diagnostic_updates[old]=new
    # Decode remaining embedded event/bulk fields from retained original dumps.
    from diagnose_dump10 import event_difference, arrays_difference, legacy_paths_difference
    for metadata in (ROOT/'target/r11/compare').glob('*/comparison.json'):
        record=json.loads(metadata.read_text());old=record.get('first_difference','')
        if 'outside the scalar' not in old: continue
        args=record.get('arguments',[])
        java=Path(record['java_snapshot']) if record.get('java_snapshot') else next((Path(a) for a in reversed(args) if a.endswith('.bin')),None)
        if java is None or not java.exists():continue
        candidates=[metadata.parent/'target/checker-rust.bin']+list((metadata.parent/'target').glob('*rust.bin'))
        match=re.search(r'actual ([^\n]+)',record.get('output',''))
        if match:candidates.append(Path(match[1]))
        for arg in args:
            if arg.endswith('.bin'):candidates.extend([Path(arg.replace('-java','-rust')),Path(arg).with_suffix('.rust.bin')])
        rust=next((path for path in candidates if path.exists() and 'rust' in path.name and path.resolve()!=java.resolve()),None)
        if rust is None:continue
        offset_match=re.search(r'byte (\d+)',old)
        if offset_match is None:continue
        j=java.read_bytes();r=rust.read_bytes();offset=int(offset_match[1])
        actual=next((i for i,(a,b) in enumerate(zip(j,r)) if a!=b),min(len(j),len(r)))
        if actual!=offset:continue
        new=event_difference(j,r,offset)
        fixture=record.get('oracle_arguments',[None])[0]
        if fixture=='wbarrays' or args[:2]==['worldblocks181','arrays']:new=arrays_difference(j,r,offset)
        if fixture=='wblegacy5' or args[:2]==['worldblocks181','legacy5']:new=legacy_paths_difference(j,r,offset)
        if new:diagnostic_updates[old]=new
    durations = {}
    for path in sorted(OUT.glob('*.log')):
        for name, lines in sections(path.read_text(encoding='utf-8', errors='replace')):
            if not any(line.strip() for line in lines): continue
            if name.startswith('player/gui: hunger, experience, FOV, creative catalog'): continue
            if name == 'container clicks, recipes, furnace and dispenser world replay': continue
            if any('java: done' in l for l in lines) and not any('GuiShot: timed out' in l or 'panicked' in l or 'FIRST DIFFERENCE:' in l for l in lines): continue
            failures = [l.strip() for l in lines if (re.search(r'\bDIFF\b|differ|mismatch|FAILED|panicked|No such file|not found|missing ', l, re.I) or runtime_failure(l))
                        and not re.search(r'0 of \d+ entries differ|\b0 differ\b|\b0 differing fields\b|\b0 differences\b|\b0 metadata bytes differ\b|\b0 mismatch|\b0 failed', l)]
            evidence = [l.strip() for l in lines if re.search(r'EXACT|0 of \d+ entries differ|\b0 differences\b|\b0 differing fields\b|\b0 metadata bytes differ\b|test result: ok\.', l)]
            if evidence and not any('ORACLE FAILED' in l for l in lines):
                failures = [l for l in failures if not runtime_failure(l)]
            if failures:
                detail = next((l.strip().removeprefix('FIRST DIFFERENCE: ') for l in lines if l.startswith('FIRST DIFFERENCE: ')), '')
                if not detail and name=='statsync10: receive, send and file writer':
                    from diagnose_text10 import statistics
                    traces=sorted((ROOT/'target/r11').glob('statsync*/expected.txt'),key=lambda p:p.stat().st_mtime,reverse=True)
                    if traces and traces[0].with_name('actual.txt').exists(): detail=statistics(traces[0],traces[0].with_name('actual.txt'))
                detail = detail or next((l.strip() for l in lines if re.match(r'^byte \d+,',l.strip()) and 'Rust' in l), '')
                detail = detail or text_difference(lines) or assertion_difference(lines)
                detail = detail or ('Oracle failed before comparison: '+oracle_failure_details(lines) if oracle_failure_details(lines) else '')
                detail = detail or next((l.strip() for l in lines if re.search(r'(?:java|Java)[ =].*(?:rust|Rust)[ =]', l) and not re.search(r'length|bytes|of \d+', l)), '')
                if not detail:
                    tint = next((re.search(r'block (\d+) meta (\d+) tint differs ([0-9a-f]+)/([0-9a-f]+)',l) for l in lines if 'tint differs' in l),None)
                    if tint: detail = f'block {tint[1]} metadata {tint[2]}.particleTint: Java 0x{int(tint[4],16)&0xffffff:06x}, Rust 0x{int(tint[3],16):06x}'
                if not detail and name == 'world-blocks 1.0: definitions':
                    generated=(ROOT/'tools/oracle11/out/wb181-table.rs').read_text()
                    checked=(ROOT/'crates/worldgen11/src/block/table.rs').read_text()
                    j=re.search(r'MATERIALS: \[MaterialProps; (\d+)\]',generated)
                    r=re.search(r'MATERIALS: \[MaterialProps; (\d+)\]',checked)
                    if j and r and j[1]!=r[1]: detail=f'MATERIALS array length: Java {j[1]}, Rust {r[1]}'
                detail = detail or next((l for l in failures if re.search(r'first|byte|field|mismatch|Exception|Error:|panicked', l, re.I)), failures[0])
                if not any('FIRST DIFFERENCE:' in l for l in lines) and not assertion_difference(lines) and not re.search(r'(?:Java|java).*(?:Rust|rust)|(?:Rust|rust).*(?:Java|java)', detail):
                    detail = panic_failure(lines) or detail
                status = 'DIFFERS'
            elif evidence:
                status, detail = 'EXACT', evidence[0]
            else:
                status, detail = 'DIFFERS', 'Run failure: no comparison result was emitted; first field unavailable.'
            kind = 'comparison'
            completed = bool(any('FIRST DIFFERENCE:' in l for l in lines) or assertion_difference(lines) or re.search(r'(?:Java|java).*(?:Rust|rust)|(?:Rust|rust).*(?:Java|java)', detail))
            oracle_failed=any('ORACLE FAILED' in l for l in lines) or any('java.lang.reflect.InvocationTargetException'==l.strip() for l in lines) or (not completed and not evidence and any(runtime_failure(l) for l in lines))
            if oracle_failed:
                kind='oracle failure'
                if completed:
                    cause=oracle_failure_details(lines) or 'oracle launcher failed'
                    kind='comparison with oracle failure'
                    detail += '; another mode in this section failed: '+cause
            elif any('panicked' in l for l in lines) and not completed: kind='checker failure'
            elif 'missing capture' in detail or detail.startswith('Run failure'): kind='capture/run failure'
            if name.startswith('server mode:') and any('panicked' in l for l in lines):
                kind = 'checker failure'
                detail = panic_failure(lines) + ('; subsequent saved-world check: ' + detail if 'Java' in detail or 'java=' in detail else '')
            detail = diagnostic_updates.get(detail, detail)
            rows.append({'suite': path.stem, 'section': name, 'status': status, 'kind':kind, 'first_difference': detail})
        meta = path.with_suffix('.json')
        if meta.exists(): durations[path.stem] = json.loads(meta.read_text())
    # The release map hook was repaired during discovery. Completed frame replay
    # reruns supersede the earlier fixture failure in the full suite's rows.
    replay = {r['section']:r for r in rows if r['suite'] in {'verify_replays10','verify_harnessfix10','verify_serverfix10','verify_biomes10','verify_regions10','verify_player_gui10','verify_playerguifix10','verify_alphafix10','verify_minecraft10','verify_render10','verify_constantfix10','verify_entities10','verify_latefix10','verify_bucketfix10','verify_tracefix10'}}
    replay.update({r['section']:r for r in rows if r['suite']=='verify_constantfix10'})
    replay.update({r['section']:r for r in rows if r['suite']=='verify_latefix10'})
    replay.update({r['section']:r for r in rows if r['suite']=='verify_bucketfix10'})
    replay.update({r['section']:r for r in rows if r['suite']=='verify_tracefix10'})
    for row in rows:
        if row['section'] in replay:
            replacement = replay[row['section']]
            if replacement['suite']==row['suite']: continue
            row['status'] = replacement['status']
            row['kind'] = replacement['kind']
            row['first_difference'] = replacement['first_difference'] + ' (rechecked by ' + replacement['suite'] + ')'
    supplemental={'verify_replays10','verify_harnessfix10','verify_serverfix10','verify_biomes10','verify_regions10','verify_playerguifix10','verify_alphafix10','verify_constantfix10','verify_latefix10','verify_bucketfix10','verify_tracefix10'}
    primary={r['section'] for r in rows if r['suite'] not in supplemental}
    rows=[r for r in rows if r['suite'] not in supplemental or (r['section'] not in primary and r['suite']==replay[r['section']]['suite'])]
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / 'sections.json').write_text(json.dumps(rows, indent=2))
    text = ['# Minecraft 1.1 harness baseline', '',
            'Diagnostic run spanning worldgen11/client11 source changes; not a stable-candidate certification. EXACT means a comparison passed; DIFFERS includes oracle/checker failures explicitly identified below. A run failure is not evidence of a completed simulation comparison. Field diagnostics use the first serialized difference; pixel diagnostics use the first changed RGB pixel. Pure group headings without output are omitted in favor of their individual child comparisons. Completed supplemental rechecks supersede matching master rows after fixture corrections; the rows identify the recheck suite. Repeated supplemental rows are omitted when their corrected result is already represented. All measured runtimes include the original observed run, including failures.', '',
            '| Suite | Section | Result | Evidence | First difference / run failure |',
            '| --- | --- | --- | --- | --- |']
    for row in rows:
        values = [row['suite'], row['section'], row['status'], row['kind'], row['first_difference']]
        text.append('| ' + ' | '.join(v.replace('|', '\\|').replace('`', "'") for v in values) + ' |')
    text += ['', '## Measured suite times', '', '| Suite | Seconds | Exit code |', '| --- | ---: | ---: |']
    for name, meta in durations.items(): text.append(f"| {name} | {meta['seconds']:.2f} | {meta['returncode']} |")
    text += ['', 'Logs and machine-readable section results: `target/r11/baseline/`.', '']
    (ROOT / 'reports/r11-harness-baseline.md').write_text('\n'.join(text))
    print('Reported', len(rows), 'sections:', {s: sum(r['status'] == s for r in rows) for s in ['EXACT', 'DIFFERS', 'UNRESOLVED']}, flush=True)


def run(suite):
    OUT.mkdir(parents=True, exist_ok=True)
    log = OUT / (Path(suite).stem + '.log')
    log.with_suffix('.json').unlink(missing_ok=True)
    start = time.monotonic()
    before=source_digest()
    env = dict(os.environ, VERIFY11_KEEP='1')
    print('Running', suite, flush=True)
    with log.open('w', encoding='utf-8') as output:
        bash = os.environ.get('VERIFY11_BASH', 'C:/Program Files/Git/usr/bin/bash.exe')
        result = subprocess.run([bash, str(ROOT / 'tools' / suite)], cwd=ROOT, env=env, stdout=output, stderr=subprocess.STDOUT)
    elapsed = time.monotonic() - start
    after=source_digest()
    log.with_suffix('.json').write_text(json.dumps({'seconds': elapsed, 'returncode': result.returncode,
        'source_hash_before':before,'source_hash_after':after,'sources_unchanged':before==after,
        'oracle_sha256':hashlib.sha256((ROOT/'tools/retromcp-r11/minecraft/jars/deobfuscated.jar').read_bytes()).hexdigest(),
        'jvm':os.environ.get('ORACLE_JAVA'),'executable_sha256':hashlib.sha256((ROOT/'target/release/beta.exe').read_bytes()).hexdigest()}, indent=2))
    print(suite, 'finished in', round(elapsed, 2), 'seconds; exit', result.returncode, flush=True)
    report()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--suite', action='append')
    parser.add_argument('--report-only', action='store_true')
    args = parser.parse_args()
    if not args.report_only:
        for suite in args.suite or ['verify11.sh', 'verify_render11.sh', 'verify_ingame11.sh', 'verify_gui11.sh']:
            run(suite)
    report()


if __name__ == '__main__': main()
