"""Annotate fixture scalar loops for the opt-in original-bytecode dump locator.

The primitive values, their evaluation order, and their serialization stay the
same. ORACLE10_TRACE_BYTE enables the locator; normal oracle runs emit no trace.
This script is idempotent and only edits tools/oracle10 Java fixture sources.
"""
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'tools/oracle10/src/net/minecraft/src'


def split_args(text):
    result = []
    level = 0
    start = 0
    for i, c in enumerate(text):
        if c in '([': level += 1
        if c in ')]': level -= 1
        if c == ',' and not level:
            result.append(text[start:i].strip()); start = i + 1
    result.append(text[start:].strip())
    return result


def main():
    pattern = re.compile(r'for\s*\(\s*(double|float|int|long)\s+(\w+)\s*:\s*new\s+\1\[\]\s*\{([^{}]+)\}\s*\)\s*(?:EntityOracle\.)?[idfl]\(\s*\w+\s*,\s*\2\s*\)', re.S)
    count = 0
    for path in SOURCE.glob('*.java'):
        text = path.read_text()
        def annotate(match):
            nonlocal count
            if 'Trace10.named' in match[3]: return match[0]
            typ = match[1].capitalize()
            if typ == 'Int': typ = 'Int'
            names = split_args(match[3])
            replacement = ','.join(f'Trace10.named{typ}({json.dumps(n)}, {n})' for n in names)
            count += 1
            at = match[0].index(match[3])
            return match[0][:at] + replacement + match[0][at+len(match[3]):]
        text = pattern.sub(annotate, text)
        if path.name.startswith('Mode_'):
            # Label scalar expressions even when several writes share a line.
            calls = list(re.finditer(r'(?<![\w.])(?:EntityOracle\.)?([idfl])\(\s*(\w+)\s*,',text))
            for call in reversed(calls):
                start=call.end();end=start;depth=1;quoted=False;escaped=False
                while end<len(text) and depth:
                    char=text[end]
                    if char=='"' and not escaped:quoted=not quoted
                    if not quoted:
                        if char=='(':depth+=1
                        elif char==')':depth-=1
                    escaped=char=='\\' and not escaped;end+=1
                value=text[start:end-1].strip()
                simple=re.fullmatch(r'Trace10.named(?:Int|Long|Float|Double)\("(\w+)",\s*\1\)',value)
                if simple:
                    text=text[:start]+simple[1]+text[end-1:];continue
                if re.fullmatch(r'\w+',value) or 'Trace10.named' in value or len(split_args(value))!=1:continue
                typ={'i':'Int','d':'Double','f':'Float','l':'Long'}[call[1]]
                text=text[:start]+f'Trace10.named{typ}({json.dumps(value)}, {value})'+text[end-1:]
        if path.name != 'Trace10.java':
            # DataOutputStream booleans bypass the inherited scalar helpers.
            calls = list(re.finditer(r'\b(\w+)\.write(Boolean|Int|Long|Float|Double|Short|Byte)\(', text))
            for call in reversed(calls):
                if call[1] == 'Trace10': continue
                start = call.end(); end = start; depth = 1; quoted = False; escaped = False
                while end < len(text) and depth:
                    char = text[end]
                    if char == '"' and not escaped: quoted = not quoted
                    if not quoted:
                        if char == '(': depth += 1
                        elif char == ')': depth -= 1
                    escaped = char == '\\' and not escaped
                    end += 1
                value = text[start:end-1]
                text = text[:call.start()] + f'Trace10.write{call[2]}({call[1]}, {json.dumps(value)}, {value})' + text[end:]
        if path.name.startswith('Mode_'):
            calls=list(re.finditer(r'\b(out|o|p|states)\.write\(',text))
            for call in reversed(calls):
                start=call.end();end=start;depth=1;quoted=False;escaped=False
                while end<len(text) and depth:
                    char=text[end]
                    if char=='"' and not escaped:quoted=not quoted
                    if not quoted:
                        if char=='(':depth+=1
                        elif char==')':depth-=1
                    escaped=char=='\\' and not escaped;end+=1
                value=text[start:end-1].strip()
                if len(split_args(value))!=1:continue
                text=text[:call.start()]+f'Trace10.writeBytes({call[1]}, {json.dumps(value)}, {value})'+text[end:]
        path.write_text(text, newline='\n')
    print('Annotated', count, 'scalar loops')


if __name__ == '__main__': main()
