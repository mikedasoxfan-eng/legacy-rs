"""Read-only original Block/Chunk complete source residue proof.
Usage: python work/verify_block_chunk_sources11.py OLD_MCP NEW_MCP OUTPUT_JSON
"""
from pathlib import Path
import difflib,hashlib,json,re,sys
old=Path(sys.argv[1])/"minecraft/src/net/minecraft/src";new=Path(sys.argv[2])/"minecraft/src/net/minecraft/src"
def tokens(s):return re.findall(r"[A-Za-z_$][\w$]*|\d+(?:\.\d+)?[FDfdLl]?|[^\s]",s)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def method(s,name):
 m=re.search(r"^[ \t]*(?:public|protected|private)[^\n]*\b"+re.escape(name)+r"\([^;{]*\)\s*\{",s,re.M);assert m,name
 i=m.end();d=1
 while d:d+=(s[i]=="{")-(s[i]=="}");i+=1
 return s[m.start():i]
sets={"Block":{"field_41050_bK":"dragonEgg","func_40198_a":"quantityDroppedWithBonus","entityJoinedWorld":"spawnEntityInWorld","func_41049_c_":"createStackedBlock","func_41033_a":"setBlockNames"},"Chunk":{"field_35472_c":"worldHeight","field_35473_a":"heightShift","field_35471_b":"xShift","func_35839_k":"updateSkylight_do","func_35144_b":"updateContainingBlockInfo","func_35455_a":"markEntityForDespawn","func_40048_X":"getParts","getFalse":"isEmpty","func_25124_i":"removeUnknownBlocks","func_26002_a":"removeUnknownBlockIDs","func_35840_c":"getPrecipitationHeight","func_35841_j":"updateSkylight","func_40740_k":"getChunkCoordIntPair"}}
rows=[]
for cls,mapping in sets.items():
 a=(old/(cls+".java")).read_text();b=(new/(cls+".java")).read_text();x=a
 for k,v in mapping.items():x=re.sub(r"\b"+k+r"\b",v,x)
 regions=[]
 if cls=="Block":
  hook=method(b,"func_43001_a");assert tokens(hook[hook.index("{"):])==["{","}"]
  at=x.index("\tstatic {");x=x[:at]+hook+"\n\n"+x[at:];regions.append({"method":"func_43001_a","change":"new empty default entity-landing callback","entire_body_empty":True,"proof":"Original9216 landing11 instrumented virtualcallbacks delegate to realBlock implementation; defaultstone/fence outcomes and RNG preserved; actual farmland override verified separately."})
 else:
  substitutions=[("else if(Block.blocksList[var9] instanceof BlockContainer)","else if(Block.blocksList[var9] instanceof BlockContainer && var9 != var4)","setBlockIDWithMetadata","remote tile removal only when block ID changes"),("this.worldObj.setBlockTileEntity(var1, var2, var3, var12);","this.worldObj.setBlockTileEntity(var10, var2, var11, var12);","setBlockIDWithMetadata","fallback tile registration converts localx/z to worldx/z"),("this.worldObj.setBlockTileEntity(var1, var2, var3, var11);","this.worldObj.setBlockTileEntity(var9, var2, var10, var11);","setBlockID","fallback tile registration converts localx/z to worldx/z")]
  for before,after,name,reason in substitutions:
   assert x.count(before)==1,(cls,before);x=x.replace(before,after);assert after in method(b,name);regions.append({"method":name,"before":before,"after":after,"change":reason,"replacement_count":1})
 assert tokens(x)==tokens(b),"Unexplained complete-source residue in "+cls
 hunks=[];diff=list(difflib.unified_diff(a.splitlines(),b.splitlines(),n=3));current=None
 for line in diff[2:]:
  if line.startswith("@@"):
   current={"header":line,"lines":[],"kinds":[]};hunks.append(current)
  else:current["lines"].append(line)
 for h in hunks:
  body="\n".join(h["lines"])
  for region in regions:
   if region.get("after",region["method"]) in body:h["kinds"].append(region["change"])
  if any(re.search(r"\b"+k+r"\b",body) for k in mapping):h["kinds"].append("listed symbol renames; exact token residue proof")
 rows.append({"class_name":cls,"old_sha256":sha(old/(cls+".java")),"new_sha256":sha(new/(cls+".java")),"symbol_map":mapping,"excluded_behavior_regions":regions,"entire_source_token_residue_equal":True,"hunk_map":hunks,"whole_inherited_java_api_verified":False})
# The new Chunk calls refer to old exact-body methods rather than changed callbacks.
dependencies=[]
for cls,a_name,b_name in [("TileEntity","func_35144_b","updateContainingBlockInfo"),("ChunkBlockMap","func_26002_a","removeUnknownBlockIDs"),("World","func_35455_a","markEntityForDespawn")]:
 a=method((old/(cls+".java")).read_text(),a_name);b=method((new/(cls+".java")).read_text(),b_name);x=re.sub(r"\b"+a_name+r"\b",b_name,a)
 if cls=="ChunkBlockMap":x=re.sub(r"\bfield_26003_a\b","convTable",x)
 if cls=="TileEntity":x=re.sub(r"\bfield_35146_o\b","blockType",x)
 assert tokens(x)==tokens(b),(cls,a_name)
 dependencies.append({"class_name":cls,"old_method":a_name,"new_method":b_name,"body_tokens_equal":True,"old_method_sha256":hashlib.sha256(a.encode()).hexdigest(),"new_method_sha256":hashlib.sha256(b.encode()).hexdigest()})
Path(sys.argv[3]).write_text(json.dumps({"schema":1,"scope":"Read-only original-source proof, with exactly enumerated3 Chunk expressions and1 new empty Block callback. Parent owns Chunk production/oracle evidence; this proof does not replace it.","classes":rows,"renamed_callee_authority":dependencies},indent=2));print("Block and Chunk complete source residue EXACT; three renamed callees EXACT")
