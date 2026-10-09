"""Check inherited fixture seeds, numeric probes and binary headers after porting."""
import re
from collections import Counter
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]


def numeric(text):
    text=re.sub(r'"(?:\\.|[^"\\])*"|//[^\n]*|/\*[\s\S]*?\*/',lambda m:' '*len(m[0]),text)
    return Counter(re.findall(r'(?<![\w.])(?:0x[0-9a-fA-F]+|\d+)(?:[lLfFdD])?\b',text))


def main():
    errors=[];checked=0
    for original in (ROOT/'tools/oracle181').rglob('*.java'):
        relative=original.relative_to(ROOT/'tools/oracle181')
        if any(part in {'build','build_try','out','generated'} for part in relative.parts):continue
        release=ROOT/'tools/oracle11'/str(relative).replace('181','10')
        if not release.exists():continue
        a,b=original.read_text(),release.read_text();na,nb=numeric(a),numeric(b)
        for value,count in na.items():
            if '181' in value and count!=nb[value]:errors.append(f'{release.name}: {value} original {count}, release {nb[value]}')
        for header in set(re.findall(r'"B181[A-Z0-9]+"',a)):
            if header not in b:errors.append(f'{release.name}: missing inherited header {header}')
        checked+=1
    if errors:raise SystemExit('\n'.join(errors))
    print(f'Fixture constant audit passed for {checked} original/release source pairs')


if __name__=='__main__':main()
