import os
os.chdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
SUMMARY = """    elif cur is not None and re.match(rb'^(ALL CASES EXACT|SOME CASES DIFFER|exit [0-9]+)[ \\t\\r]*$', line):
        cur = None  # the suite's own summary line, not part of the last section
"""
# sections.py: module-level loop (4-space indent)
p = 'tools/b181/sections.py'
s = open(p, encoding='utf-8').read()
s = s.replace("    elif cur is not None:\n", SUMMARY + "    elif cur is not None:\n", 1)
# A section that printed nothing bad is exact (some sections only print on a difference).
s = s.replace("ok = [s for s in sections if s[1] == 0 and s[2] > 0]", "ok = [s for s in sections if s[1] == 0]")
s = s.replace("bad = [s for s in sections if not (s[1] == 0 and s[2] > 0)]", "bad = [s for s in sections if s[1] != 0]")
s = s.replace("print(('  ok  ' if (nbad == 0 and ngood > 0) else '  XX  ') + name)", "print(('  ok  ' if nbad == 0 else '  XX  ') + name)")
open(p, 'w', encoding='utf-8').write(s)
p = 'tools/b181/progress.py'
s = open(p, encoding='utf-8').read()
s = s.replace("        elif cur is not None:\n", SUMMARY.replace('    ', '        ', 1).replace('\n        cur = None', '\n            cur = None') + "        elif cur is not None:\n", 1)
s = s.replace("return {k: v[0] == 0 and v[1] > 0 for k, v in out.items()}", "return {k: v[0] == 0 for k, v in out.items()}")
open(p, 'w', encoding='utf-8').write(s)
print('ok')
