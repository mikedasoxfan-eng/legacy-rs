"""Retain every original source hunk and the reviewed runtime/evidence mapping for the final parent-owned deltas."""
import json,hashlib,difflib
from pathlib import Path
R=Path(__file__).resolve().parents[2]
def digest(path):return hashlib.sha256(path.read_bytes().replace(b'\r\n',b'\n')).hexdigest()
classes=[]
for name,folder,files,groups in [
 ('ItemPotion','src',['crates/worldgen11/src/item10.rs','crates/worldgen11/src/potion_helper.rs','crates/client11/src/gui/screens/tooltip.rs','crates/client11/src/render/entity/item_table.rs'],[
  {'delta':'idEffectNameMap/effect helpers/isSplash/isEffectInstant/addInformation/hasEffect, World spawn and Potion method renames','disposition':'Symbol-only hunks; unchanged cached effects, gameplay consumption, splash throw and tooltip behavior. Existing original gameplay vectors retained.'},
  {'delta':'generic multiple-pass icon, pass-dependent color and boolean multiple-pass predicate','disposition':'Actual production item_table and held/dropped/player renderers;185primitive+752drop/inventory+352held/player+164biped original render cases.'},
  {'delta':'effect display names omit base item name; no-effect splash names omit grenade prefix; superclass label is Java-trimmed','disposition':'item10::PotionItem::display_name corrected; actual GUI tooltip calls it.159452 original display-name AND information-list comparisons across65536metadata values and all installed locale switches.'}]),
 ('Minecraft','client',['crates/client11/src/app.rs','crates/client11/src/gui/system.rs','crates/client11/src/gui/screens/container.rs','crates/client11/src/gui/lang.rs','crates/client11/src/gui/loading.rs','crates/client11/src/minecraft_input.rs','crates/client11/src/worker.rs'],[
  {'delta':'imports; profiler method/field renames; controller edit/right-click methods; colorizer setter, portal/player/spawn/save-format methods','disposition':'Reviewed all full source hunks. Signatures, operands, control flow and target semantics remain equivalent; raw binding authority independently retains owner/name/descriptors.'},
  {'delta':'Display title release1.1','disposition':'All three actual app window creation sites use Minecraft1.1; original changed operand Minecraft Minecraft1.1 retains inherited duplicate prefix outside this delta scope.'},
  {'delta':'primary and galactic FontRenderer false constructor flag; saved-language load, primary unicode/bidi flags','disposition':'FontRenderer default constructor flags false; actual GuiSystem::new loads installed language and sets primary unicode/bidi. Enchantment galactic constructor leaves false flags. Existing131072width/131072draw font corpus,56asset language maps,126frames and18save/reopen workflows; startup source bridge reviewed.'},
  {'delta':'six localized lifecycle labels','disposition':'Actual app worker-message handler calls localized_message;342original label translations across57locale switches pass.'},
  {'delta':'convertMapFormat adds explicit EnumWorldType.DEFAULT','disposition':'Original private callback invoked through virtual save/world-open boundaries; captured0,0,true,false,DEFAULT. Worker actual conversion branch uses converted_world_settings; focused original-derived fixture test passes.'},
  {'delta':'middle click guards null Item entry before subtype lookup','disposition':'Actual Minecraft input callback inventory binding;30720original callback cases for all256blockIDs,metadata/entity/air,creative/survival/hotbar.'}])]:
 old=R.parent/'retromcp-1.0/minecraft/src_original/net/minecraft'/folder/(name+'.java')
 new=R/'tools/retromcp-r11/minecraft/src_original/net/minecraft'/folder/(name+'.java')
 a=old.read_text().splitlines();b=new.read_text().splitlines();hunks=[]
 for tag,i,j,k,l in difflib.SequenceMatcher(None,a,b,autojunk=False).get_opcodes():
  if tag!='equal':hunks.append({'old_start':i+1,'new_start':k+1,'old':a[i:j],'new':b[k:l]})
 classes.append({'class_name':name,'complete_delta_review':True,'domain':'installed-resource singleplayer1.0-to1.1 delta','original_source_sha256':{'1.0':digest(old),'1.1':digest(new)},'all_raw_hunks':hunks,'reviewed_delta_groups':groups,'rust_source_sha256':{f:digest(R/f) for f in files}})
report={'classes':classes,'limits':['Inherited Java APIs are not reimplemented solely to earn delta credit. Multiplayer networking is excluded by repository scope.','Original private conversion callback uses external boundary intercepts; it does not generate/open a full world during this focused probe.','Finite original drawing corpora do not prove every possible GPU frame.']}
(R/'reports/r11-progress/parent-final11-review.json').write_text(json.dumps(report,indent=2)+'\n')
print('PARENT FINAL SOURCE REVIEW: ItemPotion and Minecraft all raw hunks retained and reviewed')
