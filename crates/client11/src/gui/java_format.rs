//! Java8 Formatter for String-only arguments used by StringTranslate callers.
//! The parser validates the entire pattern before consuming arguments, as Java does.
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct FormatError { pub class: &'static str, pub message: String }
fn error(class:&'static str,message:String)->FormatError {FormatError{class,message}}
fn unknown(c:&str)->FormatError {error("java.util.UnknownFormatConversionException",format!("Conversion = '{c}'"))}
fn mismatch(c:char,flags:&str)->FormatError {error("java.util.FormatFlagsConversionMismatchException",format!("Conversion = {c}, Flags = {flags}"))}
#[derive(Debug)]
struct Spec { index:Option<usize>, flags:String, width:Option<usize>, precision:Option<usize>, date:Option<char>, conversion:char }
impl Spec {
 fn pattern(&self)->String {format!("%{}{}{}{}{}{}",self.flags,self.index.filter(|i|*i>0).map_or(String::new(),|i|format!("{i}$")),self.width.map_or(String::new(),|w|w.to_string()),self.precision.map_or(String::new(),|p|format!(".{p}")),self.date.map_or(String::new(),|c|c.to_string()),self.conversion)}
 fn validate(&self)->Result<(),FormatError> {
  let c=self.conversion.to_ascii_lowercase();let flags=&self.flags;
  if let Some(date)=self.date {if !"HIklMSLNpzZsQBbhAaCYyjmdeRTrDFc".contains(self.conversion){return Err(unknown(&format!("{date}{}",self.conversion)));}}
  else if !"bBhHsScCdoxXeEfgGaAn%".contains(self.conversion) {return Err(unknown(&self.conversion.to_string()));}
  let invalid=if self.date.is_some() {"#+ 0,("}else{match c {'b'|'h'|'s'=>"#+ 0,(",'c'=>"#+ 0,(",'d'=>"#",'o'|'x'=>",",'e'=>",",'a'=>",(",_=>""}};
  // General/character/date checks precede missing-width checks in Java8.
  let bad:String=flags.chars().filter(|f|invalid.contains(*f)).collect();
  if !bad.is_empty(){return Err(mismatch(self.conversion,&bad));}
  if c=='n' && self.date.is_none() {
   if let Some(p)=self.precision{return Err(error("java.util.IllegalFormatPrecisionException",p.to_string()));}
   if let Some(w)=self.width{return Err(error("java.util.IllegalFormatWidthException",w.to_string()));}
   if !flags.is_empty(){return Err(error("java.util.IllegalFormatFlagsException",format!("Flags = '{flags}'")));}
  }
  if c=='%' && self.date.is_none() {
   if let Some(p)=self.precision{return Err(error("java.util.IllegalFormatPrecisionException",p.to_string()));}
   if flags.chars().any(|f|f!='-'){return Err(error("java.util.IllegalFormatFlagsException",format!("Flags = '{flags}'")));}
  }
  if (flags.contains('-')||flags.contains('0'))&&self.width.is_none(){return Err(error("java.util.MissingFormatWidthException",self.pattern()));}
  if flags.contains('+')&&flags.contains(' ')||flags.contains('-')&&flags.contains('0'){return Err(error("java.util.IllegalFormatFlagsException",format!("Flags = '{flags}'")));}
  if (self.date.is_some()||"cdox".contains(c))&&self.precision.is_some(){return Err(error("java.util.IllegalFormatPrecisionException",self.precision.unwrap().to_string()));}
  Ok(())
 }
}
enum Piece { Literal(String), Format(Spec) }
fn digits(chars:&[char],at:&mut usize)->Option<usize>{let start=*at;while *at<chars.len()&&chars[*at].is_ascii_digit(){*at+=1;}if *at==start{None}else{chars[start..*at].iter().collect::<String>().parse().ok()}}
pub fn format_strings(fmt:&str,args:&[String])->Result<String,FormatError> {
 let chars:Vec<_>=fmt.chars().collect();let mut pieces=Vec::new();let mut at=0;
 while at<chars.len() {
  if chars[at]!='%' {let start=at;while at<chars.len()&&chars[at]!='%'{at+=1;}pieces.push(Piece::Literal(chars[start..at].iter().collect()));continue;}
  let start=at;at+=1;let after=at;let number=digits(&chars,&mut at);
  let index=if number.is_some()&&chars.get(at)==Some(&'$'){at+=1;number}else{at=after;None};
  let mut flags=String::new();while chars.get(at).is_some_and(|c|"-#+ 0,(<".contains(*c)){let c=chars[at];if flags.contains(c){return Err(error("java.util.DuplicateFormatFlagsException",format!("Flags = '{c}'")));}flags.push(c);at+=1;}
  flags="-#+ 0,(<".chars().filter(|c|flags.contains(*c)).collect();
  let width=digits(&chars,&mut at);let precision=if chars.get(at)==Some(&'.'){at+=1;let p=digits(&chars,&mut at);if p.is_none(){return Err(unknown(&chars.get(start+1).unwrap_or(&'%').to_string()));}p}else{None};
  let date=if chars.get(at).is_some_and(|c|matches!(c,'t'|'T'))&&chars.get(at+1).is_some_and(|c|c.is_ascii_alphabetic()){let d=chars[at];at+=1;Some(d)}else{None};
  let Some(&conversion)=chars.get(at).filter(|c|c.is_ascii_alphabetic()||**c=='%') else{return Err(unknown(&chars.get(start+1).unwrap_or(&'%').to_string()));};at+=1;
  let spec=Spec{index,flags,width,precision,date,conversion};spec.validate()?;pieces.push(Piece::Format(spec));
 }
 let mut out=String::new();let mut ordinary=0;let mut previous=None;
 for piece in pieces {
  let s=match piece{Piece::Literal(s)=>{out.push_str(&s);continue;},Piece::Format(s)=>s};
  let c=s.conversion.to_ascii_lowercase();
  let mut value=if c=='%'&&s.date.is_none(){"%".into()}else if c=='n'&&s.date.is_none(){if cfg!(windows){"\r\n".into()}else{"\n".into()}}else {
   let index=if s.flags.contains('<'){previous}else if let Some(i)=s.index.filter(|i|*i>0){Some(i-1)}else{let i=ordinary;ordinary+=1;Some(i)};
   let Some((index,arg))=index.and_then(|i|args.get(i).map(|a|(i,a))) else{return Err(error("java.util.MissingFormatArgumentException",format!("Format specifier '{}'",s.pattern())));};previous=Some(index);
   if s.date.is_some()||!"bhs".contains(c){return Err(error("java.util.IllegalFormatConversionException",format!("{} != java.lang.String",s.conversion)));}
   match c {'b'=>"true".into(),'h'=>format!("{:x}",arg.encode_utf16().fold(0i32,|n,c|n.wrapping_mul(31).wrapping_add(c as i32)) as u32),_=>arg.clone()}
  };
  if let Some(p)=s.precision {let utf16:Vec<_>=value.encode_utf16().take(p).collect();value=String::from_utf16_lossy(&utf16);}
  if s.conversion.is_ascii_uppercase(){value=value.to_uppercase();}
  if let Some(w)=s.width {let padding=w.saturating_sub(value.encode_utf16().count());if s.flags.contains('-'){value.push_str(&" ".repeat(padding));}else{value=" ".repeat(padding)+&value;}}
  out.push_str(&value);
 }
 Ok(out)
}
