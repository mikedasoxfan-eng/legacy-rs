//! `StatsSyncher` and the file half of `StatFileWriter`: statistics and achievements kept in
//! `stats/stats_<user>_unsent.dat` next to `options.txt`, in the original's JSON with its MD5
//! checksum, so a stats file moves between this and the real game.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Default, Debug)]
struct Transfers {
    busy: bool,
    received: Option<HashMap<i32, i32>>,
    unsent: Option<HashMap<i32, i32>>,
}

pub struct Syncher {
    /// `field_27434_e`, `field_27432_g`, `field_27430_i`: the unsent file and its .tmp/.old.
    unsent: [PathBuf; 3],
    /// `field_27433_f`, `field_27431_h`, `field_27429_j`: the "sent" file (only ever read).
    sent: [PathBuf; 3],
    user: String,
    /// `AchievementMap`: stat id -> GUID, from `achievement/map.txt`.
    guids: HashMap<i32, String>,
    /// `field_27427_l`: ticks until the next periodic write.
    busy: i32,
    /// `field_27437_b`: the sent file's stats, merged into the totals on the next tick.
    transfers: Arc<Mutex<Transfers>>,
}

impl std::fmt::Debug for Syncher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Syncher").field("unsent", &self.unsent[0]).finish()
    }
}

impl Clone for Syncher {
    fn clone(&self) -> Self {
        Syncher {
            unsent: self.unsent.clone(),
            sent: self.sent.clone(),
            user: self.user.clone(),
            guids: self.guids.clone(),
            busy: self.busy,
            transfers: self.transfers.clone(),
        }
    }
}

impl Syncher {
    /// `new StatFileWriter(session, dir)` + `new StatsSyncher`: returns the syncher and the
    /// unsent stats to add (`func_27179_a`), and starts the "receive" of the sent file.
    pub fn open(dir: &Path, user: &str, map_txt: &str) -> (Syncher, Option<HashMap<i32, i32>>) {
        let stats = dir.join("stats");
        if !stats.exists() {
            let _ = std::fs::create_dir(&stats);
        }
        // Old clients kept the files in the data directory itself.
        if let Ok(list) = std::fs::read_dir(dir) {
            for e in list.flatten() {
                let name = e.file_name().to_string_lossy().into_owned();
                if name.starts_with("stats_") && name.ends_with(".dat") {
                    let to = stats.join(&name);
                    if !to.exists() {
                        println!("Relocating {name}");
                        let _ = std::fs::rename(e.path(), to);
                    }
                }
            }
        }
        let lower = user.to_lowercase();
        let file = |suffix: &str| stats.join(format!("stats_{lower}{suffix}"));
        let unsent = [file("_unsent.dat"), file("_unsent.tmp"), file("_unsent.old")];
        let sent = [file(".dat"), file(".tmp"), file(".old")];
        if lower != user {
            for (suffix, to) in [
                ("_unsent.dat", &unsent[0]),
                (".dat", &sent[0]),
                ("_unsent.old", &unsent[2]),
                (".old", &sent[2]),
                ("_unsent.tmp", &unsent[1]),
                (".tmp", &sent[1]),
            ] {
                let from = stats.join(format!("stats_{user}{suffix}"));
                if from.is_file() && !to.exists() {
                    let _ = std::fs::rename(from, to);
                }
            }
        }
        let guids = map_txt
            .lines()
            .filter_map(|l| {
                let mut p = l.split(',');
                Some((p.next()?.trim().parse().ok()?, p.next()?.to_string()))
            })
            .collect();
        let mut s = Syncher { unsent, sent, user: user.to_string(), guids, busy: 0, transfers: Arc::default() };
        let loaded = if s.unsent[0].exists() { s.read_first(&s.unsent) } else { None };
        // func_27418_a: ThreadStatSyncherReceive reads the sent file.
        s.receive();
        (s, loaded)
    }

    /// Parse one stats file (for the oracle check).
    pub fn read_file(path: &Path, map_txt: &str) -> Option<HashMap<i32, i32>> {
        let guids = map_txt.lines().filter_map(|line| {
            let (id, guid) = line.split_once(',')?;
            Some((id.trim().parse().ok()?, guid.to_string()))
        }).collect();
        let s = Syncher { unsent: Default::default(), sent: Default::default(),
            user: "Player".into(), guids, busy: 0, transfers: Arc::default() };
        let text: String = std::fs::read_to_string(path).ok()?.lines().collect();
        s.parse(&text)
    }

    /// `func_27415_a`: the .dat, else the .old, else the .tmp.
    fn read_first(&self, files: &[PathBuf; 3]) -> Option<HashMap<i32, i32>> {
        let [dat, tmp, old] = files;
        let f = if dat.exists() {
            dat
        } else if old.exists() {
            old
        } else if tmp.exists() {
            tmp
        } else {
            return None;
        };
        // BufferedReader.readLine joined without the line breaks.
        let text: String = std::fs::read_to_string(f).ok()?.lines().collect();
        self.parse(&text)
    }

    /// `StatFileWriter.func_27177_a`
    fn parse(&self, text: &str) -> Option<HashMap<i32, i32>> {
        let root = match json::parse(text) {
            Some(j) => j,
            None => {
                eprintln!("stats: invalid syntax");
                return Some(HashMap::new());
            }
        };
        let mut map = HashMap::new();
        let mut sum = String::new();
        for change in root.get("stats-change")?.array()? {
            let (k, v) = change.object()?.first()?;
            let id: i32 = k.parse().ok()?;
            let value: i32 = v.text()?.parse().ok()?;
            if !super::stats::is_registered(id) {
                println!("{id} is not a valid stat");
                continue;
            }
            sum.push_str(self.guid(id));
            sum.push(',');
            sum.push_str(&value.to_string());
            sum.push(',');
            map.insert(id, value);
        }
        if Some(md5_hex("local", &sum).as_str()) != root.get("checksum").and_then(|c| c.text()) {
            println!("CHECKSUM MISMATCH");
            return None;
        }
        Some(map)
    }

    fn guid(&self, id: i32) -> &str {
        self.guids.get(&id).map_or("null", |s| s.as_str())
    }

    /// `StatFileWriter.func_27185_a(user, "local", map)`
    fn format(&self, map: &HashMap<i32, i32>) -> String {
        let mut out = String::from("{\r\n");
        out.push_str("  \"user\":{\r\n");
        out.push_str(&format!("    \"name\":\"{}\",\r\n", self.user));
        out.push_str("    \"sessionid\":\"local\"\r\n");
        out.push_str("  },\r\n");
        out.push_str("  \"stats-change\":[");
        let mut ids: Vec<_> = map.keys().copied().collect();
        ids.sort_unstable();
        let mut sum = String::new();
        for (i, id) in ids.iter().enumerate() {
            if i > 0 {
                out.push_str("},");
            }
            out.push_str(&format!("\r\n    {{\"{id}\":{}", map[id]));
            sum.push_str(&format!("{},{},", self.guid(*id), map[id]));
        }
        if !ids.is_empty() {
            out.push('}');
        }
        out.push_str("\r\n  ],\r\n");
        out.push_str(&format!("  \"checksum\":\"{}\"\r\n", md5_hex("local", &sum)));
        out.push('}');
        out
    }

    /// `func_27410_a`: write the .tmp, the .dat becomes the .old, the .tmp the .dat.
    pub fn write(&self, map: &HashMap<i32, i32>) {
        self.write_files(map, &self.unsent);
    }

    fn write_files(&self, map: &HashMap<i32, i32>, files: &[PathBuf; 3]) {
        let [dat, tmp, old] = files;
        if let Err(e) = std::fs::write(tmp, self.format(map)) {
            eprintln!("stats: {e}");
            return;
        }
        if old.exists() {
            let _ = std::fs::remove_file(old);
        }
        if dat.exists() {
            let _ = std::fs::rename(dat, old);
        }
        let _ = std::fs::rename(tmp, dat);
    }

    /// `func_27420_b`: ready for a periodic write.
    pub fn ready(&self) -> bool {
        let state = self.transfers.lock().unwrap();
        self.busy <= 0 && !state.busy && state.unsent.is_none()
    }

    /// `func_27424_a`: the periodic write (ThreadStatSyncherSend).
    pub fn send(&mut self, map: &HashMap<i32, i32>) {
        let mut state = self.transfers.lock().unwrap();
        assert!(!state.busy, "Can't save stats while StatsSyncher is busy!");
        state.busy = true;
        self.busy = 100;
        let worker = self.clone();
        let map = map.clone();
        drop(state);
        std::thread::spawn(move || {
            worker.write(&map);
            worker.transfers.lock().unwrap().busy = false;
        });
    }

    /// `beginReceiveStats` / `ThreadStatSyncherReceive.run`.
    pub fn receive(&mut self) {
        let mut state = self.transfers.lock().unwrap();
        assert!(!state.busy, "Can't get stats from server while StatsSyncher is busy!");
        state.busy = true;
        self.busy = 100;
        let worker = self.clone();
        drop(state);
        std::thread::spawn(move || {
            let received = worker.transfers.lock().unwrap().received.clone();
            if let Some(map) = received {
                worker.write_files(&map, &worker.sent);
            } else if worker.sent[0].exists() {
                worker.transfers.lock().unwrap().received = worker.read_first(&worker.sent);
            }
            worker.transfers.lock().unwrap().busy = false;
        });
    }

    /// `syncStatsFileWithMap`: wait at most 29 times, then perform the final write.
    pub fn sync(&self, map: &HashMap<i32, i32>) {
        for _ in 0..29 {
            if !self.transfers.lock().unwrap().busy { break; }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        self.transfers.lock().unwrap().busy = true;
        self.write(map);
        self.transfers.lock().unwrap().busy = false;
    }

    /// Deterministic fixture barrier; gameplay uses the original tick/ready protocol.
    pub(crate) fn wait_idle(&self) {
        while self.transfers.lock().unwrap().busy {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    /// `func_27425_c`: count down, and hand over the sent file's stats once.
    pub fn tick(&mut self) -> (Option<HashMap<i32, i32>>, Option<HashMap<i32, i32>>) {
        if self.busy > 0 {
            self.busy -= 1;
        }
        let mut state = self.transfers.lock().unwrap();
        (state.unsent.take(), state.received.take())
    }
}

/// `MD5String(salt).func_27369_a(text)`: `new BigInteger(1, md5).toString(16)`.
fn md5_hex(salt: &str, text: &str) -> String {
    let digest = md5(format!("{salt}{text}").as_bytes());
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    let trimmed = hex.trim_start_matches('0');
    if trimmed.is_empty() { "0".to_string() } else { trimmed.to_string() }
}

/// RFC 1321.
fn md5(data: &[u8]) -> [u8; 16] {
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14,
        20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6,
        10, 15, 21,
    ];
    let k: Vec<u32> = (0..64).map(|i| ((i as f64 + 1.0).sin().abs() * 4294967296.0) as u32).collect();
    let mut msg = data.to_vec();
    let bits = (data.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bits.to_le_bytes());
    let (mut a0, mut b0, mut c0, mut d0) = (0x67452301u32, 0xefcdab89u32, 0x98badcfeu32, 0x10325476u32);
    for chunk in msg.chunks_exact(64) {
        let m: Vec<u32> = chunk.chunks_exact(4).map(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]])).collect();
        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
        for i in 0..64 {
            let (f, g) = match i / 16 {
                0 => ((b & c) | (!b & d), i),
                1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                2 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let f = f.wrapping_add(a).wrapping_add(k[i]).wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i]));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }
    let mut out = [0u8; 16];
    for (i, v) in [a0, b0, c0, d0].iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    out
}

/// Just enough JSON for the stats file (`argo`'s JdomParser in the original).
mod json {
    pub enum Value {
        Object(Vec<(String, Value)>),
        Array(Vec<Value>),
        /// Strings and numbers both keep their text (`func_27216_b`).
        Text(String),
        Other,
    }

    impl Value {
        pub fn get(&self, key: &str) -> Option<&Value> {
            self.object()?.iter().find(|(k, _)| k == key).map(|(_, v)| v)
        }
        pub fn object(&self) -> Option<&Vec<(String, Value)>> {
            if let Value::Object(o) = self { Some(o) } else { None }
        }
        pub fn array(&self) -> Option<&Vec<Value>> {
            if let Value::Array(a) = self { Some(a) } else { None }
        }
        pub fn text(&self) -> Option<&str> {
            if let Value::Text(t) = self { Some(t) } else { None }
        }
    }

    pub fn parse(s: &str) -> Option<Value> {
        let mut p = Parser { s: s.as_bytes(), i: 0 };
        let v = p.value()?;
        p.ws();
        (p.i == p.s.len()).then_some(v)
    }

    struct Parser<'a> {
        s: &'a [u8],
        i: usize,
    }

    impl Parser<'_> {
        fn ws(&mut self) {
            while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\r' | b'\n') {
                self.i += 1;
            }
        }
        fn eat(&mut self, c: u8) -> bool {
            self.ws();
            if self.s.get(self.i) == Some(&c) {
                self.i += 1;
                true
            } else {
                false
            }
        }
        fn value(&mut self) -> Option<Value> {
            self.ws();
            match *self.s.get(self.i)? {
                b'{' => {
                    self.i += 1;
                    let mut o = Vec::new();
                    if self.eat(b'}') {
                        return Some(Value::Object(o));
                    }
                    loop {
                        self.ws();
                        let k = self.string()?;
                        if !self.eat(b':') {
                            return None;
                        }
                        o.push((k, self.value()?));
                        if self.eat(b'}') {
                            return Some(Value::Object(o));
                        }
                        if !self.eat(b',') {
                            return None;
                        }
                    }
                }
                b'[' => {
                    self.i += 1;
                    let mut a = Vec::new();
                    if self.eat(b']') {
                        return Some(Value::Array(a));
                    }
                    loop {
                        a.push(self.value()?);
                        if self.eat(b']') {
                            return Some(Value::Array(a));
                        }
                        if !self.eat(b',') {
                            return None;
                        }
                    }
                }
                b'"' => self.string().map(Value::Text),
                b'-' | b'0'..=b'9' => {
                    let start = self.i;
                    while self.i < self.s.len() && matches!(self.s[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                        self.i += 1;
                    }
                    Some(Value::Text(String::from_utf8_lossy(&self.s[start..self.i]).into_owned()))
                }
                _ => {
                    for word in ["true", "false", "null"] {
                        if self.s[self.i..].starts_with(word.as_bytes()) {
                            self.i += word.len();
                            return Some(Value::Other);
                        }
                    }
                    None
                }
            }
        }
        fn string(&mut self) -> Option<String> {
            if self.s.get(self.i) != Some(&b'"') {
                return None;
            }
            self.i += 1;
            let mut out = Vec::new();
            loop {
                let c = *self.s.get(self.i)?;
                self.i += 1;
                match c {
                    b'"' => return String::from_utf8(out).ok(),
                    b'\\' => {
                        let e = *self.s.get(self.i)?;
                        self.i += 1;
                        match e {
                            b'n' => out.push(b'\n'),
                            b't' => out.push(b'\t'),
                            b'r' => out.push(b'\r'),
                            b'b' => out.push(8),
                            b'f' => out.push(12),
                            b'u' => {
                                let hex = std::str::from_utf8(self.s.get(self.i..self.i + 4)?).ok()?;
                                let ch = char::from_u32(u32::from_str_radix(hex, 16).ok()?)?;
                                self.i += 4;
                                let mut buf = [0; 4];
                                out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                            }
                            other => out.push(other),
                        }
                    }
                    _ => out.push(c),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn md5_matches_known_digests() {
        assert_eq!(super::md5_hex("", ""), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(super::md5_hex("", "abc"), "900150983cd24fb0d6963f7d28e17f72");
    }
}
