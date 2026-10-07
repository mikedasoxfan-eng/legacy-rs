"""End-to-end play tests of the real 1.0 client: every tools/playtest10/*.txt is a --play-script with
header lines `#! setup <scenario>`, `#! seed <n>` and `#! expect <shot> <x> <y> <z> <tolerance>`
(the player's position when that screenshot is taken; `*` skips an axis). A test fails if the game
panics, a screenshot is missing or a position is off. Screenshots go to target/playtest10/<test>/.
Usage: python tools/playtest10/run.py [test names...]"""
import os, re, subprocess, sys
root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
here = os.path.join(root, 'tools', 'playtest10')
exe = os.path.join(root, 'target', 'release', 'beta.exe')
out = os.path.join(root, 'target', 'playtest10')
names = sys.argv[1:] or sorted(f[:-4] for f in os.listdir(here) if f.endswith('.txt'))
failed = 0
for name in names:
    print(f'== playtest10: {name}', flush=True)
    text = open(os.path.join(here, name + '.txt'), encoding='utf-8').read()
    head = dict(re.findall(r'^#! (setup|seed) (\S+)', text, re.M))
    expects = re.findall(r'^#! expect (\S+) (\S+) (\S+) (\S+) (\S+)', text, re.M)
    os.makedirs(out, exist_ok=True)
    script = os.path.join(out, name + '.txt')
    open(script, 'w', encoding='utf-8').write(text)
    args = [exe, '--mc', '1.0', '--no-save', '--seed', head.get('seed', '1'), '--play-script', script]
    if 'setup' in head:
        args += ['--setup', head['setup']]
    try:
        r = subprocess.run(args, cwd=out, capture_output=True, text=True, encoding='utf-8', errors='replace', timeout=600)
        log = r.stdout + r.stderr
    except subprocess.TimeoutExpired:
        log, r = 'TIMEOUT', None
    open(os.path.join(out, name + '.log'), 'w', encoding='utf-8').write(log)
    problems = []
    if 'panicked' in log or log == 'TIMEOUT' or (r and r.returncode != 0):
        problems.append('the game crashed or hung (see target/playtest10/%s.log)' % name)
    if 'unknown scenario' in log:
        problems.append('unknown scenario ' + head.get('setup', ''))
    pos = {m[0]: tuple(map(float, m[1:4])) for m in re.findall(r'^script (\S+): pos (\S+) (\S+) (\S+)', log, re.M)}
    for shot, *want in expects:
        if shot not in pos:
            problems.append(f'no screenshot {shot}')
            continue
        tol = float(want[3])
        for axis, w, g in zip('xyz', want[:3], pos[shot]):
            if w != '*' and abs(float(w) - g) > tol:
                problems.append(f'{shot}: {axis} is {g}, expected {w}')
    if problems:
        failed += 1
        print('DIFFERS: playtest10 ' + name + ': ' + '; '.join(problems))
    else:
        print(f'EXACT: playtest10 {name} ({len(expects)} positions, no crash)')
print('ALL PLAY TESTS PASS' if not failed else f'{failed} PLAY TESTS FAIL')
sys.exit(1 if failed else 0)
