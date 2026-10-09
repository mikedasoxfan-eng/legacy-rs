"""Exact line-weighted progress, following r10/progress.py's class accounting.

Only explicit whole-class coverage earns credit. ported.json records each completed
class's commit; coverage.json names every required section. Pass current logs as
arguments. No fuzzy mapping, copied-file credit, or estimates.
"""
import argparse, importlib.util, json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def main():
 p=argparse.ArgumentParser();p.add_argument('logs',nargs='*',type=Path);p.add_argument('--output',type=Path);a=p.parse_args()
 # Import only the original section parser, without executing r10 main().
 source=(ROOT/'tools/r10/progress.py').read_text(encoding='utf-8')
 namespace={'__file__':str(ROOT/'tools/r10/progress.py'),'__name__':'r10_parser'}
 exec(source.rsplit('\nmain()',1)[0],namespace)
 exact={}
 for log in a.logs:exact.update(namespace['exact_sections'](str(log)))
 changes=json.loads((ROOT/'tools/r11/changes.json').read_text(encoding='utf-8'))['classes']
 coverage=json.loads((ROOT/'tools/r11/coverage.json').read_text(encoding='utf-8'))
 ported=json.loads((ROOT/'tools/r11/ported.json').read_text(encoding='utf-8'))
 rows=[r for r in changes if r['status']!='same' and not r['excluded']]
 areas={};done_classes=[]
 for row in rows:
  name=row['class_name'];commit=ported.get(name);sections=coverage.get(name,[])
  committed=bool(commit) and subprocess.run(['git','merge-base','--is-ancestor',commit,'HEAD'],cwd=ROOT,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL).returncode==0
  done=committed and bool(sections) and all(exact.get(s,False) for s in sections)
  area=areas.setdefault(row['area'],{'done_lines':0,'total_lines':0,'done_classes':0,'total_classes':0})
  area['total_lines']+=row['changed_lines'];area['total_classes']+=1
  if done:area['done_lines']+=row['changed_lines'];area['done_classes']+=1;done_classes.append(name)
 numerator=sum(r['done_lines'] for r in areas.values());denominator=sum(r['total_lines'] for r in areas.values())
 result={'method':'r10 normalized changed-line weighting; explicit whole-class credit and all required oracle sections EXACT','done_lines':numerator,'total_lines':denominator,'percentage':100*numerator/denominator,'done_classes':len(done_classes),'total_classes':len(rows),'areas':areas,'exact_sections':sum(exact.values()),'observed_sections':len(exact),'limitations':['Normalized inventory may hide changes; conservative raw behavior audit remains required.','This is certified whole-class credit, not percentage of code written.','A checkpoint commit alone gives no completion credit.']}
 print(json.dumps(result,indent=2))
 if a.output:a.output.write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
if __name__=='__main__':main()
