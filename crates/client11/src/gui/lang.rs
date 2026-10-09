//! Release 1.1 StringTranslate: UTF-8 line parsing, English fallback and locale flags.
use std::collections::{BTreeMap, HashMap};

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

    /// `translateKeyFormat` for `%s`/`%d`-style arguments already rendered as strings.
    pub fn translate_key_format(&self, key: &str, args: &[String]) -> String {
        let mut out = String::new();
        let fmt = self.translate_key(key);
        let mut it = fmt.chars().peekable();
        let mut n = 0;
        while let Some(c) = it.next() {
            if c == '%' {
                // Positional `%N$s` / `%N$d`.
                let rest: String = it.clone().take(4).collect();
                let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
                if !digits.is_empty() && rest[digits.len()..].starts_with('$') {
                    let after = rest[digits.len() + 1..].chars().next();
                    if matches!(after, Some('s') | Some('d')) {
                        for _ in 0..digits.len() + 2 {
                            it.next();
                        }
                        let k: usize = digits.parse().unwrap_or(1);
                        out.push_str(args.get(k.saturating_sub(1)).map(String::as_str).unwrap_or(""));
                        continue;
                    }
                }
                match it.peek() {
                    Some('s') | Some('d') => {
                        it.next();
                        out.push_str(args.get(n).map(String::as_str).unwrap_or(""));
                        n += 1;
                        continue;
                    }
                    Some('%') => {
                        it.next();
                        out.push('%');
                        continue;
                    }
                    _ => {}
                }
            }
            out.push(c);
        }
        out
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
        let reader=|path:&str|Ok(if path=="lang/en_US.lang" {b"fallback=english\nname=english\n".to_vec()} else {"name=עברית\n".as_bytes().to_vec()});
        assert!(!lang.load_language("en_US",reader).unwrap());
        assert!(lang.load_language("he_IL",reader).unwrap());
        assert_eq!(lang.translate_key("fallback"),"english");
        assert_eq!(lang.translate_key("name"),"עברית");assert!(lang.unicode());assert!(lang.bidi());
        assert!(lang.load_language("en_US",reader).unwrap());assert!(!lang.unicode());assert!(!lang.bidi());
        assert_eq!(lang.translate_key("name"),"english");
    }
    #[test]
    fn language_list_does_not_trim_or_treat_comments_as_properties() {
        let mut lang=Lang::default();lang.load_languages("z=Last\n a = First \nz=Override\nempty=\n#key=value\n".as_bytes());
        assert_eq!(lang.languages(),vec![(" a "," First "),("#key","value"),("z","Override")]);
    }
}
