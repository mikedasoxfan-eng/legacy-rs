"""Audit full source residue after explicit tested 1.1 behavior hunks.
Usage: python verify_entity_player_item_delta11.py OLD_MCP NEW_MCP OUTPUT_JSON
"""
import difflib,hashlib,json,re,sys
from pathlib import Path
old=Path(sys.argv[1])/"minecraft/src/net/minecraft/src";new=Path(sys.argv[2])/"minecraft/src/net/minecraft/src"
def tokens(s):return re.findall(r"[A-Za-z_$][\w$]*|\d+(?:\.\d+)?[FDfdLl]?|[^\s]",s)
def sha(b):return hashlib.sha256(b).hexdigest()
def method(s,name):
 m=re.search(r"^[ \t]*(?:public|protected|private)[^\n]*\b"+re.escape(name)+r"\([^;{]*\)\s*\{",s,re.M);assert m,name
 i=m.end();depth=1
 while depth:depth+=(s[i]=="{")-(s[i]=="}");i+=1
 return s[m.start():i]
base={"beenAttacked":"velocityChanged","func_40046_d":"setFire","func_40045_B":"extinguish","func_40047_D":"isImmuneToFire","field_35472_c":"worldHeight","func_41001_Z":"getAir","func_41003_g":"setAir","entityJoinedWorld":"spawnEntityInWorld","getEntityFlag":"getFlag","func_41062_b":"getWatchableObjectShort","func_40048_X":"getParts","func_41004_h":"isEntityEqual","func_35162_ad":"isBlocking","potionDigSpeed":"digSpeed","potionDigSlow":"digSlowdown","func_40116_f":"decreaseAirSupply","func_40600_b":"readCapabilitiesFromNBT","func_40601_a":"writeCapabilitiesToNBT","func_40181_c":"displayGUIEnchantment","func_40128_b":"applyPotionDamageCalculations","field_40129_bA":"carryoverDamage","isWolfTamed":"isTamed","getWolfOwner":"getOwner","isWolfSitting":"isSitting","func_40119_ar":"getTotalArmorValue","unblockable":"isUnblockable","func_40115_d":"applyArmorCalculations","func_40180_a":"displayGUIBrewingStand","potionDamageBoost":"damageBoost","potionWeakness":"weakness","potionBlindness":"blindness","isNether":"isAlternateDimension","func_40184_i":"decreaseLevel","func_35197_b":"canEat","func_35190_e":"canPlayerEdit","func_36001_a":"getExperiencePoints","func_35163_av":"isPlayer","func_41014_d":"copyPlayer","func_41022_a":"copyInventory","potionConfusion":"confusion","func_35166_t_":"getSpeedModifier","func_40407_b":"setPotionModifier","func_35424_o":"setAlwaysEdible","potionRegeneration":"regeneration","potionHunger":"hunger","potionPoison":"poison","field_39010_c":"potionModifier","getPlacedBlockMetadata":"getMetadata","func_40400_h":"getLocalItemName","func_40405_m":"getPotionModifier","func_40406_n":"isValidBrewingIngredient","func_40404_a":"addInformation","func_40397_d":"getItemDisplayName","func_40403_e":"hasEffect","func_40711_u":"isItemEnchanted","func_40398_f":"getRarity","func_40401_i":"isItemTool","field_40373_k":"gunpowderEffect","field_40375_i":"redstoneEffect","field_40372_j":"glowstoneEffect","field_40365_b":"sugarEffect","field_40366_c":"ghastTearEffect","field_40363_d":"spiderEyeEffect","field_40364_e":"fermentedSpiderEyeEffect","field_40362_g":"blazePowderEffect","field_40374_h":"magmaCreamEffect","field_40361_f":"speckledMelonEffect"}
covered={"Entity":["updateFallState","setInWeb"],"EntityPlayer":["damageEntity","increaseXP","getItemIcon"],"EntityPlayerSP":[],"Item":["setContainerItem"]}
rows=[]
for cls,changed in covered.items():
 a=(old/(cls+".java")).read_text();b=(new/(cls+".java")).read_text();x=a
 mapping={k:v for k,v in base.items() if re.search(r"\b"+k+r"\b",a)}
 for k,v in mapping.items():x=re.sub(r"\b"+k+r"\b",v,x)
 edits=[]
 if cls=="EntityPlayer":x=x.replace("protected int getTotalArmorValue","public int getTotalArmorValue");edits.append("getTotalArmorValue access widening")
 if cls=="EntityPlayerSP":
  x=x.replace(method(x,"getPlayerArmorValue"),"");edits.append("removed redundant inventory-armor getter; callers use inherited same-body getTotalArmorValue")
  at=x.index("\tpublic void onLivingUpdate()");x=x[:at]+method(b,"func_44001_ad")+"\n\n"+x[at:];edits.append("new func_44001_ad unconditional true; original living_guard11 and actual player dispatcher tested")
 if cls=="Item":
  x=x.replace("new ItemAppleGold(66, 10, 1.2F, false)","new ItemAppleGold(66, 4, 1.2F, false)").replace("setPotionEffect(Potion.regeneration.id, 30, 0, 1.0F)","setPotionEffect(Potion.regeneration.id, 5, 0, 1.0F)");edits.append("golden apple heal4 and regeneration5seconds; actual96 item-start/finish state+RNG cases")
  line=next(line for line in b.splitlines(True) if "public static Item field_44019_bC" in line);at=x.index("\tpublic static Item record13");x=x[:at]+line+x[at:];edits.append("spawn egg registry383; existing egg metadata/creative/spawn/NBT960case authority")
  x=x.replace("getColorFromDamage(int var1)","getColorFromDamage(int var1, int var2)");edits.append("unused default color-pass parameter")
  at=x.index("\tpublic Item getContainerItem");x=x[:at]+method(b,"func_46059_i")+"\n\n"+method(b,"func_46056_k")+"\n\n"+x[at:];edits.append("new crafting remainder defaulttrue and packet extended-tag defaultfalse")
  at=x.index("\tstatic {");x=x[:at]+method(b,"func_46058_c")+"\n\n"+method(b,"func_46057_a")+"\n\n"+x[at:];edits.append("new default multipass=false and per-pass icon delegates old icon")
 changedproof=[]
 for name in changed:
  before=method(x,name);after=method(b,name);x=x.replace(before,after);changedproof.append({"method":name,"new_method_sha256":sha(after.encode()),"status":"explicit changed behavior hunk; direct original tests referenced in completion review"})
 assert tokens(x)==tokens(b),"Unexplained source residue: "+cls
 rows.append({"class_name":cls,"old_sha256":sha((old/(cls+".java")).read_bytes()),"new_sha256":sha((new/(cls+".java")).read_bytes()),"entire_remaining_source_tokens_equal":True,"symbol_map":mapping,"structural_changes":edits,"explicit_behavior_hunks":changedproof,"all_hunks":list(difflib.unified_diff(a.splitlines(),b.splitlines(),n=3))})
Path(sys.argv[3]).write_text(json.dumps({"schema":1,"scope":"Source-residue proof after explicitly enumerated behavior changes; source comparison does not itself assert those behaviors tested or absent facade implemented.","classes":rows},indent=2));print("Full source residue EXACT:",len(rows))
