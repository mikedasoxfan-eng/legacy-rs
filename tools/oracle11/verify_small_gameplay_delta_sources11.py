"""Full-class source delta normalization for bounded gameplay classes.
Usage: python verify_small_gameplay_delta_sources11.py OLD_MCP NEW_MCP OUTPUT_JSON
This is source proof, never blanket inherited Java API verification.
"""
import difflib,hashlib,json,re,sys
from pathlib import Path
old=Path(sys.argv[1])/"minecraft/src/net/minecraft/src";new=Path(sys.argv[2])/"minecraft/src/net/minecraft/src"
sets={"EntityCaveSpider":{"potionPoison":"poison"},"EntityCreature":{"func_35166_t_":"getSpeedModifier"},"EntityGiantZombie":{},"EntitySilverfish":{"func_40124_t":"getCreatureAttribute"},"EntitySlime":{"field_35171_bJ":"experienceValue","entityJoinedWorld":"spawnEntityInWorld"},"StepSoundSand":{},"StepSoundStone":{},"EntityDamageSource":{},"EntitySkeleton":{"func_40046_d":"setFire","entityJoinedWorld":"spawnEntityInWorld","func_40124_t":"getCreatureAttribute"},"ItemColored":{"field_35436_a":"blockRef","field_41034_b":"blockNames","getPlacedBlockMetadata":"getMetadata","func_41033_a":"setBlockNames"},"ItemLeaves":{"getPlacedBlockMetadata":"getMetadata"},"ItemLilyPad":{"func_35190_e":"canPlayerEdit"}}
def tokens(s):return re.findall(r"[A-Za-z_$][\w$]*|\d+(?:\.\d+)?[FDfdLl]?|[^\s]",s)
def sha(b):return hashlib.sha256(b).hexdigest()
rows=[]
for cls,m in sets.items():
 a=(old/(cls+".java")).read_text();b=(new/(cls+".java")).read_text();x=a
 for k,v in m.items():x=re.sub(r"\b"+k+r"\b",v,x)
 transforms=[]
 if cls.startswith("StepSound"):x=x.replace("class "+cls,"final class "+cls);transforms.append("class becomes final; same methods/constructors")
 elif cls=="EntityDamageSource":x=x.replace("private Entity damageSourceEntity","protected Entity damageSourceEntity");transforms.append("field access widening")
 else:
  for olddecl,newdecl in [("protected boolean attackEntityAsMob","public boolean attackEntityAsMob"),("protected float getBlockPathWeight","public float getBlockPathWeight"),("protected int getVerticalFaceSpeed","public int getVerticalFaceSpeed")]:
   if olddecl in x:x=x.replace(olddecl,newdecl);transforms.append(olddecl+" -> "+newdecl)
 if cls in ["ItemColored","ItemLeaves","ItemLilyPad"]:
  x=x.replace("getColorFromDamage(int var1)","getColorFromDamage(int var1, int var2)");transforms.append("unused color pass parameter added; body identical and never reads var2")
 assert tokens(x)==tokens(b),cls
 rows.append({"class_name":cls,"disposition":"entire_delta_declaration_or_symbol_only","whole_class_behavioral_oracle_claim":False,"old_sha256":sha((old/(cls+".java")).read_bytes()),"new_sha256":sha((new/(cls+".java")).read_bytes()),"entire_source_tokens_equal_after_listed_transformations":True,"symbol_map":m,"declaration_transforms":transforms,"all_hunks":list(difflib.unified_diff(a.splitlines(),b.splitlines(),n=3))})
Path(sys.argv[3]).write_text(json.dumps({"schema":1,"scope":"Full original-source deltas only. No absent Java facade APIs claimed implemented; inherited methods need no new behavioral reproof.","classes":rows},indent=2))
print("Entire-class token normalization EXACT:",len(rows))
