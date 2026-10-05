"""Index == sections in a completed verify181 log without treating silent failures as exact."""
import json
import re
import sys
from pathlib import Path


def sections(path):
    result = []
    current = None
    raw = Path(path).read_bytes()
    # Windows PowerShell redirects external command output as UTF-16; bash uses UTF-8.
    log = raw.decode('utf-16' if raw.startswith((b'\xff\xfe', b'\xfe\xff')) else 'utf-8-sig', errors='replace')
    for line_number, line in enumerate(log.splitlines(), 1):
        if line.startswith('== '):
            current = {'label': line[3:], 'line': line_number, 'lines': []}
            result.append(current)
        elif current is not None:
            current['lines'].append((line_number, line))
    for section in result:
        failures = []
        exact = False
        for number, line in section.pop('lines'):
            if re.search(r'\bEXACT\b|\b0 differences\b|\b0 mismatches\b|test result: ok\. [1-9][0-9]* passed; 0 failed', line, re.I): exact = True
            if re.search(r'CASES DIFFER|FOCUSED CASES|SOME CASES DIFFER', line): continue
            if re.search(r'\b0 differ|\b0 differences|\b0 mismatches', line): continue
            if re.search(r'(?i)\bdiffers\b|\bdiffer at\b|DIFF:|Exception in thread|Caused by:|panicked|^error:|test result: FAILED|[1-9][0-9]* (?:mismatches|differences)|ORACLE FAILED', line):
                failures.append({'line': number, 'message': line})
        section['status'] = 'failed' if failures else 'exact' if exact else 'unverified'
        if failures: section['failure'] = failures[0]
    return result


if __name__ == '__main__':
    result = sections(sys.argv[1])
    counts = {status: sum(s['status'] == status for s in result) for status in ['exact', 'failed', 'unverified']}
    output = json.dumps({'total': len(result), **counts, 'sections': result}, indent=2)
    if len(sys.argv) > 2:
        Path(sys.argv[2]).write_text(output + '\n', encoding='utf-8')
    else:
        print(output)
