"""Read-only complete original source/reference and bytecode-constant-pool audit.
Usage: python tools/oracle11/audit_dormant_ai11.py ORIGINAL_MCP OUTPUT_JSON
"""
import hashlib,json,re,struct,subprocess,sys,zipfile,os
from pathlib import Path
mcp=Path(sys.argv[1]);src=mcp/'minecraft/src/net/minecraft/src';jar=mcp/'minecraft/jars/deobfuscated.jar'
AI=['EntityAIAttackOnCollide','EntityAIBase','EntityAILookIdle','EntityAISwimming','EntityAITaskEntry','EntityAITasks','EntityAIWander','EntityAIWatchClosest','EntityJumpHelper','EntityLookHelper','EntityMoveHelper','INavigate','PathNavigate']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def cp(data):
 pos=8;count=struct.unpack_from('>H',data,pos)[0];pos+=2;pool=[None]*count;i=1
 while i<count:
  tag=data[pos];pos+=1
  if tag==1:n=struct.unpack_from('>H',data,pos)[0];pos+=2;value=data[pos:pos+n].decode('utf8','replace');pos+=n
  elif tag in [3,4]:value=None;pos+=4
  elif tag in [5,6]:value=None;pos+=8;pool[i]=(tag,value);i+=2;continue
  elif tag in [7,8,16,19,20]:value=struct.unpack_from('>H',data,pos)[0];pos+=2
  elif tag in [9,10,11,12,18]:value=struct.unpack_from('>HH',data,pos);pos+=4
  elif tag==15:value=None;pos+=3
  else:raise ValueError(tag)
  pool[i]=(tag,value);i+=1
 def utf(i):return pool[i][1]
 calls=[]
 for e in pool:
  if e and e[0] in [10,11]:
   c,nt=e[1];name,desc=pool[nt][1];calls.append((utf(pool[c][1]).split('/')[-1],utf(name),utf(desc)))
 # Inspect actual method tables, including overrides not mentioned by source calls.
 def u2():
  nonlocal pos
  x=struct.unpack_from('>H',data,pos)[0];pos+=2;return x
 def attrs():
  nonlocal pos
  out={}
  for _ in range(u2()):
   key=utf(u2());length=struct.unpack_from('>I',data,pos)[0];pos+=4;out[key]=data[pos:pos+length];pos+=length
  return out
 pos+=6
 count_interfaces=u2();pos+=2*count_interfaces
 for _ in range(u2()):pos+=6;attrs()
 declarations=[]
 for _ in range(u2()):
  access=u2();name=utf(u2());desc=utf(u2());a=attrs()
  if name=='func_46006_aR':
   code=a['Code'];length=struct.unpack_from('>I',code,4)[0];instructions=code[8:8+length];assert instructions==bytes([3,172]),instructions
   declarations.append({'method':name,'descriptor':desc,'code_hex':instructions.hex()})
 return calls,declarations
references=[]
bytecode_guard_declarations=[]
with zipfile.ZipFile(jar) as z:
 for n in sorted(z.namelist()):
  if not n.endswith('.class'):continue
  refs,decls=cp(z.read(n))
  bytecode_guard_declarations.extend(dict(d,class_name=n.split('/')[-1][:-6]) for d in decls)
  for c,method,desc in refs:
   if c in AI or method in ['func_46006_aR','func_46014_aS','func_46008_aG','func_46009_aH','func_46005_aI','func_46012_aJ','func_46007_aL','func_46010_f','func_46003_g','func_46013_aQ','func_46004_aK','func_46011_aM']:
    references.append({'caller_class':n.split('/')[-1][:-6],'target_class':c,'target_method':method,'descriptor':desc})
allowed=set(AI+['EntityLiving','EntityZombie','EntityPigZombie'])
external=[r for r in references if r['caller_class'] not in allowed]
assert not external,external
assert {d['class_name'] for d in bytecode_guard_declarations}=={'EntityLiving','EntityZombie','EntityPigZombie'}
source_files=list(src.glob('*.java'));guards=[];source_references={}
for p in source_files:
 text=p.read_text();
 for m in re.finditer(r'(?:protected|public)\s+boolean\s+func_46006_aR\(\)\s*\{([^}]+)\}',text):
  assert m.group(1).strip()=='return false;',p
  guards.append({'class_name':p.stem,'line':text[:m.start()].count('\n')+1,'body':m.group(1).strip(),'source_sha256':sha(p)})
 for token in AI+['func_46006_aR','func_46014_aS','field_46015_bf','field_46016_bg','field_46020_bQ','func_46004_aK','func_46011_aM']:
  for line_no,line in enumerate(text.splitlines(),1):
   if re.search(r'\b'+token+r'\b',line):source_references.setdefault(token,[]).append({'class_name':p.stem,'line':line_no,'source':line.strip()})
assert {g['class_name'] for g in guards}=={'EntityLiving','EntityZombie','EntityPigZombie'}
for yaw in ['field_46015_bf','field_46016_bg']:assert {r['class_name'] for r in source_references[yaw]}=={'EntityLiving'}
javap=Path(os.environ['JAVA_HOME'])/'bin/javap.exe'
classes=AI+['EntityLiving','EntityZombie','EntityPigZombie']
bc=subprocess.check_output([str(javap),'-classpath',str(jar),'-p','-c']+['net.minecraft.src.'+n for n in classes],text=True)
sections={}
for name in classes:
 start=bc.index('class net.minecraft.src.'+name) if name!='INavigate' else bc.index('interface net.minecraft.src.'+name)
 end=bc.find('Compiled from',start);sections[name]=bc[start:end if end>=0 else len(bc)]
for n in ['EntityLiving','EntityZombie','EntityPigZombie']:
 method=sections[n].split('boolean func_46006_aR();')[1].split('\n  ',1)[0] if False else re.search(r'boolean func_46006_aR\(\);\s+Code:(.*?)(?=\n  (?:public|private|protected)|\n})',sections[n],re.S).group(1)
 assert re.findall(r'\d+:\s+(\w+)',method)==['iconst_0','ireturn'],(n,method)
constructor_audit=[]
for n in AI:
 assert 'static {};' not in sections[n],n
 match=re.search(r'(?:public|protected|private) net\.minecraft\.src\.'+n+r'\([^;]*\);\s+Code:(.*?)(?=\n  (?:public|protected|private)|\n})',sections[n],re.S)
 code=match.group(1) if match else ''
 assert 'putstatic' not in code,n
 calls=re.findall(r'// (?:InterfaceMethod|Method) ([^\n]+)',code)
 assert not any('Random.' in c or 'World.' in c or 'Entity."<init>"' in c for c in calls),(n,calls)
 constructor_audit.append({'class_name':n,'source_sha256':sha(src/(n+'.java')),'static_initializer':False,'constructor_bytecode':code.strip(),'constructor_calls':calls,'effect_disposition':'Interface has no constructor' if n=='INavigate' else 'Only own fields, referenced position/speed reads, superclass Object/AIBase constructor, local ArrayList allocation and own mutex setter; no entity allocation/RNG/World call/static write'})
Path(sys.argv[2]).write_text(json.dumps({'schema':1,'original_jar_sha256':sha(jar),'source_file_count':len(source_files),'guard_declarations':guards,'all_jar_guard_declarations':bytecode_guard_declarations,'constant_pool_method_reference_scan':'Every class in original jar; conservative references may include unused CP entries, but no external callers occur','original_method_references':references,'external_callers':external,'source_references':source_references,'constructor_audit':constructor_audit,'original_javap_sha256':hashlib.sha256(bc.encode()).hexdigest(),'scope':'Source and original-bytecode reachability/effects proof. Missing Rust public helper APIs are not declared implemented.'},indent=2)+'\n')
print('ORIGINAL AI STATIC GRAPH EXACT:13 helper classes,3 constant-false virtual guard declarations,no external helper callers')
