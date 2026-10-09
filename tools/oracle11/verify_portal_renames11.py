"""Prove portal-related 1.1 deltas are exact symbol renames, not new behavior.
Usage: python tools/oracle11/verify_portal_renames11.py OLD_MCP NEW_MCP
"""
import hashlib,json,re,sys
from pathlib import Path
old=Path(sys.argv[1])/"minecraft/src/net/minecraft/src"
new=Path(sys.argv[2])/"minecraft/src/net/minecraft/src"
def sha(s):return hashlib.sha256(s.encode()).hexdigest()
def normalize(s,mapping):
 for a,b in mapping.items():s=re.sub(r"\b"+re.escape(a)+r"\b",b,s)
 return re.findall(r"[A-Za-z_$][\w$]*|\d+(?:\.\d+)?[FDfdLl]?|[^\s]",s)
def method(s,name):
 m=re.search(r"(?:public|protected|private)\s+boolean\s+"+name+r"\([^)]*\)\s*\{",s);assert m,name
 start=m.start();i=m.end();depth=1
 while depth:
  depth+=(s[i]=="{")-(s[i]=="}");i+=1
 return s[start:i]
renames={"field_41051_a":"bossDefeated","func_40212_d":"isEnderEyeInserted","func_35190_e":"canPlayerEdit","entityJoinedWorld":"spawnEntityInWorld"}
classes=[]
for name in ["BlockEndPortal","BlockEndPortalFrame","ItemEnderEye"]:
 a=(old/(name+".java")).read_text();b=(new/(name+".java")).read_text();x=normalize(a,renames);y=normalize(b,{})
 assert x==y,"Unexplained delta in "+name
 classes.append({"class_name":name,"disposition":"no_new_behavior_exact_symbol_rename","old_sha256":sha(a),"new_sha256":sha(b),"normalized_tokens_sha256":sha(json.dumps(x)),"token_count":len(x),"entire_class_token_equality":True,"rename_map":{k:v for k,v in renames.items() if re.search(r"\b"+k+r"\b",a)},"behavioral_oracle_claim":False})
deps=[]
for cls,a_name,b_name in [("EntityPlayer","func_35190_e","canPlayerEdit"),("World","entityJoinedWorld","spawnEntityInWorld")]:
 a=method((old/(cls+".java")).read_text(),a_name);b=method((new/(cls+".java")).read_text(),b_name);assert normalize(a,renames)==normalize(b,{})
 deps.append({"class":cls,"old_method":a_name,"new_method":b_name,"old_method_sha256":sha(a),"new_method_sha256":sha(b),"normalized_method_tokens_equal":True})
print(json.dumps({"schema":1,"scope":"Original-source normalization proof only; no new inherited gameplay oracle credit","classes":classes,"renamed_callee_authority":deps,"excluded_claims":["No new whole Java API proof","No claim inherited portal gameplay is fully integration-tested"]},indent=2))
