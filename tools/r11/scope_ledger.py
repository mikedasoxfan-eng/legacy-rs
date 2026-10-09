"""Create a conservative class review ledger; no inferred or fuzzy verification credit."""
import argparse,csv,json
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('inventory',type=Path);p.add_argument('output',type=Path);args=p.parse_args()
inventory=json.loads(args.inventory.read_text(encoding='utf-8-sig'))
rows=[]
for entry in inventory['classes']:
    rows.append({'class_name':entry['class_name'],'area':entry['area'],'source_status':entry['status'],
      'raw_candidate':bool(entry.get('raw_changed_lines')) or entry['status']=='new',
      'raw_changed_lines':entry.get('raw_changed_lines',0),'strict_changed_lines':entry.get('conservative_changed_lines',0),
      'proposed_exclusion':entry.get('excluded',False),'review':'unreviewed','disposition':'pending',
      'authority':'','reason':'','rust_files':[],'behaviors':[],'notes':''})
for name in inventory['removed']:
    rows.append({'class_name':name,'area':'unclassified','source_status':'removed','raw_candidate':True,
      'raw_changed_lines':0,'strict_changed_lines':0,'proposed_exclusion':False,'review':'unreviewed',
      'disposition':'pending','authority':'','reason':'','rust_files':[],'behaviors':[],'notes':''})
if args.output.exists():
    previous={r['class_name']:r for r in json.loads(args.output.read_text(encoding='utf-8'))['classes']}
    for row in rows:
        old=previous.get(row['class_name'])
        if old:
            for field in ['review','disposition','authority','reason','rust_files','behaviors','notes']:row[field]=old[field]
data={'schema':1,'scope':'existing repository singleplayer scope; proposed exclusions require review',
      'completion_rule':'explicit reviewed dispositions and per-behavior required-check evidence on final candidate',
      'classes':rows}
args.output.parent.mkdir(parents=True,exist_ok=True)
args.output.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
print(f'{len(rows)} review rows; {sum(r["raw_candidate"] for r in rows)} raw candidates including proposed exclusions/removal; no automatic coverage credit')
