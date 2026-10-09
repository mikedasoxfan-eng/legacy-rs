//! Release 1.1 StringTranslate: UTF-8 line parsing, English fallback and locale flags.
use std::collections::{BTreeMap, HashMap};
#[path="java_format.rs"] mod java_format;
pub use java_format::FormatError;

pub struct Lang {
    table: HashMap<String, String>,
    languages: BTreeMap<String, String>,
    language: String,
    unicode: bool,
}
impl Default for Lang {
    fn default() -> Self { Self { table: HashMap::new(), languages: BTreeMap::new(), language: "en_US".into(), unicode: false } }
}
impl Lang {
    /// Load a translation overlay using 1.1's literal split rules, not Properties.
    pub fn load(&mut self, bytes: &[u8]) { load_table(&mut self.table, bytes); }
    /// languages.txt is not trimmed and is ordered by Java String comparison.
    pub fn load_languages(&mut self, bytes: &[u8]) {
        self.languages = pairs(bytes, false).into_iter().collect();
    }
    pub fn languages(&self) -> Vec<(&str, &str)> {
        let mut entries: Vec<_> = self.languages.iter().map(|(k,v)|(k.as_str(),v.as_str())).collect();
        entries.sort_by(|a,b| a.0.encode_utf16().cmp(b.0.encode_utf16())); entries
    }
    pub fn translation_entries(&self) -> Vec<(&str, &str)> {
        let mut entries: Vec<_> = self.table.iter().map(|(k,v)|(k.as_str(),v.as_str())).collect();
        entries.sort_by(|a,b| a.0.encode_utf16().cmp(b.0.encode_utf16())); entries
    }
    pub fn language(&self) -> &str { &self.language }
    pub fn unicode(&self) -> bool { self.unicode }
    pub fn bidi(&self) -> bool { Self::language_is_bidi(&self.language) }
    pub fn language_is_bidi(code: &str) -> bool { matches!(code, "ar_SA" | "he_IL") }
    /// Switch with a fresh English table. Caller reads resources from the installed jar.
    /// An overlay read failure leaves the active table/code unchanged, as in Java.
    pub fn load_language<F>(&mut self, code: &str, mut read: F) -> Result<bool, String>
    where F: FnMut(&str) -> Result<Vec<u8>, String> {
        if code == self.language { return Ok(false); }
        let mut table = HashMap::new();
        if let Ok(bytes) = read("lang/en_US.lang") { load_table(&mut table, &bytes); }
        self.unicode = false;
        if code != "en_US" {
            let bytes = read(&format!("lang/{code}.lang"))?;
            load_table(&mut table, &bytes);
            self.unicode = table.values().any(|value| value.encode_utf16().any(|unit| unit >= 256));
        }
        self.language = code.into(); self.table = table; Ok(true)
    }
    /// `translateKey`: the value, or the key itself if missing.
    pub fn translate_key(&self, key: &str) -> String {
        self.table.get(key).cloned().unwrap_or_else(|| key.to_string())
    }

    /// StringTranslate propagates Formatter's unchecked exception on malformed input.
    /// Callers that need to expose the original exception can use the checked entrypoint.
    pub fn translate_key_format(&self, key: &str, args: &[String]) -> String {
        self.try_translate_key_format(key,args).unwrap_or_else(|e|panic!("{}: {}",e.class,e.message))
    }
    pub fn try_translate_key_format(&self,key:&str,args:&[String])->Result<String,FormatError> {
        java_format::format_strings(&self.translate_key(key),args)
    }

    /// `translateNamedKey`: `key.name` or "".
    pub fn translate_named_key(&self, key: &str) -> String {
        self.table.get(&format!("{key}.name")).cloned().unwrap_or_default()
    }
}

fn pairs(bytes: &[u8], translations: bool) -> Vec<(String, String)> {
    let text = String::from_utf8_lossy(bytes).replace("\r\n", "\n").replace('\r', "\n");
    text.split('\n').filter_map(|raw| {
        let line = if translations { raw.trim_matches(|ch: char| ch <= '\u{20}') } else { raw };
        if translations && line.starts_with('#') { return None; }
        let mut pieces: Vec<_> = line.split('=').collect();
        while pieces.last().is_some_and(|piece| piece.is_empty()) { pieces.pop(); }
        if pieces.len() == 2 { Some((pieces[0].into(), pieces[1].into())) } else { None }
    }).collect()
}
fn load_table(table: &mut HashMap<String, String>, bytes: &[u8]) { table.extend(pairs(bytes, true)); }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn installed_greek_overlay_preserves_original_formatter_exception() {
        let mut lang=Lang::default();lang.load(b"achievement.requires=Requires '%1$s'\n");
        assert_eq!(lang.translate_key_format("achievement.requires",&["Taking Inventory".into()]),"Requires 'Taking Inventory'");
        lang.load("achievement.requires=Απαιτείται '%1$ s'\n".as_bytes());
        let error=lang.try_translate_key_format("achievement.requires",&["Taking Inventory".into()]).unwrap_err();
        assert_eq!(error.class,"java.util.FormatFlagsConversionMismatchException");
        assert_eq!(error.message,"Conversion = s, Flags =  ");
        assert!(std::panic::catch_unwind(||lang.translate_key_format("achievement.requires",&["Taking Inventory".into()])).is_err());
    }
    #[test]
    fn formatter_indexes_and_errors_use_java_string_argument_rules() {
        let lang=Lang::default();let args=vec!["Aé".into(),"Bß".into()];
        assert_eq!(lang.translate_key_format("%2$s/%s/%<s/%1$s",&args),"Bß/Aé/Aé/Aé");
        assert_eq!(lang.translate_key_format("[%1$5s][%1$-5s][%1$.2s]%%",&args[..1]),"[   Aé][Aé   ][Aé]%");
        assert_eq!(lang.try_translate_key_format("%d",&args).unwrap_err().class,"java.util.IllegalFormatConversionException");
        let error=lang.try_translate_key_format("%2$s",&args[..1]).unwrap_err();
        assert_eq!(error.class,"java.util.MissingFormatArgumentException");assert_eq!(error.message,"Format specifier '%2$s'");
    }
    #[test]
    fn utf8_parser_keeps_java_split_and_trim_semantics() {
        let mut lang=Lang::default();
        lang.load(" #skip=x\na=\nb=value=\nc=one=two\n spaced = interior \n!literal=x\nslash=\\u0041\\n\n\u{a0}key\u{a0}=\u{a0}value\u{a0}\n".as_bytes());
        assert_eq!(lang.translate_key("a"),"a");
        assert_eq!(lang.translate_key("b"),"value");
        assert_eq!(lang.translate_key("c"),"c");
        assert_eq!(lang.translate_key("spaced ")," interior");
        assert_eq!(lang.translate_key("!literal"),"x");
        assert_eq!(lang.translate_key("slash"),"\\u0041\\n");
        assert_eq!(lang.translate_key("\u{a0}key\u{a0}"),"\u{a0}value\u{a0}");
    }
    #[test]
    fn overlay_fallback_and_flags_follow_language_switches() {
        let mut lang=Lang::default();lang.load(b"fallback=english\nname=english\n");
        let reader=|path:&str|Ok(if path=="lang/en_US.lang" {b"fallback=english\nname=english\n".to_vec()} else {"name=×¢×‘×¨×™×ª\n".as_bytes().to_vec()});
        assert!(!lang.load_language("en_US",reader).unwrap());
        assert!(lang.load_language("he_IL",reader).unwrap());
        assert_eq!(lang.translate_key("fallback"),"english");
        assert_eq!(lang.translate_key("name"),"×¢×‘×¨×™×ª");assert!(lang.unicode());assert!(lang.bidi());
        assert!(lang.load_language("en_US",reader).unwrap());assert!(!lang.unicode());assert!(!lang.bidi());
        assert_eq!(lang.translate_key("name"),"english");
    }
    #[test]
    fn language_list_does_not_trim_or_treat_comments_as_properties() {
        let mut lang=Lang::default();lang.load_languages("z=Last\n a = First \nz=Override\nempty=\n#key=value\n".as_bytes());
        assert_eq!(lang.languages(),vec![(" a "," First "),("#key","value"),("z","Override")]);
    }
}
