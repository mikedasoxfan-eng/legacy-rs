"""Exact line-weighted progress, following r10/progress.py's class accounting.

Only a reviewed complete 1.1 class delta earns credit. ported.json records its
implementation commit; coverage.json names every required section. Saved results
are invalidated when their source snapshot or evidence report changes. Declaration
only changes receive the same no-behavior credit supported by r10. No fuzzy mapping.
"""
import argparse, hashlib, json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
TEXT_SUFFIXES={'.rs','.txt','.toml','.json','.py','.ps1','.java','.log','.md','.yml','.yaml','.sh','.tsv','.csv'}
def pinned_digest(path):
 data=path.read_bytes()
 # Git stores text as LF (.gitattributes); terminal tools may write CRLF locally.
 # Normalize only text line endings so checkout platform cannot invalidate evidence.
 if path.suffix.lower() in TEXT_SUFFIXES:data=data.replace(b'\r\n',b'\n')
 return hashlib.sha256(data).hexdigest()

def main():
 p=argparse.ArgumentParser();p.add_argument('logs',nargs='*',type=Path);p.add_argument('--output',type=Path);a=p.parse_args()
 # Import only the original section parser, without executing r10 main().
 source=(ROOT/'tools/r10/progress.py').read_text(encoding='utf-8')
 namespace={'__file__':str(ROOT/'tools/r10/progress.py'),'__name__':'r10_parser'}
 exec(source.rsplit('\nmain()',1)[0],namespace)
 evidence_path=ROOT/'tools/r11/sections.json'
 evidence=json.loads(evidence_path.read_text(encoding='utf-8')) if evidence_path.exists() else {}
 snapshots_path=ROOT/'tools/r11/source-snapshots.json'
 snapshots=json.loads(snapshots_path.read_text(encoding='utf-8')) if snapshots_path.exists() else {}
 def unchanged(hashes):
  return bool(hashes) and all((ROOT/path).is_file() and pinned_digest(ROOT/path)==digest for path,digest in hashes.items())
 valid={name: unchanged(snapshots.get(record.get('source_snapshot'),record.get('rust_source_sha256',{}))) and unchanged({record['report']:record['report_sha256']}) for name,record in evidence.items()}
 exact={name: record.get('exact') is True and valid[name] for name,record in evidence.items()}
 for log in a.logs:
  parsed=namespace['exact_sections'](str(log))
  # Additional logs can invalidate registered evidence, never invent coverage.
  for name,passed in parsed.items():
   if name in exact:exact[name]=exact[name] and passed
 changes=json.loads((ROOT/'tools/r11/changes.json').read_text(encoding='utf-8'))['classes']
 coverage=json.loads((ROOT/'tools/r11/coverage.json').read_text(encoding='utf-8'))
 ported=json.loads((ROOT/'tools/r11/ported.json').read_text(encoding='utf-8'))
 review_path=ROOT/'tools/r11/completion-review.json'
 reviews=json.loads(review_path.read_text(encoding='utf-8')) if review_path.exists() else {}
 rows=[r for r in changes if r['status']!='same' and not r['excluded']]
 areas={};done_classes=[]
 for row in rows:
  name=row['class_name'];commit=ported.get(name);sections=coverage.get(name,[])
  committed=bool(commit) and subprocess.run(['git','merge-base','--is-ancestor',commit,'HEAD'],cwd=ROOT,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL).returncode==0
  review=reviews.get(name,{})
  reviewed=review.get('complete_delta_review') is True and unchanged(review.get('rust_source_sha256',{}))
  no_delta=review.get('no_behavior_change') is True and bool(review.get('reason'))
  done=committed and reviewed and (no_delta or (bool(sections) and all(exact.get(s,False) for s in sections)))
  area=areas.setdefault(row['area'],{'done_lines':0,'total_lines':0,'done_classes':0,'total_classes':0})
  area['total_lines']+=row['changed_lines'];area['total_classes']+=1
  if done:area['done_lines']+=row['changed_lines'];area['done_classes']+=1;done_classes.append(name)
 numerator=sum(r['done_lines'] for r in areas.values());denominator=sum(r['total_lines'] for r in areas.values())
 result={'method':'r10 normalized changed-line weighting; complete 1.1 delta review, committed implementation, explicit oracle coverage and source-pinned EXACT results','candidate_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'done_lines':numerator,'total_lines':denominator,'percentage':100*numerator/denominator,'done_classes':len(done_classes),'total_classes':len(rows),'credited_classes':done_classes,'areas':areas,'exact_sections':sum(exact.values()),'observed_sections':len(exact),'limitations':['Normalized inventory may hide changes; conservative raw behavior audit remains required.','This is verified completion credit, not percentage of code written; unreviewed or partially verified classes receive zero credit.','A checkpoint commit alone gives no completion credit.','Unfinished client integration, final suite, older-version regressions and packaging remain completion gates.']}
 print(json.dumps(result,indent=2))
 if a.output:a.output.write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
if __name__=='__main__':main()
