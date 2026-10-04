import re,pathlib
root=pathlib.Path(__file__).resolve().parent.parent
source=(root/'tools/retromcp/minecraft/src/net/minecraft/src/RenderBlocks.java').read_text()
types={'int':'i32','float':'f32','double':'f64','boolean':'bool','Block':'i32','BlockRail':'i32'}
types.update({'byte':'i32','BlockDoor':'i32'})
rank={'bool':-1,'i32':0,'f32':1,'f64':2}
names={'renderBlockTorch':'damage_torch','renderTorchAtAngle':'damage_torch_angle','renderBlockLadder':'damage_ladder','renderBlockMinecartTrack':'damage_rail','renderBlockCrops':'damage_crops','func_1245_b':'damage_crop_planes','renderBlockFence':'damage_fence','renderBlockStairs':'damage_stairs'}
names.update({j:'damage_'+r+'_face' for j,r in [('renderBottomFace','bottom'),('renderTopFace','top'),('renderEastFace','east'),('renderWestFace','west'),('renderNorthFace','north'),('renderSouthFace','south')]})
names.update({'func_31074_b':'damage_piston','func_31080_c':'damage_extension','func_31076_a':'damage_stem_a','func_31081_b':'damage_stem_b','func_31077_c':'damage_stem_c'})
names.update({'renderBlockBed':'damage_bed','renderBlockRepeater':'damage_repeater','renderBlockDoor':'damage_door'})
names.update({'renderBlockRedstoneWire':'damage_wire','renderBlockFire':'damage_fire'})
fields={'this.field_31087_g':('self.damage_uv[0]','i32'),'this.field_31086_h':('self.damage_uv[1]','i32'),'this.field_31085_i':('self.damage_uv[2]','i32'),'this.field_31084_j':('self.damage_uv[3]','i32'),'this.field_31083_k':('self.damage_uv[4]','i32'),'this.field_31082_l':('self.damage_uv[5]','i32'),'this.flipTexture':('self.damage_flip','bool'),'this.enableAO':('false','bool'),'this.renderAllFaces':('false','bool')}
for axis in ['X','Y','Z']:
 for end in ['min','max']:
  fields['var1.'+end+axis]=('self.b.'+end+'_'+axis.lower(),'f64');fields['var6.'+end+axis]=('self.b.'+end+'_'+axis.lower(),'f64')
prec={'||':1,'&&':2,'|':3,'^':4,'&':5,'==':6,'!=':6,'<':7,'>':7,'<=':7,'>=':7,'<<':8,'>>':8,'+':9,'-':9,'*':10,'/':10,'%':10}
def cast(v,t):return v if v[1]==t else ('('+v[0]+' as '+t+')',t)
class Parser:
 def __init__(self,s):self.ts=re.findall(r'\d+\.\d+(?:[FD])?|\d+(?:[FD])?|[A-Za-z_][\w.]*|\+\+|--|==|!=|<=|>=|&&|\|\||<<|>>|\+=|-=|\*=|&=|[^\s]',s);self.i=0;self.vars={}
 def take(self,t=None):v=self.ts[self.i];self.i+=1;assert t is None or t==v,(t,v,self.ts[max(0,self.i-8):self.i+8]);return v
 def peek(self):return self.ts[self.i] if self.i<len(self.ts) else ''
 def expr(self,p=0):
  q=self.take()
  if q=='(':
   if self.peek() in types and self.ts[self.i+1]==')':typ=types[self.take()];self.take(')');v=cast(self.expr(11),typ)
   else:v=self.expr();self.take(')')
  elif q in ('-','!','+'):
   v=self.expr(11);v=('('+q+v[0]+')',v[1])
  elif re.match(r'^\d',q):
   typ='f32' if q.endswith('F') else 'f64' if q.endswith('D') or '.' in q else 'i32';v=(q.rstrip('FD')+('f32' if typ=='f32' else 'f64' if typ=='f64' else ''),typ)
  elif q in ('true','false'):v=(q,'bool')
  elif self.peek()=='(':
   self.take();args=[]
   if self.peek()!=')':
    while True:
     args.append(self.expr())
     if self.peek()!=',':break
     self.take()
   self.take(')');v=self.call(q,args)
  elif q in fields:v=fields[q]
  elif q.startswith('this.color'):v=('0.0f32','f32')
  elif q in ('ModelBed.field_22280_a','ModelBed.field_22279_b','BlockRedstoneRepeater.field_22024_a'):
   self.take('[');a=self.expr();self.take(']');arr={'ModelBed.field_22280_a':'[3i32,4,2,5]','ModelBed.field_22279_b':'[2i32,3,0,1]','BlockRedstoneRepeater.field_22024_a':'[-0.0625f64,0.0625,0.1875,0.3125]'}[q];v=(arr+'['+a[0]+' as usize]','f64' if q.startswith('BlockRedstone') else 'i32')
  elif q=='Block.lightValue':
   self.take('[');a=self.expr();self.take(']');v=('(worldgen::block::light_value('+a[0]+' as u8) as i32)','i32')
  elif q=='this.overrideBlockTexture':v=('self.override_texture.unwrap()','i32')
  elif q=='this.blockAccess':v=('0','i32')
  elif q=='var1.blockID':v=('var1','i32')
  elif q=='Block.redstoneWire.blockID':v=('55','i32')
  else:v=(q,self.vars[q])
  while self.peek() in prec and prec[self.peek()]>=p:
   op=self.take();rhs=self.expr(prec[op]+1);typ=max((v[1],rhs[1]),key=lambda t:rank[t]);v=cast(v,typ);rhs=cast(rhs,typ);v=('('+v[0]+' '+op+' '+rhs[0]+')','bool' if op in ('==','!=','<','>','<=','>=','&&','||') else typ)
  if p==0 and self.peek()=='?':
   self.take();a=self.expr();self.take(':');b=self.expr();typ=max((a[1],b[1]),key=lambda t:rank[t]);v=('(if '+v[0]+' { '+cast(a,typ)[0]+' } else { '+cast(b,typ)[0]+' })',typ)
  return v
 def call(self,q,a):
  if q in ('BlockPistonBase.func_31044_d','BlockPistonExtension.func_31050_c'):return ('('+a[0][0]+' & 7)','i32')
  if q in ('var1.getBlockBrightness',):return ('1.0f32','f32')
  if q=='BlockBed.getDirectionFromMetadata':return ('('+a[0][0]+' & 3)','i32')
  if q=='BlockBed.isBlockFootOfBed':return ('(('+a[0][0]+' & 8) != 0)','bool')
  if q=='var1.shouldSideBeRendered':return ('self.should_side_be_rendered(var1 as u8,'+','.join(x[0] for x in a[1:])+')','bool')
  if q=='var1.getBlockTexture':return ('self.damage_native_texture(var1 as u8,'+','.join(x[0] for x in a[1:])+')','i32')
  if q in ('var1.getBlockTextureFromSide','var1.getBlockTextureFromSideAndMetadata'):return ('(rprops(var1 as u8).textures['+(a[1][0]+' as usize & 15' if len(a)>1 else '0')+']['+a[0][0]+' as usize] as i32)','i32')
  if q=='var1.getIsPowered':return ('matches!(var1,27|28)','bool')
  if q=='this.blockAccess.getBlockMetadata':return ('self.w.meta('+','.join(x[0] for x in a)+')','i32')
  if q=='this.blockAccess.getBlockId':return ('(self.w.block('+','.join(x[0] for x in a)+') as i32)','i32')
  if q=='this.blockAccess.isBlockNormalCube':return ('self.damage_normal_cube('+','.join(x[0] for x in a)+')','bool')
  if q=='BlockRedstoneWire.isPowerProviderOrWire':return ('self.damage_wire_connect('+','.join(x[0] for x in a[1:])+')','bool')
  if q=='Block.fire.canBlockCatchFire':return ('matches!(self.w.block('+','.join(x[0] for x in a[1:])+'),5|85|53|17|18|47|46|31|35)','bool')
  if q.endswith('.setColorOpaque_F'):return ('()','void')
  if q.endswith('.addVertexWithUV'):return ('self.t.vertex_uv('+','.join(cast(x,'f64')[0] for x in a)+')','void')
  if q=='var1.setBlockBounds':return ('self.damage_bounds(['+','.join(cast(x,'f32')[0] for x in a)+'])','void')
  if q=='this.renderStandardBlock':return ('self.render_standard_block(var1 as u8,'+','.join(x[0] for x in a[1:])+')','bool')
  if q.startswith('this.') and q[5:] in names:return ('self.'+names[q[5:]]+'('+','.join(x[0] for x in a)+')','void')
  raise ValueError(q)
 def stmt(self):
  q=self.peek()
  if q=='{':
   self.take();s='{\n'
   while self.peek()!='}':s+=self.stmt()+'\n'
   self.take();return s+'}'
  if q=='if':
   self.take();self.take('(');v=self.expr();self.take(')');s='if '+v[0]+' '+self.stmt()
   if self.peek()=='else':self.take();s+=' else '+self.stmt()
   return s
  if q=='switch':
   self.take();self.take('(');v=self.expr();self.take(')');self.take('{');cases=[];labels=[];body=''
   while self.peek()!='}':
    if self.peek() in ('case','default'):
     if body:cases.append((labels,body));labels=[];body=''
     q=self.take();labels.append(self.take() if q=='case' else '_');self.take(':')
    elif self.peek()=='break':
     self.take();self.take(';');cases.append((labels,body));labels=[];body=''
    else:body+=self.stmt()+'\n'
   if labels:cases.append((labels,body))
   self.take();normal=[([x for x in ls if x!='_'],b) for ls,b in cases];default=next((b for ls,b in cases if '_' in ls),'');return 'match '+v[0]+' {\n'+''.join('|'.join(ls)+' => {\n'+b+'},\n' for ls,b in normal if ls)+'_ => { '+default+' },\n}'
  if q=='return':self.take();v=self.expr();self.take(';');return 'return '+v[0]+';'
  if q=='Tessellator':
   while self.take()!=';':pass
   return ''
  if q in types:
   typ=types[self.take()];name=self.take();self.vars[name]=typ
   if self.peek()=='=':self.take();v=cast(self.expr(),typ)[0]
   else:v={'i32':'0','f32':'0.0','f64':'0.0','bool':'false'}[typ]
   self.take(';');return 'let mut '+name+': '+typ+' = '+v+';'
  if q=='++':self.take();name=self.take();self.take(';');return name+' += '+('1.0' if self.vars[name] in ('f32','f64') else '1')+';'
  if self.ts[self.i+1] in ('=','+=','-=','*=','&='):
   name=self.take();op=self.take();lhs=fields.get(name,(name,self.vars.get(name)));v=cast(self.expr(),lhs[1]);self.take(';');return lhs[0]+' '+op+' '+v[0]+';'
  v=self.expr();self.take(';');return v[0]+';'
out=['// Literal destruction-only RenderBlocks geometry. The oracle checks raw bits.','#[allow(non_snake_case,unused_mut,unused_variables,unused_parens,unused_assignments)]',"impl<'a,'b> RenderBlocks<'a,'b> {",'fn damage_bounds(&mut self,b:[f32;6]) { self.b=Bounds{min_x:b[0] as f64,min_y:b[1] as f64,min_z:b[2] as f64,max_x:b[3] as f64,max_y:b[4] as f64,max_z:b[5] as f64}; }']
out.append('fn damage_native_texture(&self,id:u8,x:i32,y:i32,z:i32,side:i32)->i32 { let m=self.w.meta(x,y,z);if matches!(id,64|71) { let base=if id==64 {97} else {98};if side<2{return base;}let state=if m&4==0{(m-1)&3}else{m&3};if ((state==0 || state==2) ^ (side<=3)){return base;}let flip=state/2+((side&1)^state)+(m&4)/4;let tex=base-(m&8)*2;return if flip&1!=0{-tex}else{tex}; }rprops(id).textures[m as usize][side as usize] as i32 }')
out.append('fn damage_wire_connect(&self,x:i32,y:i32,z:i32,side:i32)->bool { let id=self.w.block(x,y,z);if id==55 || worldgen::entity::block_hooks::provides_power(id){return true;}if matches!(id,93|94){return side==[2,3,0,1][(self.w.meta(x,y,z)&3) as usize];}false }')
for name,rust in names.items():
 m=re.search(r'(?:public|private) (boolean|void) '+name+r'\(([^)]*)\)\s*\{',source);a=m.end()-1;end=a;depth=0
 while True:
  if source[end]=='{':depth+=1
  if source[end]=='}':
   depth-=1
   if depth==0:break
  end+=1
 p=Parser(source[a:end+1]);args=[]
 for param in m[2].split(','):
  typ,n=param.strip().split();typ=types[typ];p.vars[n]=typ;args.append('mut '+n+': '+typ)
 out.append('fn '+rust+'(&mut self,'+','.join(args)+')'+(' -> bool' if m[1]=='boolean' else '')+' '+p.stmt())
out.append('}')
(root/'crates/client/src/render/blocks_damage.rs').write_text('\n'.join(out)+'\n')






