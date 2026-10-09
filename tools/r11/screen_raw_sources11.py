"""Read-only screen of all308 raw-only SP classes; no completion accounting.
Usage: python work/screen_raw_only11.py
"""
from pathlib import Path
import json,re,difflib,hashlib,collections,argparse
repo=Path(__file__).resolve().parents[2]
parser=argparse.ArgumentParser();parser.add_argument("--old-mcp",type=Path,default=repo.parent/"retromcp-1.0");parser.add_argument("--new-mcp",type=Path,default=repo.parent/"retromcp-1.1");args=parser.parse_args()
inventory=repo/"tools/r11/changes.json";j=json.loads(inventory.read_text());scope=[c for c in j["classes"] if c["raw_changed_lines"] and not c["excluded"]];rows=[c for c in scope if c["status"]=="same"]
old=args.old_mcp/"minecraft/src/net/minecraft";new=args.new_mcp/"minecraft/src/net/minecraft"
def path(base,n):
 p=base/"src"/(n+".java")
 if not p.exists():p=base/"client"/(n+".java")
 assert p.exists(),n
 return p
literal=r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\''
def clean(s):
 # Preserve quoted URLs, including file:// and https://, while stripping comments.
 s=re.sub(literal+r"|/\*.*?\*/|//[^\n]*",lambda m:m.group() if m.group()[0] in [chr(34),chr(39)] else "",s,flags=re.S)
 return re.sub(r"^\s*import .*?;","",s,flags=re.M)
def tok(s):return re.findall(literal+r"|[A-Za-z_$][\w$]*|\d+(?:\.\d+)?[fFdDlL]?|[^\s]",clean(s))
keywords=set("abstract assert boolean break byte case catch char class const continue default do double else enum extends final finally float for goto if implements import instanceof int interface long native new package private protected public return short static strictfp super switch synchronized this throw throws transient try void volatile while true false null".split())
findings=[];safe=[];merges=[]
for c in rows:
 pa=path(old,c["class_name"]);pb=path(new,c["class_name"]);a=pa.read_text();b=pb.read_text();x=tok(a);y=tok(b);mapping={};reverse={};fail=[];collisions=[]
 if len(x)!=len(y):fail.append("token_count")
 else:
  for u,v in zip(x,y):
   if u==v:continue
   if re.fullmatch(r"[A-Za-z_$][\w$]*",u) and re.fullmatch(r"[A-Za-z_$][\w$]*",v) and u not in keywords and v not in keywords:
    if u in mapping and mapping[u]!=v:fail.append("inconsistent source identifier binding: "+u)
    if v in reverse and reverse[v]!=u:collisions.append({"new_name":v,"old_names":[reverse[v],u]})
    mapping[u]=v;reverse[v]=u
   else:fail.append("syntax/literal/keyword "+u+" -> "+v)
  if [mapping.get(t,t) for t in x]!=y:fail.append("whole-class identifier substitution residue")
 item={"class_name":c["class_name"],"area":c["area"],"raw_changed_lines":c["raw_changed_lines"],"issues":sorted(set(fail)),"identifier_map":mapping,"same_name_merge_warnings":collisions,"old_sha256":hashlib.sha256(pa.read_bytes()).hexdigest(),"new_sha256":hashlib.sha256(pb.read_bytes()).hexdigest(),"full_diff":"".join(difflib.unified_diff(a.splitlines(True),b.splitlines(True),n=2))}
 if fail:findings.append(item)
 else:safe.append(item)
 if collisions:merges.append(c["class_name"])
result={"schema":1,"inventory_path":str(inventory),"inventory_sha256":hashlib.sha256(inventory.read_bytes()).hexdigest(),"in_scope_raw_class_count":len(scope),"normalized_delta_classes":len(scope)-len(rows),"raw_only_count":len(rows),"consistent_entire_class_identifier_residue_count":len(safe),"requires_structural_review":findings,"same_name_merge_warning_classes":merges,"identifier_only_classes":safe,"limitation":"Exact syntax/literal/body structure under a consistent identifier map is a source screen. Same-name method/field merges and external callee binding changes require type/signature authority. No hidden behavior or inherited runtime completion is inferred automatically; no accounting changed."}
(repo/"reports/r11-progress/raw-source-screen11.json").write_text(json.dumps(result,indent=2));print("Raw SP",len(scope),"normalized",len(scope)-len(rows),"raw-only",len(rows),"consistent source residue",len(safe),"structural holds",len(findings),"merge warning classes",len(merges));print("\n".join(c["class_name"]+": "+str(c["issues"]) for c in findings))
