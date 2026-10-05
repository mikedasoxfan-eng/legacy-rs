"""Strict mechanical translation of village block-placement bodies; rejects unknown syntax."""
from pathlib import Path
import re
src=Path('tools/retromcp-b181/minecraft/src/net/minecraft/src')
blocktext=(src/'Block.java').read_text()
ids={}
ids['cloth']='35'
for m in re.finditer(r'public static (?:final )?Block(?:\w*) (\w+) = \(?new \w+\((\d+)',blocktext):ids[m[1]]=m[2]
print('block names',len(ids))
classes=['House4_Garden','Church','House1','WoodHut','Hall','Field','Field2','House2','House3','Torch','Well','PathGen']
def args(s):
    out=[];depth=0;start=0
    for i,c in enumerate(s):
        if c in '([':depth+=1
        elif c in ')]':depth-=1
        elif c==',' and depth==0:out.append(s[start:i].strip());start=i+1
    out.append(s[start:].strip());return out
def calls(s):
    # Resolve innermost call expressions and statements first.
    pattern=r'(this\.func_\d+_[abc]|var[123]\.\w+|this\.field_35024_g\.func_35745_a|Math\.(?:min|max)|MathHelper\.func_35598_a)\('
    matches=list(re.finditer(pattern,s))
    if not matches:return s
    m=matches[-1];depth=1;i=m.end()
    while depth:
        if s[i]=='(':depth+=1
        elif s[i]==')':depth-=1
        i+=1
    a=args(s[m.end():i-1]);n=m[1]
    c='self.component'
    if n=='this.func_35011_a':v=f'{c}.fill({a[0]},{a[1]},[{",".join(a[2:8])}],{",".join(a[8:])})'
    elif n=='this.func_35018_a':v=f'{c}.put({a[0]},{a[6]},{",".join(a[1:6])})'
    elif n=='this.func_35007_a':v=f'{c}.get({a[0]},{a[4]},{",".join(a[1:4])})'
    elif n=='this.func_35016_b':v=f'{c}.clear_up({a[0]},{a[4]},{",".join(a[1:4])})'
    elif n=='this.func_35005_b':v=f'{c}.fill_down({a[0]},{a[6]},{",".join(a[1:6])})'
    elif n=='this.func_35002_a':v=f'{c}.door({a[0]},{a[1]},{",".join(a[3:])})'
    elif n=='this.func_35009_c':v=f'{c}.rotate({",".join(a)})'
    elif n=='this.func_35075_b':v=f'average_height({a[0]},{a[1]},{c}.bounds)'
    elif n=='this.field_35024_g.func_35745_a':v=f'{c}.bounds.offset({",".join(a)})'
    elif n in ['Math.min','Math.max']:v=f'{n[5:]}({",".join(a)})'
    elif n=='MathHelper.func_35598_a':v=f'inclusive({",".join(a)})'
    elif n=='var2.nextInt':v=f'r.next_int({",".join(a)})'
    elif n=='var1.getTopSolidOrLiquidBlock':v=f'top_solid({"w,"+",".join(a)})'
    elif n=='var1.setBlock':v=f'w.set_block({",".join(a[:-1])},{a[-1]} as u8)'
    elif n=='var3.func_35742_b':v=f'clip.contains({",".join(a)})'
    else:raise ValueError(n)
    # Protect Rust calls from subsequent Java matcher by variable substitution now.
    v=re.sub(r'\bvar1\b','w',v);v=re.sub(r'\bvar2\b','r',v);v=re.sub(r'\bvar3\b','clip',v)
    return calls(s[:m.start()]+v+s[i:])
def expr(s,fields):
    s=calls(s)
    for f,v in fields.items():s=s.replace('this.'+f,'self.'+v)
    for f,v in zip(['field_35753_a','field_35751_b','field_35752_c','field_35749_d','field_35750_e','field_35748_f'],['min_x','min_y','min_z','max_x','max_y','max_z']):s=s.replace('this.field_35024_g.'+f,'self.component.bounds.'+v)
    s=s.replace('this.field_35025_h','self.component.orientation')
    s=re.sub(r'Block\.(\w+)\.blockID',lambda m:ids[m[1]],s)
    s=re.sub(r'\bvar1\b','w',s);s=re.sub(r'\bvar2\b','r',s);s=re.sub(r'\bvar3\b','clip',s)
    if 'this.' in s or 'func_' in s or 'Block.' in s:raise ValueError(s)
    return s
out=['//! Literal block-placement bodies from the Beta 1.8.1 village classes.','#![allow(unused_variables, unused_mut)]','use super::*;','impl VillagePiece {']
for kind,cl in enumerate(classes):
    text=(src/f'ComponentVillage{cl}.java').read_text();start=text.index('public boolean func_35023_a(');body=text[text.index('{',start)+1:];body=body[:body.rfind('}')].strip();body=body[:body.rfind('}')]
    fields={m[1]:'height' for m in re.finditer(r'private int (\w+) = -1;',text)}
    for m in re.finditer(r'private final boolean (\w+)',text):fields[m[1]]='flag'
    for m in re.finditer(r'private final int (\w+)',text):fields[m[1]]='style'
    if cl=='PathGen':fields['field_35088_a']='length'
    out.append(f'pub(super) fn generate_{kind}(&mut self,w:&mut World,r:&mut JavaRandom,clip:B)->bool {{')
    for raw in body.splitlines():
        line=raw.strip()
        if not line:continue
        if line=='w.getClass();' or line=='var1.getClass();':continue
        m=re.fullmatch(r'for\((?:int )?(\w+) = (.*); \1 (<|<=) (.*); \+\+\1\) \{',line)
        if m:line=f'for {m[1]} in ({expr(m[2],fields)})..{"=" if m[3]=="<=" else ""}({expr(m[4],fields)}) {{'
        elif line.startswith('for('):
            m=re.fullmatch(r'for\((?:int )?(\w+) = (.*); \1 >= (.*); --\1\) \{',line)
            if not m:raise ValueError(line)
            line=f'for {m[1]} in (({expr(m[3],fields)})..=({expr(m[2],fields)})).rev() {{'
        elif re.fullmatch(r'int (\w+);',line):line=re.sub(r'int (\w+);',r'let mut \1:i32;',line)
        elif line.startswith('int '):line=re.sub(r'int (\w+) = (.*);',lambda m:f'let mut {m[1]}:i32 = {expr(m[2],fields)};',line)
        elif line.startswith('++'):line=line[2:-1]+' += 1;'
        else:line=expr(line,fields)
        if line.startswith('if('):line='if '+line[3:-3]+' {'
        elif line.startswith('} else if('):line='} else if '+line[10:-3]+' {'
        if 'inclusive(r,' in line:
            start=line.index('inclusive(r,');end=line.index(')',start)+1
            call=line[start:end].replace('inclusive(r,','inclusive(crate::structures::component::random(w,r),')
            out.append('let crop_metadata = '+call+';')
            line=line[:start]+'crop_metadata'+line[end:]
        out.append(line)
    out.append('}')
out.append('}')
Path('crates/worldgen181/src/structures/village_generated.rs').write_text('\n'.join(out)+'\n')
