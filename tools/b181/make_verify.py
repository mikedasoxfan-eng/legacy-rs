"""Derive the b1.8.1 verify scripts (tools/*181.sh) from the b1.7.3 ones.

Same checks, pointed at tools/oracle181 (the b1.8.1 jar), the compare181 binary and
`beta --mc b1.8.1`. Every script takes a lock so two runs never share one output dir.
Re-run after changing a b1.7.3 verify script: python tools/b181/make_verify.py
"""
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
NAMES = ['verify', 'verify_containers', 'verify_decorations', 'verify_gui', 'verify_vehicles']
LOCK = '''# One run at a time per tree (two runs would overwrite each other's oracle output).
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p "$root/target"
  if ! mkdir "$lock" 2>/dev/null; then echo "another b1.8.1 verify is running ($lock); remove it if stale" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
beta() { "$root/target/release/beta" --mc b1.8.1 "$@"; }
'''


def convert(s):
    s = s.replace('tools/oracle/', 'tools/oracle181/').replace('oracle/', 'oracle181/').replace('oracle181181/', 'oracle181/')
    s = s.replace('release/compare"', 'release/compare181"').replace('release/compare ', 'release/compare181 ')
    s = s.replace('release/container_compare', 'release/container_compare181')
    s = s.replace('-p worldgen ', '-p worldgen181 ').replace('-p worldgen\n', '-p worldgen181\n').replace('-p worldgen|', '-p worldgen181|')
    for n in NAMES:
        s = s.replace(f'tools/{n}.sh', f'tools/{n}181.sh').replace(f'/{n}.sh"', f'/{n}181.sh"')
    s = s.replace('b1.7.3', 'b1.8.1')
    s = re.sub(r'^beta="\$root/target/release/beta"\n', '', s, flags=re.M)
    s = s.replace('"$beta"', 'beta').replace('$beta ', 'beta ')
    # Insert the lock + beta wrapper right after root= is set.
    s = re.sub(r'^(root="[^\n]*\n)', lambda m: m.group(1) + LOCK, s, count=1, flags=re.M)
    s = s.replace('saves/_verify', 'b1.8.1/saves/_verify')
    return s


for n in NAMES:
    src = os.path.join(ROOT, 'tools', n + '.sh')
    s = open(src, encoding='utf-8', newline='').read().replace('\r\n', '\n')
    out = os.path.join(ROOT, 'tools', n + '181.sh')
    open(out, 'w', encoding='utf-8', newline='\n').write(convert(s))
    left = [l for l in convert(s).split('\n') if re.search(r'tools/oracle/|\$beta\b|release/compare"', l)]
    print(n + '181.sh', 'leftover refs:', len(left))
    for l in left:
        print('   ', l.strip()[:120])
