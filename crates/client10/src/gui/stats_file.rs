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
    /// `field_27433_f`, `field_27431_h`, `field_27429_j`: the "sent" file and rotation files.
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
        f.debug_struct("Syncher")
            .field("unsent", &self.unsent[0])
            .finish()
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
        Self::open_checked(dir, Some(user), map_txt).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Checked Java constructor boundary. mkdir/rename failures are ignored,
    /// but a null listFiles result or session username aborts construction.
    pub(crate) fn open_checked(
        dir: &Path,
        user: Option<&str>,
        map_txt: &str,
    ) -> Result<(Syncher, Option<HashMap<i32, i32>>), &'static str> {
        let stats = dir.join("stats");
        if !stats.exists() {
            let _ = std::fs::create_dir(&stats);
        }
        // Old clients kept the files in the data directory itself.
        {
            let list = std::fs::read_dir(dir).map_err(|_| "NullPointerException")?;
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
        let user = user.ok_or("NullPointerException")?;
        let lower = user.to_lowercase();
        let file = |suffix: &str| stats.join(format!("stats_{lower}{suffix}"));
        let unsent = [
            file("_unsent.dat"),
            file("_unsent.tmp"),
            file("_unsent.old"),
        ];
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
        let mut s = Syncher {
            unsent,
            sent,
            user: user.to_string(),
            guids,
            busy: 0,
            transfers: Arc::default(),
        };
        let loaded = if s.unsent[0].exists() {
            s.read_first(&s.unsent)
        } else {
            None
        };
        // func_27418_a: ThreadStatSyncherReceive reads the sent file.
        s.receive();
        Ok((s, loaded))
    }

    /// Parse one stats file (for the oracle check).
    pub fn read_file(path: &Path, map_txt: &str) -> Option<HashMap<i32, i32>> {
        let guids = map_txt
            .lines()
            .filter_map(|line| {
                let (id, guid) = line.split_once(',')?;
                Some((id.trim().parse().ok()?, guid.to_string()))
            })
            .collect();
        let s = Syncher {
            unsent: Default::default(),
            sent: Default::default(),
            user: "Player".into(),
            guids,
            busy: 0,
            transfers: Arc::default(),
        };
        let text: String = decode_file(&std::fs::read(path).ok()?);
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
        let text: String = decode_file(&std::fs::read(f).ok()?);
        self.parse(&text)
    }

    /// `StatFileWriter.func_27177_a`
    fn parse(&self, text: &str) -> Option<HashMap<i32, i32>> {
        self.parse_checked(text).unwrap_or_else(|error| {
            // Argo never returns for EOF in an unclosed string. Preserve that
            // liveness bug at the file boundary; the checked API reports it so
            // the oracle can observe it without exhausting either process.
            if error == "Nonterminating" {
                loop {
                    std::thread::park();
                }
            }
            eprintln!("stats: {error}");
            None
        })
    }

    /// Static StatFileWriter parsing distinguishes syntax rejection (an empty map)
    /// from unchecked schema/number exceptions. StatsSyncher catches the latter.
    pub(crate) fn parse_checked(
        &self,
        text: &str,
    ) -> Result<Option<HashMap<i32, i32>>, &'static str> {
        self.parse_units_checked(&text.encode_utf16().collect::<Vec<_>>())
    }

    pub(crate) fn parse_units_checked(
        &self,
        text: &[u16],
    ) -> Result<Option<HashMap<i32, i32>>, &'static str> {
        let root = match json::parse(text)? {
            Some(j) => j,
            None => {
                eprintln!("stats: invalid syntax");
                return Ok(Some(HashMap::new()));
            }
        };
        let mut map = HashMap::new();
        let mut sum = String::new();
        let schema = "J_JsonNodeDoesNotMatchPathElementsException";
        let changes = root
            .get("stats-change")
            .and_then(|v| v.array())
            .ok_or(schema)?;
        for change in changes {
            let fields = change.object().ok_or("IllegalStateException")?;
            let (k, v) = json::first_field(fields).ok_or("NoSuchElementException")?;
            let id = json::integer(k).ok_or("NumberFormatException")?;
            let value = json::integer(v.text().ok_or("IllegalStateException")?)
                .ok_or("NumberFormatException")?;
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
        let checksum = root
            .get("checksum")
            .and_then(|c| c.string())
            .ok_or(schema)?;
        if md5_checked("local", &sum)?
            .encode_utf16()
            .collect::<Vec<_>>()
            != checksum
        {
            println!("CHECKSUM MISMATCH");
            return Ok(None);
        }
        Ok(Some(map))
    }

    pub(crate) fn fixture(map_txt: &str) -> Self {
        Self {
            unsent: Default::default(),
            sent: Default::default(),
            user: "Player".into(),
            guids: map_txt
                .lines()
                .filter_map(|l| {
                    let (id, guid) = l.split_once(',')?;
                    Some((id.parse().ok()?, guid.into()))
                })
                .collect(),
            busy: 0,
            transfers: Arc::default(),
        }
    }

    fn guid(&self, id: i32) -> &str {
        self.guids.get(&id).map_or("null", |s| s.as_str())
    }

    /// Serialize the supplied Map's iteration order; StatFileWriter does not
    /// escape user/session strings or discard zero/null values.
    pub(crate) fn format_entries(
        &self,
        user: Option<&str>,
        session: Option<&str>,
        entries: impl IntoIterator<Item = (i32, Option<i32>)>,
    ) -> String {
        let user = user.map(|s| s.encode_utf16().collect::<Vec<_>>());
        let session = session.map(|s| s.encode_utf16().collect::<Vec<_>>());
        let out = self
            .format_units(user.as_deref(), session.as_deref(), entries)
            .unwrap_or_else(|e| panic!("{e}"));
        String::from_utf16(&out).expect("Rust strings contain valid UTF-16")
    }

    pub(crate) fn format_units(
        &self,
        user: Option<&[u16]>,
        session: Option<&[u16]>,
        entries: impl IntoIterator<Item = (i32, Option<i32>)>,
    ) -> Result<Vec<u16>, &'static str> {
        fn append(out: &mut Vec<u16>, s: &str) {
            out.extend(s.encode_utf16());
        }
        let mut out = Vec::new();
        append(&mut out, "{\r\n");
        if let (Some(user), Some(session)) = (user, session) {
            append(&mut out, "  \"user\":{\r\n    \"name\":\"");
            out.extend(user);
            append(&mut out, "\",\r\n    \"sessionid\":\"");
            out.extend(session);
            append(&mut out, "\"\r\n  },\r\n");
        }
        append(&mut out, "  \"stats-change\":[");
        let mut sum = String::new();
        let mut first = true;
        for (id, value) in entries {
            if !first {
                append(&mut out, "},");
            }
            first = false;
            let value = value.map_or_else(|| "null".to_string(), |n| n.to_string());
            append(&mut out, &format!("\r\n    {{\"{id}\":{value}"));
            sum.push_str(&format!("{},{value},", self.guid(id)));
        }
        if !first {
            append(&mut out, "}");
        }
        let mut checksum =
            session.map_or_else(|| "null".encode_utf16().collect::<Vec<_>>(), |s| s.to_vec());
        checksum.extend(sum.encode_utf16());
        append(
            &mut out,
            &format!(
                "\r\n  ],\r\n  \"checksum\":\"{}\"\r\n}}",
                md5_units(&checksum)?
            ),
        );
        Ok(out)
    }

    /// File writes use the local session. Vanilla's StatBase keys inherit JVM
    /// identity hashes, so its iteration input varies between original JVMs.
    /// Sorted IDs select a reproducible local input; format_entries is byte
    /// identical when supplied the original iteration order (statfile10_181).
    fn format(&self, map: &HashMap<i32, i32>) -> String {
        let mut ids: Vec<_> = map.keys().copied().collect();
        ids.sort_unstable();
        self.format_entries(
            Some(&self.user),
            Some("local"),
            ids.into_iter().map(|id| (id, Some(map[&id]))),
        )
    }

    /// `func_27410_a`: write the .tmp, the .dat becomes the .old, the .tmp the .dat.
    pub fn write(&self, map: &HashMap<i32, i32>) {
        self.write_files(map, &self.unsent);
    }

    fn write_files(&self, map: &HashMap<i32, i32>, files: &[PathBuf; 3]) {
        let [dat, tmp, old] = files;
        if let Err(e) = std::fs::write(tmp, encode_file(&self.format(map))) {
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
        assert!(
            !state.busy,
            "Can't get stats from server while StatsSyncher is busy!"
        );
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
            if !self.transfers.lock().unwrap().busy {
                break;
            }
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

    /// The original fixture sets its private volatile mailboxes by reflection.
    pub(crate) fn fixture_mailboxes(&mut self, sent: HashMap<i32, i32>, unsent: HashMap<i32, i32>) {
        self.wait_idle();
        let mut state = self.transfers.lock().unwrap();
        state.received = Some(sent);
        state.unsent = Some(unsent);
        drop(state);
        self.receive();
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
    md5_checked(salt, text).unwrap_or_else(|e| panic!("{e}"))
}

pub(crate) fn encode_file(text: &str) -> Vec<u8> {
    encode_units(&text.encode_utf16().collect::<Vec<_>>())
}

pub(crate) fn encode_units(text: &[u16]) -> Vec<u8> {
    let mut out = Vec::new();
    for c in char::decode_utf16(text.iter().copied()).map(|c| c.unwrap_or('\u{fffd}')) {
        // Java's windows-1252 encoder replaces one surrogate pair with one '?'.
        let byte = (0..=255)
            .find(|&b| {
                b != 0x81
                    && b != 0x8d
                    && b != 0x8f
                    && b != 0x90
                    && b != 0x9d
                    && crate::texture_packs::windows_1252(&[b]).chars().next() == Some(c)
            })
            .unwrap_or(b'?');
        out.push(byte);
    }
    out
}

pub(crate) fn decode_file(bytes: &[u8]) -> String {
    // BufferedReader.readLine removes CR, LF and CRLF (also a trailing CR).
    crate::texture_packs::windows_1252(bytes).replace(['\r', '\n'], "")
}

pub(crate) fn md5_checked(salt: &str, text: &str) -> Result<String, &'static str> {
    md5_units(&format!("{salt}{text}").encode_utf16().collect::<Vec<_>>())
}

pub(crate) fn md5_units(joined: &[u16]) -> Result<String, &'static str> {
    let bytes = encode_units(joined);
    let count = joined.len();
    // MD5String supplies String.length rather than the encoded byte count.
    if count > bytes.len() {
        return Err("IllegalArgumentException");
    }
    let digest = md5(&bytes[..count]);
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    let trimmed = hex.trim_start_matches('0');
    if trimmed.is_empty() {
        Ok("0".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

/// RFC 1321.
fn md5(data: &[u8]) -> [u8; 16] {
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    let k: Vec<u32> = (0..64)
        .map(|i| ((i as f64 + 1.0).sin().abs() * 4294967296.0) as u32)
        .collect();
    let mut msg = data.to_vec();
    let bits = (data.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bits.to_le_bytes());
    let (mut a0, mut b0, mut c0, mut d0) =
        (0x67452301u32, 0xefcdab89u32, 0x98badcfeu32, 0x10325476u32);
    for chunk in msg.chunks_exact(64) {
        let m: Vec<u32> = chunk
            .chunks_exact(4)
            .map(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
            .collect();
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

/// Argo's JdomParser preserves Java UTF-16 strings and numeric token text.
mod json {
    pub enum Value {
        Object(Vec<(Vec<u16>, Value)>),
        Array(Vec<Value>),
        String(Vec<u16>),
        Number(Vec<u16>),
        Other,
    }
    impl Value {
        pub fn get(&self, key: &str) -> Option<&Value> {
            let key: Vec<_> = key.encode_utf16().collect();
            self.object()?
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v)
        }
        pub fn object(&self) -> Option<&[(Vec<u16>, Value)]> {
            if let Self::Object(o) = self {
                Some(o)
            } else {
                None
            }
        }
        pub fn array(&self) -> Option<&[Value]> {
            if let Self::Array(a) = self {
                Some(a)
            } else {
                None
            }
        }
        pub fn text(&self) -> Option<&[u16]> {
            match self {
                Self::String(t) | Self::Number(t) => Some(t),
                _ => None,
            }
        }
        pub fn string(&self) -> Option<&[u16]> {
            if let Self::String(t) = self {
                Some(t)
            } else {
                None
            }
        }
    }
    fn hash(key: &[u16]) -> i32 {
        key.iter()
            .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(*c as i32))
    }
    fn order(fields: &[(Vec<u16>, Value)], copy: bool) -> Vec<usize> {
        use worldgen::world::java_hash::JavaHashSet;
        let mut set = if copy {
            JavaHashSet::with_capacity((fields.len() as f32 / 0.75 + 1.0) as usize)
        } else {
            JavaHashSet::default()
        };
        let mut keys: Vec<_> = (0..fields.len()).collect();
        keys.sort_by(|&a, &b| fields[a].0.cmp(&fields[b].0));
        let mut ranks = vec![0; fields.len()];
        for (rank, index) in keys.into_iter().enumerate() {
            ranks[index] = rank as i32;
        }
        for (index, (key, _)) in fields.iter().enumerate() {
            // Comparable J_JsonStringNode uses UTF-16 lexical order on equal
            // hashes. Rank supplies exactly that ordering to the existing tree.
            set.insert_with_identity(hash(key), ranks[index], index);
        }
        set.iter().collect()
    }
    /// J_JsonObject.getFields returns a second HashMap copy.
    pub fn first_field(fields: &[(Vec<u16>, Value)]) -> Option<&(Vec<u16>, Value)> {
        order(fields, true).first().map(|&i| &fields[i])
    }
    fn object(fields: Vec<(Vec<u16>, Value)>) -> Value {
        // NodeList uses the default HashMap, then J_JsonObject copies it.
        let initial = order(&fields, false);
        let mut slots: Vec<_> = fields.into_iter().map(Some).collect();
        let fields: Vec<_> = initial
            .into_iter()
            .map(|i| slots[i].take().unwrap())
            .collect();
        let copied = order(&fields, true);
        let mut slots: Vec<_> = fields.into_iter().map(Some).collect();
        Value::Object(
            copied
                .into_iter()
                .map(|i| slots[i].take().unwrap())
                .collect(),
        )
    }
    /// Integer.parseInt accepts a sign and Java Character.digit decimal digits.
    pub fn integer(text: &[u16]) -> Option<i32> {
        let (negative, start) = match text.first()? {
            45 => (true, 1),
            43 => (false, 1),
            _ => (false, 0),
        };
        if start == text.len() {
            return None;
        }
        let mut n = 0i64;
        for &c in &text[start..] {
            let mut digit = None;
            // BMP decimal-digit blocks supported by Java 8 Character (Unicode 6.2).
            for zero in [
                0x30, 0x660, 0x6f0, 0x7c0, 0x966, 0x9e6, 0xa66, 0xae6, 0xb66, 0xbe6, 0xc66, 0xce6,
                0xd66, 0xe50, 0xed0, 0xf20, 0x1040, 0x1090, 0x17e0, 0x1810, 0x1946, 0x19d0, 0x1a80,
                0x1a90, 0x1b50, 0x1bb0, 0x1c40, 0x1c50, 0xa620, 0xa8d0, 0xa900, 0xa9d0, 0xaa50,
                0xabf0, 0xff10,
            ] {
                if c >= zero && c < zero + 10 {
                    digit = Some((c - zero) as i64);
                    break;
                }
            }
            n = n.checked_mul(10)?.checked_add(digit?)?;
            if n > if negative { 2147483648 } else { 2147483647 } {
                return None;
            }
        }
        Some(if negative { -n } else { n } as i32)
    }
    pub fn parse(data: &[u16]) -> Result<Option<Value>, &'static str> {
        // J_SajParser reads its first character before skipping whitespace.
        if !matches!(data.first(), Some(91 | 123)) {
            return Ok(None);
        }
        let mut p = Parser {
            s: data,
            i: 0,
            failure: None,
        };
        let v = p.value();
        if let Some(e) = p.failure {
            return Err(e);
        }
        p.ws();
        Ok(v.filter(|_| p.i == data.len()))
    }
    struct Parser<'a> {
        s: &'a [u16],
        i: usize,
        failure: Option<&'static str>,
    }
    impl Parser<'_> {
        fn ws(&mut self) {
            while matches!(self.s.get(self.i), Some(9 | 10 | 13 | 32)) {
                self.i += 1;
            }
        }
        fn eat(&mut self, c: u16) -> bool {
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
                123 => {
                    self.i += 1;
                    let mut o: Vec<(Vec<u16>, Value)> = Vec::new();
                    if self.eat(125) {
                        return Some(object(o));
                    }
                    loop {
                        self.ws();
                        let k = self.string()?;
                        if !self.eat(58) {
                            return None;
                        }
                        let value = self.value()?;
                        // J_JsonObjectNodeList.put: the last duplicate value wins.
                        if let Some((_, old)) = o.iter_mut().find(|(key, _)| *key == k) {
                            *old = value;
                        } else {
                            o.push((k, value));
                        }
                        if self.eat(125) {
                            return Some(object(o));
                        }
                        if !self.eat(44) {
                            return None;
                        }
                    }
                }
                91 => {
                    self.i += 1;
                    let mut a = Vec::new();
                    if self.eat(93) {
                        return Some(Value::Array(a));
                    }
                    loop {
                        a.push(self.value()?);
                        if self.eat(93) {
                            return Some(Value::Array(a));
                        }
                        if !self.eat(44) {
                            return None;
                        }
                    }
                }
                34 => self.string().map(Value::String),
                45 | 48..=57 => self.number().map(Value::Number),
                _ => {
                    for word in ["true", "false", "null"] {
                        let word: Vec<_> = word.encode_utf16().collect();
                        if self.s[self.i..].starts_with(&word) {
                            self.i += word.len();
                            return Some(Value::Other);
                        }
                    }
                    None
                }
            }
        }
        fn digits(&mut self) {
            while matches!(self.s.get(self.i), Some(48..=57)) {
                self.i += 1;
            }
        }
        fn number(&mut self) -> Option<Vec<u16>> {
            let start = self.i;
            if self.s.get(self.i) == Some(&45) {
                self.i += 1;
            }
            match self.s.get(self.i)? {
                48 => self.i += 1,
                49..=57 => self.digits(),
                _ => return None,
            }
            if self.s.get(self.i) == Some(&46) {
                self.i += 1;
                if !matches!(self.s.get(self.i), Some(48..=57)) {
                    return None;
                }
                self.digits();
            }
            // This Argo version's possibleExponent tests '.' or uppercase 'E';
            // lowercase 'e' is rejected, and a second '.' becomes 'E' in token text.
            let exponent = self.i;
            if matches!(self.s.get(self.i), Some(46 | 69)) {
                self.i += 1;
                if matches!(self.s.get(self.i), Some(43 | 45)) {
                    self.i += 1;
                }
                if !matches!(self.s.get(self.i), Some(48..=57)) {
                    return None;
                }
                self.digits();
            }
            let mut out = self.s[start..self.i].to_vec();
            if exponent < self.i {
                out[exponent - start] = 69;
            }
            Some(out)
        }
        fn string(&mut self) -> Option<Vec<u16>> {
            if self.s.get(self.i) != Some(&34) {
                return None;
            }
            self.i += 1;
            let mut out = Vec::new();
            loop {
                let Some(&c) = self.s.get(self.i) else {
                    self.failure = Some("Nonterminating");
                    return None;
                };
                self.i += 1;
                match c {
                    34 => return Some(out),
                    92 => {
                        let e = *self.s.get(self.i)?;
                        self.i += 1;
                        out.push(match e {
                            34 | 47 | 92 => e,
                            110 => 10,
                            116 => 9,
                            114 => 13,
                            98 => 8,
                            102 => 12,
                            117 => {
                                if self.i == self.s.len() {
                                    self.failure = Some("StringIndexOutOfBoundsException");
                                    return None;
                                }
                                let chars = self.s.get(self.i..self.i + 4)?;
                                let (negative, start) = match chars[0] {
                                    45 => (true, 1),
                                    43 => (false, 1),
                                    _ => (false, 0),
                                };
                                let mut n = 0u16;
                                for &c in &chars[start..] {
                                    let d = match c {
                                        65..=70 => c - 65 + 10,
                                        97..=102 => c - 97 + 10,
                                        0xff21..=0xff26 => c - 0xff21 + 10,
                                        0xff41..=0xff46 => c - 0xff41 + 10,
                                        _ => integer(&[c])? as u16,
                                    };
                                    n = n * 16 + d;
                                }
                                self.i += 4;
                                if negative {
                                    n.wrapping_neg()
                                } else {
                                    n
                                }
                            }
                            _ => return None,
                        });
                    }
                    _ => out.push(c), // Vanilla accepts literal control characters.
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
        assert_eq!(
            super::md5_hex("", "abc"),
            "900150983cd24fb0d6963f7d28e17f72"
        );
    }
}
