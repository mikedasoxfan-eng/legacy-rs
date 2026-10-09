import base64,struct,sys
from pathlib import Path
out=bytearray();count=0
decode=lambda s:base64.b64decode(s).decode('utf8')
def text(s):
 b=s.encode('utf8');out.extend(struct.pack('>I',len(b)));out.extend(b)
for line in Path(sys.argv[1]).read_text().splitlines():
 code,key,variant,fmt,status,value=line.split('\t');v=int(variant)
 if v>3:continue
 args=[[],['Aé'],['Aé','Bß'],['Aé','Bß','C中','D🙂']][v]
 text(code);text(decode(key));text(decode(fmt));out.extend(struct.pack('>I',len(args)))
 for arg in args:text(arg)
 text(status);text(decode(value));count+=1
Path(sys.argv[2]).write_bytes(struct.pack('>I',count)+out)
print(f'Prepared {count} original String-only results/exceptions')
