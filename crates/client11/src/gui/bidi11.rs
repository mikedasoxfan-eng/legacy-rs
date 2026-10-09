//! Java 8u51-compatible pre-isolate UTF-16 ordering for the 1.1 font.
//! Character data is Unicode 6.1, verified against the pinned runtime.
fn removed(c:u8)->bool {matches!(c,11|12|14|15|16|18)}
fn dir(l:u8)->u8{l&1}
fn legacy(classes:&[u8],bases:&[u8])->(Vec<u8>,Option<u8>){
 let n=classes.len();if n==0{return (vec![],Some(0));}let mut levels=vec![0;n];let mut current=bases[0];let mut assigned=current;let mut stack=Vec::new();let mut flags=0u32;let(mut over60,mut over61)=(0,0);
 for i in 0..n {let c=classes[i];match c{
  11|12|14|15=>{let raw=current&127;let next=if c==11||c==12{(raw+2)&!1}else{(raw+1)|1};if next<=61{stack.push(current);current=next|if c==12||c==15{128}else{0};}else if raw==61{over61+=1;}else{over60+=1;}}
  16=>{if over61>0{over61-=1;}else if over60>0&&current&127!=61{over60-=1;}else if let Some(prev)=stack.pop(){current=prev;}}
  7=>{stack.clear();over60=0;over61=0;assigned=bases[i];if i+1<n{current=bases[i+1];}}
  18=>{}
  _=>{assigned=current;}
 }
 if removed(c){flags|=1<<18;}else if c==7{flags|=1<<7;}else {
  if assigned!=if i==0{bases[0]}else{levels[i-1]} {flags|=1<<if assigned&128!=0{if assigned&1==0{12}else{15}}else{if assigned&1==0{11}else{14}};}
  if assigned&128==0{flags|=1<<c;}
 }
 levels[i]=assigned;}
 let possible=(1<<3)|(1<<4)|(1<<6)|(1<<7)|(1<<8)|(1<<9)|(1<<10)|(1<<11)|(1<<12)|(1<<14)|(1<<15)|(1<<16)|(1<<18);
 if flags&(possible|(1<<17))!=0{flags|=1<<bases[0];}
 let rtl=flags&((1<<1)|(1<<13)|(1<<14)|(1<<15))!=0 || flags&(1<<5)!=0&&flags&possible!=0;
 let ltr=flags&((1<<0)|(1<<2)|(1<<5)|(1<<11)|(1<<12))!=0;
 if !rtl||!ltr{return (bases.to_vec(),Some(if rtl{bases[0]|1}else{(bases[0]+1)&!1}));}
 let explicit=levels.clone();let mut start=0;
 while start<n{let lev=explicit[start];let mut end=start+1;while end<n&&explicit[end]==lev{end+=1;}
  if lev&128!=0 {for i in start..end{levels[i]=lev&127;}}else{
   let prev=if start==0{bases[0]}else{explicit[start-1]&127};let next=if end==n{bases[n-1]}else{explicit[end]&127};
   let sor=if start>0&&classes[start-1]==7{dir(bases[start])}else{dir(lev.max(prev))};let eor=dir(lev.max(next));
   let indices:Vec<usize>=(start..end).filter(|i|!removed(classes[*i])).collect();let mut types:Vec<u8>=indices.iter().map(|i|classes[*i]).collect();

   // W1: nonspacing marks inherit their preceding type.
   let mut prior=if classes[start]==17&&bases[start]==0{sor}else{10};for t in &mut types{if *t==17{*t=prior;}else{prior=*t;}}
   for t in &mut types {if *t==7{*t=dir(lev);}}
   // W2 and W3: European numbers following Arabic; Arabic strong -> R.
   let mut strong=sor;for t in &mut types{if *t==2&&strong==13{*t=5;}if matches!(*t,0|1|13){strong=*t;}}
   for t in &mut types{if *t==13{*t=1;}}
   // W4: numeric separators.
   if types.len()>2{for i in 1..types.len()-1{if types[i]==3&&types[i-1]==2&&types[i+1]==2{types[i]=2;}else if types[i]==6&&types[i-1]==types[i+1]&&matches!(types[i-1],2|5){types[i]=types[i-1];}}}
   // W5: European terminators next to European numbers.
   let mut i=0;while i<types.len(){if types[i]!=4{i+=1;continue;}let begin=i;while i<types.len()&&types[i]==4{i+=1;}if (begin>0&&types[begin-1]==2)||(i<types.len()&&types[i]==2){types[begin..i].fill(2);}}
   for t in &mut types{if matches!(*t,3|4|6){*t=10;}}
   // W7: European numbers following L resolve to L.
   strong=sor;for t in &mut types{if *t==2&&strong==0{*t=0;}if matches!(*t,0|1){strong=*t;}}
   // N1/N2: neutral sequences between strong types, with numbers acting as R.
   let mut i=0;while i<types.len(){if !matches!(types[i],7|8|9|10){i+=1;continue;}let begin=i;while i<types.len()&&matches!(types[i],7|8|9|10){i+=1;}let before=if begin==0{sor}else{if types[begin-1]==0{0}else{1}};let after=if i==types.len(){eor}else{if types[i]==0{0}else{1}};let resolved=if before==after{before}else{dir(lev)};types[begin..i].fill(resolved);}
   for (index,t) in indices.into_iter().zip(types){levels[index]=lev+if lev&1==0{if t==1{1}else if matches!(t,2|5){2}else{0}}else{if matches!(t,0|2|5){1}else{0}};}
  }start=end;
 }
 let mut i=n;while i>0{while i>0{i-=1;if matches!(classes[i],7|8|9|11|12|14|15|16|18){levels[i]=bases[i];}else{break;}}while i>0{i-=1;if removed(classes[i]){levels[i]=levels[i+1];}else if matches!(classes[i],7|8){levels[i]=bases[i];break;}}}
 (levels,None)
}
fn bases(classes:&[u8],_text:&[u16])->Vec<u8>{
 let mut result=vec![0;classes.len()];let mut start=0;
 while start<classes.len(){let mut end=start;while end<classes.len(){end+=1;if classes[end-1]==7{break;}}
 let level=classes[start..end].iter().find(|c|matches!(**c,0|1|13)).map_or(0,|c|if *c==0{0}else{1});result[start..end].fill(level);start=end;}
 result
}
fn reorder(text:&[u16],levels:&[u8])->Vec<u16>{
 let mut runs=Vec::new();let mut start=0;
 while start<text.len(){let mut end=start+1;while end<text.len()&&levels[end]==levels[start]{end+=1;}runs.push((text[start..end].to_vec(),levels[start]));start=end;}
 let mut order:Vec<usize>=(0..runs.len()).collect();
 if let Some(min)=levels.iter().filter(|l|**l&1==1).min(){
  for threshold in (*min..=levels.iter().copied().max().unwrap_or(0)).rev(){let mut i=0;while i<order.len(){if runs[order[i]].1<threshold{i+=1;continue;}let mut end=i+1;while end<order.len()&&runs[order[end]].1>=threshold{end+=1;}order[i..end].reverse();i=end;}}
 }
 let mut result=Vec::new();for index in order{let units=&runs[index].0;let level=runs.iter().find(|r|r.0==*units).unwrap().1;
 if level&1==0{result.extend(units);}else{result.extend(units.iter().rev().map(|u|match *u{40=>41,41=>40,n=>n}));}}
 result
}

pub struct Analysis {pub levels:Vec<u8>,pub runs:Vec<(usize,usize,u8)>,pub base:u8,pub required:bool,pub output:Vec<u16>}
fn class(point:u32)->u8 {let ranges=super::bidi_data11::RANGES;let index=ranges.partition_point(|r|r.1<point);ranges.get(index).filter(|r|r.0<=point).map_or(0,|r|r.2)}
pub fn analyze(text:&[u16])->Analysis {
 let mut classes=vec![0;text.len()];let mut i=0;
 while i<text.len(){let unit=text[i];if (0xd800..=0xdbff).contains(&unit)&&i+1<text.len()&&(0xdc00..=0xdfff).contains(&text[i+1]){classes[i]=18;classes[i+1]=class(0x10000+((unit as u32-0xd800)<<10)+(text[i+1] as u32-0xdc00));i+=2;}else{classes[i]=class(unit as u32);i+=1;}}
 let para=bases(&classes,text);let base=para.first().copied().unwrap_or(0);let (levels,collapsed)=legacy(&classes,&para);let run_levels=collapsed.map_or_else(||levels.clone(),|l|vec![l;text.len()]);
 let mut runs=Vec::new();let mut start=0;while start<text.len(){let mut end=start+1;while end<text.len()&&run_levels[end]==run_levels[start]{end+=1;}runs.push((start,end,run_levels[start]));start=end;}
 let required=classes.iter().any(|c|matches!(*c,1|5|13|14|15));let output=if required{reorder(text,&run_levels)}else{text.to_vec()};
 Analysis{levels,runs,base,required,output}
}
pub fn reorder_utf16(text:&[u16])->Vec<u16>{analyze(text).output}
#[cfg(test)]
mod tests {
 use super::*;
 struct Input{bytes:Vec<u8>,at:usize}
 impl Input{
  fn byte(&mut self)->u8{let b=self.bytes[self.at];self.at+=1;b}
  fn int(&mut self)->usize{(0..4).fold(0,|n,_|(n<<8)|self.byte() as usize)}
  fn text(&mut self)->Vec<u16>{let len=self.int();(0..len).map(|_|(self.byte() as u16)<<8|self.byte() as u16).collect()}
 }
 #[test]
 #[ignore="requires original 1.1 oracle fixtures: BIDI11_FIXTURES, separated by semicolons"]
 fn original_bytecode_corpora(){
  for path in std::env::var("BIDI11_FIXTURES").expect("BIDI11_FIXTURES").split(';'){
   let mut input=Input{bytes:std::fs::read(path).unwrap(),at:0};assert_eq!(input.int(),0x42444931);let count=input.int();
   for case in 0..count{
    let text=input.text();let required=input.byte()!=0;let base=input.int();let levels:Vec<u8>=(0..text.len()).map(|_|input.byte()).collect();let run_count=input.int();let runs:Vec<_>=(0..run_count).map(|_|(input.int(),input.int(),input.byte())).collect();let output=input.text();let actual=analyze(&text);
    assert_eq!(actual.required,required,"requires {path} case {case}");assert_eq!(actual.base as usize,base,"base {path} case {case}");assert_eq!(actual.levels,levels,"levels {path} case {case}");assert_eq!(actual.runs,runs,"runs {path} case {case}");assert_eq!(actual.output,output,"output {path} case {case}");
   }
   assert_eq!(input.at,input.bytes.len());println!("BIDI11 EXACT {count} cases: {path}");
  }
 }
}
