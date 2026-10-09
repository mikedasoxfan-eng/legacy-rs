"""Prepare unsealed LWJGL library copies for the isolated font GL recorder.

Only the Sealed manifest flag changes. Original Minecraft bytecode is untouched.
Original library jars remain untouched; all copies live under ignored target/.
"""
import argparse, hashlib, json, zipfile
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('mcp',type=Path);p.add_argument('output',type=Path);a=p.parse_args();a.output.mkdir(parents=True,exist_ok=True)
records=[]
for src in (a.mcp/'libraries/org/lwjgl').rglob('*.jar'):
 if 'natives' in src.name or 'sources' in src.name:continue
 dest=a.output/src.name
 with zipfile.ZipFile(src) as original,zipfile.ZipFile(dest,'w') as copied:
  for entry in original.infolist():
   data=original.read(entry.filename)
   if entry.filename.upper()=='META-INF/MANIFEST.MF':data=data.replace(b'Sealed: true',b'Sealed: false')
   copied.writestr(entry,data)
 records.append({'source':str(src),'source_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'copy':str(dest),'copy_sha256':hashlib.sha256(dest.read_bytes()).hexdigest(),'change':'manifest sealing only'})
(a.output/'provenance.json').write_text(json.dumps(records,indent=2)+'\n',encoding='utf-8')
print(f'Prepared {len(records)} isolated LWJGL libraries; original game jar untouched')
