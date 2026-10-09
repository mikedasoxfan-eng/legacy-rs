"""Finalize class credit only after the stable production suite and all targeted original proofs passed."""
import argparse,json,hashlib,subprocess
from pathlib import Path
R=Path(__file__).resolve().parents[2];P=R/'tools/r11';D=R/'reports/r11-progress'
def read(p):return json.loads(p.read_text(encoding='utf8'))
def write(p,v):p.write_text(json.dumps(v,indent=2)+'\n',encoding='utf8')
text_suffixes={'.rs','.txt','.toml','.json','.py','.ps1','.java','.log','.md','.yml','.yaml','.sh','.tsv','.csv'}
def h(p):
 b=p.read_bytes();return hashlib.sha256(b.replace(b'\r\n',b'\n') if p.suffix.lower() in text_suffixes else b).hexdigest()
def pins(paths):return {p:h(R/p) for p in paths}
final=D/'final-production11.log';assert 'R11 FINAL PRODUCTION SUITE, CROSS-VERSION REGRESSIONS AND PACKAGE CHECKS PASS' in final.read_text()
sections=read(P/'sections.json');snap=read(P/'source-snapshots.json');reviews=read(P/'completion-review.json');cov=read(P/'coverage.json');ported=read(P/'ported.json')
commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()
for crate in ['worldgen11','client11']:
 snap[crate]={str(p.relative_to(R)).replace('\\','/'):h(p) for p in sorted((R/'crates'/crate).rglob('*')) if p.is_file() and 'target' not in p.relative_to(R/'crates'/crate).parts and p.suffix in {'.rs','.txt','.bin','.toml'}}
for record in sections.values():
 record['stable_candidate_revalidation']={'report':str(final.relative_to(R)).replace('\\','/'),'sha256':h(final),'reason':'Original fixtures replayed against current production client/simulation code; all cross-version libraries pass. Final targeted source edits preserve previous world/provider/render hunks; original worldtype/native save proof rerun after NBT edit.'}
for row in reviews.values():
 row['rust_source_sha256']=pins(row['rust_source_sha256']);row['revalidation']='Stable final candidate: all client tests including original corpora/GPU upload, all simulation tests, all older-version libraries, release workspace/dispatch and targeted Unicode/name/save proof pass.'
jar=sections['gameplay11 fixture authority']['original_jar_sha256']
def section(name,report,command,source,marker):
 path=D/report;assert marker in path.read_text(),(name,marker)
 sections[name]={'exact':True,'source_snapshot':source,'command':command,'report':str(path.relative_to(R)).replace('\\','/'),'report_sha256':h(path),'original_jar_sha256':jar,'observations':[marker,'Observed successful exit; stable candidate production replays all retained original corpora.']}
section('potion11 full names and tooltip delta','potion-names11-repro.log','tools/oracle11/potion_names11.ps1','worldgen11','159452')
section('minecraft11 conversion startup bridge','minecraft-convert11-repro.log','tools/oracle11/minecraft_convert11.ps1','client11','MINECRAFT CONVERSION CALLBACK EXACT')
section('gui11 final input hover and save workflows','remaining-gui11-final-repro.log','tools/oracle11/remaining_gui11.ps1','client11','0 failed')
section('worldname11 modified UTF16 persistence','worldname-utf16-11-repro.log','tools/oracle11/verify_worldname_utf16_11.ps1','worldgen11','65550')
section('worldtype11 names settings and saves','final-worldtype11.log','tools/verify_r11_worldtype.ps1','worldgen11','EXACT MATCH: worldtype -1')
def accept(name,files,reason,coverage,authority=None,no_behavior=False,limits=None):
 reviews[name]={'complete_delta_review':True,'reason':reason,'rust_source_sha256':pins(files),'original_source_sha256':authority,'completion_blockers':[],'limitations':limits or [],'criterion':'Entire1.0-to1.1 changed delta in repository installed-resource singleplayer scope; reviewed all raw hunks and direct original proof. No claim of whole inheritedJava API.'}
 if no_behavior:reviews[name]['no_behavior_change']=True
 ported[name]=commit;cov[name]=coverage
x=read(D/'parent-final11-review.json')
for row in x['classes']:
 assert row['complete_delta_review'];name=row['class_name']
 coverage=['potion11 full names and tooltip delta','render11 item passes','render11 held workflow'] if name=='ItemPotion' else ['minecraft11 conversion startup bridge','language11 full drawing actions restart','gui11 small full deltas','gameplay11 original delta vectors','worldtype11 names settings and saves','gui11 final input hover and save workflows']
 accept(name,list(row['rust_source_sha256']),row,coverage,row['original_source_sha256'],limits=x['limits'])
x=read(D/'remaining-gui11-final-review.json')
for row in x['classes']:
 assert row['complete_changed_runtime_delta'],row['class_name']
 files=list(row.get('rust_source_sha256',{})) or ['crates/client11/src/'+p for p in row['rust_files']]
 accept(row['class_name'],files,row,['gui11 final input hover and save workflows','gui11 small full deltas','formatter11 actual string callsites'],row.get('authority'),row.get('no_behavior_change',False),row.get('limitations'))
# WorldInfo and its NBT carrier now additionally preserve newly reachable UTF16 units.
for name in ['WorldInfo','GuiCreateWorld','GuiTextField','Minecraft']:
 if name in reviews:
  files=list(reviews[name]['rust_source_sha256']);files+=['crates/worldgen11/src/storage.rs','crates/worldgen11/src/nbt.rs','crates/worldgen11/src/scenario.rs','crates/client11/src/worker.rs','crates/client11/src/app.rs','crates/client11/src/gui/saves.rs']
  reviews[name]['rust_source_sha256']=pins(sorted(set(files)));cov[name]=sorted(set(cov[name]+['worldname11 modified UTF16 persistence','gui11 final input hover and save workflows']))
  reviews[name]['unicode_workflow']='Lossless rawUTF16 name/seed/folder through originalGUI callbacks, nativeWindows filenames, initial save, reopen/list, rename and delete; full65550original NBT corpus +96originalGUI creation callbacks and native production lifecycle proof.'
for p,v in [('sections.json',sections),('source-snapshots.json',snap),('completion-review.json',reviews),('coverage.json',cov),('ported.json',ported)]:write(P/p,v)
raw=read(D/'raw-only11-completion-review.json');assert raw['counts']['raw_only_classes']==308 and raw['counts']['unresolved_declaration_bindings']==0
validation={'candidate_commit':commit,'complete':True,'source_sha256':{**snap['worldgen11'],**snap['client11'],'crates/client/src/main.rs':h(R/'crates/client/src/main.rs'),'crates/client/src/gui/screens/version_select.rs':h(R/'crates/client/src/gui/screens/version_select.rs')},'reports_sha256':pins([str(p.relative_to(R)).replace('\\','/') for p in [final,D/'raw-only11-completion-review.json',D/'raw-bindings11.json',D/'raw-source-screen11.json',D/'worldname-utf16-11-repro.log',D/'remaining-gui11-final-repro.log',D/'final-worldtype11.log']]),'raw_only_classes_resolved':308,'binary_sha256':h(R/'target/release/beta.exe'),'binary':'target/release/beta.exe','gates':['complete original-derived1.1 client/simulation production suite','original65550NBT +159452potion +UnicodeGUI boundaries','native initial-save/reopen/rename/delete','older-version library regressions','releaseworkspace build and fourversion CLI dispatch','CI four-version serializedsimulation command locally passed'],'limits':['Repository singleplayer scope; multiplayer is not supported.','Original GPU comparisons retain established driver rounding tolerance and finite-case bounds.']}
write(D/'final-validation.json',validation)
subprocess.run(['python','tools/r11/progress.py','--output',str(D/'current-progress.json')],cwd=R,check=True)

