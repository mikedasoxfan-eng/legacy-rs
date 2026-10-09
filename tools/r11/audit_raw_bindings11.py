"""Compare raw-only original class bytecode bindings, descriptors and hierarchies.
Run from workspace: python work/legacy-rs/tools/r11/audit_raw_bindings11.py
No runtime/source/ledger writes. CP indexes/debug attributes are not semantics.
"""
from pathlib import Path
import struct,zipfile,json,collections,hashlib,difflib,argparse
REPO=Path(__file__).resolve().parents[2]
parser=argparse.ArgumentParser();parser.add_argument('--old-mcp',type=Path,default=REPO.parent/'retromcp-1.0');parser.add_argument('--new-mcp',type=Path,default=REPO.parent/'retromcp-1.1');args=parser.parse_args()
MCP={'1.0':args.old_mcp,'1.1':args.new_mcp}; OUT=REPO/'reports/r11-progress/raw-bindings11.json'
screen=json.loads((REPO/'reports/r11-progress/raw-source-screen11.json').read_text())
class Reader:
 def __init__(self,data):self.data=data;self.p=0
 def take(self,n):v=self.data[self.p:self.p+n];self.p+=n;return v
 def u1(self):return self.take(1)[0]
 def u2(self):return int.from_bytes(self.take(2),'big')
 def u4(self):return int.from_bytes(self.take(4),'big')
def parse(data):
 r=Reader(data); assert r.u4()==0xcafebabe;r.take(4);cp=[None]*r.u2();i=1
 while i<len(cp):
  tag=r.u1()
  if tag==1:
   raw=r.take(r.u2());cp[i]=(tag,raw.decode('utf8',errors='replace'),raw)
  elif tag in (3,4):cp[i]=(tag,r.take(4).hex())
  elif tag in (5,6):cp[i]=(tag,r.take(8).hex());i+=1
  elif tag in (7,8,16,19,20):cp[i]=(tag,r.u2())
  elif tag in (9,10,11,12,17,18):cp[i]=(tag,r.u2(),r.u2())
  elif tag==15:cp[i]=(tag,r.u1(),r.u2())
  else:raise ValueError(tag)
  i+=1
 def val(idx):
  if idx==0:return None
  x=cp[idx];tag=x[0]
  if tag==1:return x[1]
  if tag in (3,4,5,6):return x
  if tag==7:return (tag,val(x[1]))
  if tag==8:return (tag,cp[x[1]][2].hex()) # preserve modified UTF8 literals byte-exact
  if tag==12:return (val(x[1]),val(x[2]))
  if tag in (9,10,11):return (tag,val(x[1])[1],*val(x[2]))
  return x
 access=r.u2();owner=val(r.u2())[1];superclass=val(r.u2());interfaces=[val(r.u2())[1] for _ in range(r.u2())]
 def attrs():
  a={}
  for _ in range(r.u2()):name=val(r.u2());a[name]=r.take(r.u4())
  return a
 members={}
 for kind in ('f','m'):
  for _ in range(r.u2()):
   flags=r.u2();name=val(r.u2());desc=val(r.u2());a=attrs();m={'flags':flags,'attrs':a}
   if 'Code' in a:
    cr=Reader(a['Code']);m['stack']=cr.u2();m['locals']=cr.u2();m['code']=cr.take(cr.u4());m['handlers']=[tuple(cr.u2() for _ in range(4)) for _ in range(cr.u2())]
   members[kind,name,desc]=m
 attrs()
 return {'owner':owner,'super':superclass[1] if superclass else None,'interfaces':interfaces,'flags':access,'members':members,'cp':val,'sha256':hashlib.sha256(data).hexdigest()}
archives=[zipfile.ZipFile(MCP[v]/'minecraft/jars/deobfuscated.jar') for v in ('1.0','1.1')]
classes=[{n[:-6]:parse(a.read(n)) for n in a.namelist() if n.endswith('.class')} for a in archives]
rows=screen['identifier_only_classes']+screen['requires_structural_review'];aliases=collections.defaultdict(set);own={}
for row in rows:
 owner='net/minecraft/'+('client/' if row['class_name']=='Minecraft' else 'src/')+row['class_name'];own[owner]=row['identifier_map']
 if row['class_name'] not in ('CraftingManager','ThreadedChunkLoader'):
  for a,b in row['identifier_map'].items():aliases[a].add(b)
aliases['compareTo'].add('a') # Exact typed comparator rename, descriptor separates Object bridge.
# Renames from independently exact Block/Chunk source-residue review.
# Explicit pairs only, with original target declaration kind and descriptor required below.
for a,b in {'field_35472_a':'worldHeight','field_35473_b':'heightShift','field_35471_c':'xShift','func_35144_b':'updateContainingBlockInfo','func_35455_a':'markEntityForDespawn','func_40048_X':'getParts','func_25124_i':'removeUnknownBlocks','func_26002_a':'removeUnknownBlockIDs','func_35840_c':'getPrecipitationHeight','func_35841_j':'updateSkylight','func_40740_k':'getChunkCoordIntPair'}.items():aliases[a].add(b)
renames={};ambiguous=[]
for owner,old in classes[0].items():
 if owner not in classes[1]:continue
 new=classes[1][owner]
 for kind,name,desc in old['members']:
  if (kind,name,desc) in new['members']:target=name
  else:
   names=aliases.get(name,set())|({own[owner][name]} if name in own.get(owner,{}) else set())
   candidates=sorted(n for n in names if (kind,n,desc) in new['members'])
   if len(candidates)!=1:
    if candidates:ambiguous.append([owner,kind,name,desc,candidates])
    continue
   target=candidates[0]
  renames[owner,kind,name,desc]=target

def resolve(version,owner,kind,name,desc,seen=None):
 if seen is None:seen=set()
 if owner in seen:return None
 seen.add(owner);c=classes[version].get(owner)
 if c is None:return None
 if (kind,name,desc) in c['members']:return owner
 # JVMS field: own then interfaces then superclass. Method: own then superclass,
 # interfaces. Interface Object special cases remain external unchanged names.
 parents=c['interfaces']+([c['super']] if c['super'] else []) if kind=='f' else ([c['super']] if c['super'] else [])+c['interfaces']
 for parent in parents:
  found=resolve(version,parent,kind,name,desc,seen)
  if found:return found
 return None
bindings=[];unresolved=[];compiler_forms=[]
def cpval(version,c,index,caller,method,offset):
 v=c['cp'](index)
 if isinstance(v,tuple) and v[0] in (9,10,11):
  tag,owner,name,desc=v;kind='f' if tag==9 else 'm';decl=resolve(version,owner,kind,name,desc)
  if version==0:
   target=renames.get((decl,kind,name,desc),name)
   resolved_new=resolve(1,owner,kind,target,desc)
   row={'caller':caller,'method':method,'bytecode_offset':offset,'tag':tag,'reference_owner':owner,'old_name':name,'new_name':target,'descriptor':desc,'old_declaration_owner':decl,'new_declaration_owner':resolved_new}
   if name!=target or (decl is not None and resolved_new!=decl):bindings.append(row)
   if decl is not None and resolved_new!=decl:unresolved.append(row)
   return (tag,owner,target,desc,decl)
  return (*v,decl)
 return v
# Decode all JVM instruction operands. Normalize CP index and instruction offset only.
length={op:1 for op in range(256)}
for op in (16,18,21,22,23,24,25,54,55,56,57,58,169,188):length[op]=2
for op in (17,19,20,132,*range(153,169),*range(178,185),187,189,192,193,198,199):length[op]=3
length.update({185:5,186:5,197:4,200:5,201:5})
def code(version,c,key,m):
 if 'code' not in m:return None
 data=m['code'];p=0;items=[];offsets={}
 while p<len(data):
  start=p;op=data[p];p+=1;args=None
  if op in (170,171):
   while p%4:p+=1
   def s4():
    nonlocal p
    v=int.from_bytes(data[p:p+4],'big',signed=True);p+=4;return v
   default=s4()
   if op==170:lo=s4();hi=s4();args=('table',lo,hi,[start+default]+[start+s4() for _ in range(hi-lo+1)])
   else:n=s4();args=('lookup',start+default,[(s4(),start+s4()) for _ in range(n)])
  elif op==196:
   sub=data[p];n=5 if sub==132 else 3;args=list(data[p:p+n]);p+=n
  else:
   n=length[op]-1;raw=data[p:p+n];p+=n
   if op in (18,19,20,*range(178,187),187,189,192,193,197):
    idx=raw[0] if op==18 else int.from_bytes(raw[:2],'big');args=[cpval(version,c,idx,c['owner'],list(key),start),*raw[(1 if op==18 else 2):]]
    if op==19:op=18 # same ldc semantic, width depends constant pool layout
   elif op in (*range(153,169),198,199,200,201):args=('branch',start+int.from_bytes(raw,'big',signed=True))
   else:args=list(raw)
  offsets[start]=len(items);items.append([op,args])
 offsets[len(data)]=len(items)
 for item in items:
  a=item[1]
  if isinstance(a,tuple) and a[0]=='branch':item[1]=['branch',offsets[a[1]]]
  elif isinstance(a,tuple) and a[0]=='table':item[1]=['table',a[1],a[2],[offsets[x] for x in a[3]]]
  elif isinstance(a,tuple) and a[0]=='lookup':item[1]=['lookup',offsets[a[1]],[(x,offsets[y]) for x,y in a[2]]]
 handlers=[(offsets[a],offsets[b],offsets[d],c['cp'](t)) for a,b,d,t in m['handlers']]
 # Both compilers implement list.remove(i--) with the SAME old int on stack.
 # Canonicalize ONLY proved local post-decrement sequences; all remaining ops retained.
 def iload(item):
  op,args=item
  return args[0] if op==21 else op-26 if 26<=op<=29 else None
 def istore(item):
  op,args=item
  return args[0] if op==54 else op-59 if 59<=op<=62 else None
 incoming=set()
 for op,a in items:
  if isinstance(a,list) and a and a[0]=='branch':incoming.add(a[1])
  elif isinstance(a,list) and a and a[0]=='table':incoming.update(a[3])
  elif isinstance(a,list) and a and a[0]=='lookup':incoming.add(a[1]);incoming.update(y for x,y in a[2])
 for a,b,d,t in handlers:incoming.update((a,b,d))
 out=[];remap={};i=0
 while i<len(items):
  local=iload(items[i]);n=0
  if local is not None:
   if i+4<len(items) and items[i+1:i+4]==[[89,[]],[4,[]],[100,[]]] and istore(items[i+4])==local:n=5
   elif i+1<len(items) and items[i+1]==[132,[local,255]]:n=2
  if n and not any(j in incoming for j in range(i+1,i+n)):
   remap[i]=len(out);out.append(['postdecrement-int-local',local]);compiler_forms.append({'version':version,'class':c['owner'],'method':list(key),'instruction':i,'original_instruction_count':n,'local':local});i+=n
  else:remap[i]=len(out);out.append(items[i]);i+=1
 remap[len(items)]=len(out)
 for op,a in out:
  if isinstance(a,list) and a and a[0]=='branch':a[1]=remap[a[1]]
  elif isinstance(a,list) and a and a[0]=='table':a[3]=[remap[x] for x in a[3]]
  elif isinstance(a,list) and a and a[0]=='lookup':a[1]=remap[a[1]];a[2]=[(x,remap[y]) for x,y in a[2]]
 handlers=[(remap[a],remap[b],remap[d],t) for a,b,d,t in handlers]
 return {'instructions':out,'handlers':handlers,'locals':m['locals']}

def membervalue(version,c,key,m):
 value={'flags':m['flags'],'code':code(version,c,key,m)}
 for a in ('ConstantValue','Exceptions'):
  if a in m['attrs']:
   r=Reader(m['attrs'][a]);value[a]=c['cp'](r.u2()) if a=='ConstantValue' else [c['cp'](r.u2()) for _ in range(r.u2())]
 return value
results=[]
for row in rows:
 for v,key in [('1.0','old_sha256'),('1.1','new_sha256')]:
  source=MCP[v]/'minecraft/src/net/minecraft/src'/(row['class_name']+'.java')
  if not source.exists():source=MCP[v]/'minecraft/src/net/minecraft/client'/(row['class_name']+'.java')
  assert hashlib.sha256(source.read_bytes()).hexdigest()==row[key],(v,row['class_name'],'source snapshot drift')
 owner='net/minecraft/'+('client/' if row['class_name']=='Minecraft' else 'src/')+row['class_name'];old,new=(cs[owner] for cs in classes)
 differences=[];count=0
 if old['super']!=new['super'] or sorted(old['interfaces'])!=sorted(new['interfaces']) or old['flags']!=new['flags']:differences.append({'kind':'hierarchy_or_class_flags','old':[old['super'],old['interfaces'],old['flags']],'new':[new['super'],new['interfaces'],new['flags']]})
 targets=set()
 for key,m in old['members'].items():
  kind,name,desc=key;target=(kind,renames.get((owner,*key),name),desc);targets.add(target)
  if target not in new['members']:differences.append({'kind':'missing_member','old_key':key,'target_key':target});continue
  a=membervalue(0,old,key,m);b=membervalue(1,new,target,new['members'][target]);count+=1
  if a!=b:
   # Compact exact mismatch witnesses, retain complete normalized method values.
   differences.append({'kind':'member_bytecode_or_flags','old_key':key,'target_key':target,'old':a,'new':b})
 for key in new['members'].keys()-targets:differences.append({'kind':'new_member','new_key':key})
 results.append({'class_name':row['class_name'],'owner':owner,'old_class_sha256':old['sha256'],'new_class_sha256':new['sha256'],'original_hierarchy':{'superclass':old['super'],'interfaces':old['interfaces'],'flags':old['flags']},'new_hierarchy':{'superclass':new['super'],'interfaces':new['interfaces'],'flags':new['flags']},'members_compared':count,'exact':not differences,'differences':differences})
# Assert the ONLY residuals, without suppressing them from exact comparison.
residuals={c['class_name']:c for c in results if not c['exact']}
assert set(residuals)=={'EntityDragon','EnumArmorMaterial','CraftingManager'},sorted(residuals)
residue_assertions=[]
for name in ('EntityDragon','CraftingManager'):
 c=residuals[name];assert len(c['differences'])==1
 d=c['differences'][0];a,b=d['old'],d['new'];assert a['flags']==b['flags']
 changed=[i for i,(x,y) in enumerate(zip(a['code']['instructions'],b['code']['instructions'])) if x!=y]
 assert len(changed)==1 and len(a['code']['instructions'])==len(b['code']['instructions'])
 i=changed[0];ac=dict(a['code']);ac['instructions']=list(ac['instructions']);ac['instructions'][i]=b['code']['instructions'][i];assert ac==b['code']
 residue_assertions.append({'class_name':name,'exactly_one_changed_instruction':i,'old':a['code']['instructions'][i],'new':b['code']['instructions'][i],'entire_remaining_members_flags_code_handlers_exact':True})
c=residuals['EnumArmorMaterial'];assert len(c['differences'])==1
d=c['differences'][0];assert d['old_key']==('m','values','()[Lnet/minecraft/src/EnumArmorMaterial;') and d['old']['flags']==25 and d['new']['flags']==9 and d['old']['code']==d['new']['code']
assert c['original_hierarchy']['flags'] & 16 and c['new_hierarchy']['flags'] & 16
residue_assertions.append({'class_name':'EnumArmorMaterial','only_change':'values accessflag FINAL removed; PUBLIC STATIC retained; class FINAL retained; entire code and handlers identical'})
source_declarations={v:[owner for owner,c in classes[index].items() if ('m','getSourceOfDamage','()Lnet/minecraft/src/Entity;') in c['members']] for index,v in enumerate(('1.0','1.1'))}
assert source_declarations['1.0']==['net/minecraft/src/DamageSource'],source_declarations
def tiny_members(path):
 out={};owner=None
 for line in path.read_text().splitlines():
  t=line.split('\t')
  if t[0]=='c':owner=t[1]
  elif len(t)>4 and t[1] in ('f','m'):out[owner,t[1],t[3],t[2]]={'client_name':t[4],'server_name':t[5] if len(t)>5 else ''}
 return out
tiny=[tiny_members(MCP[v]/'conf/mappings.tiny') for v in ('1.0','1.1')]
for b in bindings:
 b['old_tiny_mapping']=tiny[0].get((b['old_declaration_owner'],'f' if b['tag']==9 else 'm',b['old_name'],b['descriptor']))
 b['new_tiny_mapping']=tiny[1].get((b['new_declaration_owner'],'f' if b['tag']==9 else 'm',b['new_name'],b['descriptor']))
def subtype(version,child,parent,seen=None):
 if child==parent:return True
 if seen is None:seen=set()
 if child in seen:return False
 seen.add(child);c=classes[version].get(child)
 return bool(c and any(subtype(version,x,parent,seen) for x in c['interfaces']+([c['super']] if c['super'] else [])))
dispatch=[];dispatch_holds=[]
unique={(b['reference_owner'],b['old_name'],b['new_name'],b['descriptor']) for b in bindings if b['tag'] in (10,11) and b['old_name'] not in ('<init>','<clinit>')}
for owner,a,b,desc in sorted(unique):
 # Enumerate all receivers in both ORIGINAL jars, not just the 308 caller classes.
 witnesses=[]
 for receiver in sorted(classes[0].keys() & classes[1].keys()):
  if not subtype(0,receiver,owner):continue
  od=resolve(0,receiver,'m',a,desc);nd=resolve(1,receiver,'m',b,desc)
  expected=renames.get((od,'m',a,desc),a)
  ok=subtype(1,receiver,owner) and od==nd and expected==b
  w={'receiver':receiver,'old_declaration_owner':od,'new_declaration_owner':nd,'expected_new_declaration_name':expected,'exact_binding':ok}
  witnesses.append(w)
  if not ok:dispatch_holds.append({'reference_owner':owner,'old_name':a,'new_name':b,'descriptor':desc,**w})
 dispatch.append({'reference_owner':owner,'old_name':a,'new_name':b,'descriptor':desc,'receiver_witnesses':witnesses})
report={'schema':1,'authority':'Original unchanged deobfuscated jars, both full member tables and superclass/interface graphs; owner/kind/name/descriptor symbolic bytecode comparison. Sources and aliases from exact raw-only source token screen. No name-only target acceptance.','jar_sha256':{v:hashlib.sha256((MCP[v]/'minecraft/jars/deobfuscated.jar').read_bytes()).hexdigest() for v in ('1.0','1.1')},'raw_only_classes':len(rows),'exact_classes':sum(x['exact'] for x in results),'difference_classes':[x['class_name'] for x in results if not x['exact']],'mapped_callsite_count':len(bindings),'binding_owner_changes_or_unresolved':unresolved,'ambiguous_declaration_aliases':ambiguous,'renamed_bindings':bindings,'virtual_dispatch_bindings':dispatch,'virtual_dispatch_holds':dispatch_holds,'tiny_mapping_sha256':{v:hashlib.sha256((MCP[v]/'conf/mappings.tiny').read_bytes()).hexdigest() for v in ('1.0','1.1')},'classes':results,'explicit_residue_assertions':residue_assertions,'getSourceOfDamage_original_jar_declarations':source_declarations,'compiler_postdecrement_forms':compiler_forms,'limits':['max_stack is a verified classfile capacity, not an observable instruction; both original classes execute valid bytecode. Only explicit local post-decrement sequences are canonicalized, with no branch/handler entering their interior. Full witnesses retained. Debug line/local-variable/source-file attributes intentionally ignored. JVM flags, superclass, interface set, fields/method descriptors, constants, opcode/operands, exception handlers and external owner/name/descriptor references retained. Interface order is treated as set: no SP getInterfaces consumer, Java7 lacks default interface methods.','This compares changed callers and declaration resolution; bodies of changed external callees belong to separately reviewed normalized deltas. No unchanged inherited public-API parity assertion.','External Java/LWJGL classes absent from original jars are retained with exact owner/name/descriptor; no renamed absent external declaration is accepted.']}
OUT.parent.mkdir(parents=True,exist_ok=True);OUT.write_text(json.dumps(report,indent=2));print('classes',len(rows),'exact',report['exact_classes'],'differences',report['difference_classes'],'mapped bindings',len(bindings),'unresolved',len(unresolved),'ambiguous declarations',len(ambiguous),'dispatch holds',len(dispatch_holds));print(OUT)
