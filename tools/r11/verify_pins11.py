"""Verify all completion hash pins against staged Git blobs, independent of checkout line endings."""
import json,hashlib,subprocess
from pathlib import Path
R=Path(__file__).resolve().parents[2]
def read(p):return json.loads((R/p).read_text())
pins={}
def add(items):
 for path,digest in items.items():
  assert path not in pins or pins[path]==digest,('conflicting pin',path)
  pins[path]=digest
for hashes in read('tools/r11/source-snapshots.json').values():add(hashes)
for row in read('tools/r11/completion-review.json').values():add(row['rust_source_sha256'])
for row in read('tools/r11/sections.json').values():
 add({row['report']:row['report_sha256']})
 if 'stable_candidate_revalidation' in row:
  r=row['stable_candidate_revalidation'];add({r['report']:r['sha256']})
r=read('reports/r11-progress/final-validation.json');add(r['source_sha256']);add(r['reports_sha256'])
paths=sorted(pins);query=''.join(':'+p+'\n' for p in paths).encode()
result=subprocess.run(['git','cat-file','--batch'],cwd=R,input=query,stdout=subprocess.PIPE,check=True).stdout
at=0
for path in paths:
 end=result.index(b'\n',at);header=result[at:end].decode();fields=header.split();assert len(fields)==3 and fields[1]=='blob',(path,header)
 n=int(fields[2]);at=end+1;body=result[at:at+n];at+=n+1;assert hashlib.sha256(body).hexdigest()==pins[path],('Git blob hash mismatch',path)
assert at==len(result)
print(f'GIT BLOB PINS EXACT: {len(paths)} committed-source/evidence hashes, zero mismatches')
