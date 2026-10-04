//! `StringTranslate`: `lang/en_US.lang` + `lang/stats_US.lang`, read like
//! `java.util.Properties.load(InputStream)` (ISO-8859-1, `\uXXXX` escapes, continuations).

use std::collections::HashMap;

#[derive(Default)]
pub struct Lang {
    table: HashMap<String, String>,
}

impl Lang {
    pub fn load(&mut self, bytes: &[u8]) {
        // ISO-8859-1: every byte is one char.
        let text: String = bytes.iter().map(|&b| b as char).collect();
        let text = text.replace("\r\n", "\n").replace('\r', "\n");
        let mut logical = String::new();
        for raw in text.split('\n') {
            let line = raw.trim_start_matches([' ', '\t', '\x0c']);
            if logical.is_empty() && (line.is_empty() || line.starts_with('#') || line.starts_with('!')) {
                continue;
            }
            // An odd number of trailing backslashes continues the line.
            let trailing = line.chars().rev().take_while(|&c| c == '\\').count();
            if trailing % 2 == 1 {
                logical.push_str(&line[..line.len() - 1]);
                continue;
            }
            logical.push_str(line);
            let entry = std::mem::take(&mut logical);
            self.put(&entry);
        }
        if !logical.is_empty() {
            let entry = std::mem::take(&mut logical);
            self.put(&entry);
        }
    }

    fn put(&mut self, line: &str) {
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        let mut key = String::new();
        while i < chars.len() {
            let c = chars[i];
            if c == '\\' {
                i += 1;
                if i < chars.len() {
                    let (ch, n) = unescape(&chars[i..]);
                    key.push(ch);
                    i += n;
                }
                continue;
            }
            if c == '=' || c == ':' || c == ' ' || c == '\t' || c == '\x0c' {
                break;
            }
            key.push(c);
            i += 1;
        }
        // Separator: whitespace, then at most one '=' or ':', then whitespace.
        while i < chars.len() && matches!(chars[i], ' ' | '\t' | '\x0c') {
            i += 1;
        }
        if i < chars.len() && matches!(chars[i], '=' | ':') {
            i += 1;
        }
        while i < chars.len() && matches!(chars[i], ' ' | '\t' | '\x0c') {
            i += 1;
        }
        let mut value = String::new();
        while i < chars.len() {
            if chars[i] == '\\' && i + 1 < chars.len() {
                let (ch, n) = unescape(&chars[i + 1..]);
                value.push(ch);
                i += 1 + n;
            } else {
                value.push(chars[i]);
                i += 1;
            }
        }
        self.table.insert(key, value);
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

fn unescape(rest: &[char]) -> (char, usize) {
    match rest[0] {
        't' => ('\t', 1),
        'n' => ('\n', 1),
        'r' => ('\r', 1),
        'f' => ('\x0c', 1),
        'u' if rest.len() >= 5 => {
            let hex: String = rest[1..5].iter().collect();
            match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                Some(c) => (c, 5),
                None => ('u', 1),
            }
        }
        c => (c, 1),
    }
}
