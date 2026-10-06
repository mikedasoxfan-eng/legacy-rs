//! Named Binary Tag format as b1.7.3 reads and writes it (big-endian, 10 tag types).

use std::io::{self, Read, Write};

/// NBTBase's named wrapper. None represents NBTTagEnd (no payload/name on disk).
#[derive(Clone, Debug, PartialEq)]
pub struct NamedTag10 {
    pub key: String,
    pub payload: Option<Tag>,
}
impl NamedTag10 {
    pub fn new(key: Option<&str>, payload: Option<Tag>) -> Self {
        Self { key: key.unwrap_or("").to_owned(), payload }
    }
    pub fn set_key(&mut self, key: Option<&str>) { self.key = key.unwrap_or("").to_owned(); }
    pub fn id(&self) -> u8 { self.payload.as_ref().map_or(0, Tag::id) }
    pub fn copy10(&self) -> Self {
        if let Some(tag) = &self.payload { Self::new(Some(&self.key), Some(tag.copy10())) }
        else { Self::new(None, None) }
    }
    pub fn create(id: u8, key: Option<&str>) -> Option<Self> {
        let payload = match id {
            0 => return Some(Self::new(None, None)),
            1 => Tag::Byte(0), 2 => Tag::Short(0), 3 => Tag::Int(0), 4 => Tag::Long(0),
            5 => Tag::Float(0.0), 6 => Tag::Double(0.0), 7 => Tag::ByteArray(Vec::new()),
            8 => Tag::String(String::new()), 9 => Tag::List(0, Vec::new()),
            10 => Tag::Compound(Compound::new()), _ => return None,
        };
        Some(Self::new(key, Some(payload)))
    }
    pub fn read<R: Read>(r: &mut R) -> io::Result<Self> {
        let id = read_n::<1, _>(r)?[0];
        if id == 0 { return Ok(Self::new(None, None)); }
        let key = read_utf(r)?;
        let tag = Tag::read_payload(id, r, 0)?;
        Ok(Self::new(Some(&key), Some(tag)))
    }
    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        if let Some(tag) = &self.payload { write_named(w, &self.key, tag) }
        else { w.write_all(&[0]) }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    /// Element type id and elements.
    List(u8, Vec<Tag>),
    Compound(Compound),
}

/// Key/value pairs in insertion order; serialized in Java HashMap iteration order.
#[derive(Clone, Debug, Default)]
pub struct Compound(pub Vec<(String, Tag)>);

// NBTTagCompound.equals compares entry sets, independently of insertion order.
impl PartialEq for Compound {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len() && self.0.iter().all(|(key, value)| other.get(key) == Some(value))
    }
}

impl Compound {
    /// NBTTagCompound.func_40195_b inserts recursively copied tags in Java key-set order.
    pub fn copy10(&self) -> Self {
        let mut result = Self::new();
        for i in self.java_order() {
            let (key, value) = &self.0[i];
            result.put(key, value.copy10());
        }
        result
    }
    pub fn java_order(&self) -> Vec<usize> {
        let mut sorted: Vec<usize> = (0..self.0.len()).collect();
        sorted.sort_by(|&a, &b| self.0[a].0.encode_utf16().cmp(self.0[b].0.encode_utf16()));
        let mut ranks = vec![0; sorted.len()];
        for (rank, &i) in sorted.iter().enumerate() {
            ranks[i] = rank;
        }
        let mut order = crate::structures::long_map_order::LongMapOrder::default();
        for (i, (key, _)) in self.0.iter().enumerate() {
            let hash = key
                .encode_utf16()
                .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32));
            order.insert(hash, ranks[i]);
        }
        order.iter().map(|rank| sorted[rank]).collect()
    }
    pub fn new() -> Self {
        Compound(Vec::new())
    }
    pub fn put(&mut self, key: &str, tag: Tag) {
        if let Some(e) = self.0.iter_mut().find(|e| e.0 == key) {
            e.1 = tag;
        } else {
            self.0.push((key.to_string(), tag));
        }
    }
    pub fn get(&self, key: &str) -> Option<&Tag> {
        self.0.iter().find(|e| e.0 == key).map(|e| &e.1)
    }
    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
    // Getters with the original's defaults for missing keys.
    pub fn byte(&self, key: &str) -> i8 {
        match self.get(key) {
            Some(Tag::Byte(v)) => *v,
            _ => 0,
        }
    }
    pub fn boolean(&self, key: &str) -> bool {
        self.byte(key) != 0
    }
    pub fn short(&self, key: &str) -> i16 {
        match self.get(key) {
            Some(Tag::Short(v)) => *v,
            _ => 0,
        }
    }
    pub fn int(&self, key: &str) -> i32 {
        match self.get(key) {
            Some(Tag::Int(v)) => *v,
            _ => 0,
        }
    }
    pub fn long(&self, key: &str) -> i64 {
        match self.get(key) {
            Some(Tag::Long(v)) => *v,
            _ => 0,
        }
    }
    pub fn string(&self, key: &str) -> String {
        match self.get(key) {
            Some(Tag::String(v)) => v.clone(),
            _ => String::new(),
        }
    }
    pub fn bytes(&self, key: &str) -> Option<&[u8]> {
        match self.get(key) {
            Some(Tag::ByteArray(v)) => Some(v),
            _ => None,
        }
    }
    pub fn compound(&self, key: &str) -> Option<&Compound> {
        match self.get(key) {
            Some(Tag::Compound(c)) => Some(c),
            _ => None,
        }
    }
    pub fn list(&self, key: &str) -> &[Tag] {
        match self.get(key) {
            Some(Tag::List(_, v)) => v,
            _ => &[],
        }
    }
}

impl Tag {
    /// NBTBase.func_40195_b: lists and byte arrays are independent deep copies.
    pub fn copy10(&self) -> Self {
        match self {
            Self::Compound(c) => Self::Compound(c.copy10()),
            Self::List(ty, items) => Self::List(*ty, items.iter().map(Self::copy10).collect()),
            _ => self.clone(),
        }
    }
    pub fn id(&self) -> u8 {
        match self {
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::String(_) => 8,
            Tag::List(..) => 9,
            Tag::Compound(_) => 10,
        }
    }

    fn write_payload<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            Tag::Byte(v) => w.write_all(&[*v as u8]),
            Tag::Short(v) => w.write_all(&v.to_be_bytes()),
            Tag::Int(v) => w.write_all(&v.to_be_bytes()),
            Tag::Long(v) => w.write_all(&v.to_be_bytes()),
            Tag::Float(v) => w.write_all(&(if v.is_nan() { 0x7fc00000u32 } else { v.to_bits() }).to_be_bytes()),
            Tag::Double(v) => w.write_all(&(if v.is_nan() { 0x7ff8000000000000u64 } else { v.to_bits() }).to_be_bytes()),
            Tag::ByteArray(v) => {
                w.write_all(&(v.len() as i32).to_be_bytes())?;
                w.write_all(v)
            }
            Tag::String(s) => write_utf(w, s),
            Tag::List(_, items) => {
                // NBTTagList writes the type of its first element (1 = byte when empty).
                let ty = items
                    .first()
                    .map(Tag::id)
                    .unwrap_or(1);
                w.write_all(&[ty])?;
                w.write_all(&(items.len() as i32).to_be_bytes())?;
                for t in items {
                    t.write_payload(w)?;
                }
                Ok(())
            }
            Tag::Compound(c) => {
                for i in c.java_order() {
                    let (k, t) = &c.0[i];
                    write_named(w, k, t)?;
                }
                w.write_all(&[0])
            }
        }
    }

    fn read_payload<R: Read>(id: u8, r: &mut R, depth: u32) -> io::Result<Tag> {
        if depth > 512 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "NBT nested too deep",
            ));
        }
        Ok(match id {
            1 => Tag::Byte(read_n::<1, _>(r)?[0] as i8),
            2 => Tag::Short(i16::from_be_bytes(read_n(r)?)),
            3 => Tag::Int(i32::from_be_bytes(read_n(r)?)),
            4 => Tag::Long(i64::from_be_bytes(read_n(r)?)),
            5 => Tag::Float(f32::from_be_bytes(read_n(r)?)),
            6 => Tag::Double(f64::from_be_bytes(read_n(r)?)),
            7 => {
                let n = i32::from_be_bytes(read_n(r)?);
                if n < 0 { return Err(io::Error::new(io::ErrorKind::InvalidData,"negative byte-array length")); }
                let n = n as usize;
                let mut v = vec![0; n];
                r.read_exact(&mut v)?;
                Tag::ByteArray(v)
            }
            8 => Tag::String(read_utf(r)?),
            9 => {
                let ty = read_n::<1, _>(r)?[0];
                let n = i32::from_be_bytes(read_n(r)?).max(0) as usize;
                let mut items = Vec::with_capacity(n.min(4096));
                for _ in 0..n {
                    items.push(Tag::read_payload(ty, r, depth + 1)?);
                }
                Tag::List(ty, items)
            }
            10 => {
                let mut c = Compound::new();
                loop {
                    let ty = read_n::<1, _>(r)?[0];
                    if ty == 0 {
                        break;
                    }
                    let name = read_utf(r)?;
                    let t = Tag::read_payload(ty, r, depth + 1)?;
                    c.put(&name, t);
                }
                Tag::Compound(c)
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("bad NBT tag type {id}"),
                ))
            }
        })
    }
}

fn read_n<const N: usize, R: Read>(r: &mut R) -> io::Result<[u8; N]> {
    let mut b = [0u8; N];
    r.read_exact(&mut b)?;
    Ok(b)
}

/// DataOutputStream.writeUTF encodes UTF-16 units, including NUL as C0 80.
pub(crate) fn write_utf<W: Write>(w: &mut W, s: &str) -> io::Result<()> {
    let mut bytes=Vec::new();
    for unit in s.encode_utf16() {
        if (1..=127).contains(&unit) { bytes.push(unit as u8); }
        else if unit>2047 { bytes.extend([(0xe0|(unit>>12)) as u8,(0x80|((unit>>6)&63)) as u8,(0x80|(unit&63)) as u8]); }
        else { bytes.extend([(0xc0|(unit>>6)) as u8,(0x80|(unit&63)) as u8]); }
    }
    if bytes.len()>65535 {return Err(io::Error::new(io::ErrorKind::InvalidInput,"UTF encoding exceeds 65535 bytes"));}
    w.write_all(&(bytes.len() as u16).to_be_bytes())?;
    w.write_all(&bytes)
}

fn read_utf<R: Read>(r: &mut R) -> io::Result<String> {
    let n = u16::from_be_bytes(read_n(r)?) as usize;
    let mut v = vec![0; n];
    r.read_exact(&mut v)?;
    let mut units=Vec::new();let mut i=0;
    while i<n {
        let c=v[i];
        match c>>4 {
            0..=7=>{units.push(c as u16);i+=1;},
            12|13 if i+1<n&&v[i+1]&0xc0==0x80=>{
                units.push(((c as u16&31)<<6)|(v[i+1] as u16&63));i+=2;
            },
            14 if i+2<n&&v[i+1]&0xc0==0x80&&v[i+2]&0xc0==0x80=>{
                units.push(((c as u16&15)<<12)|((v[i+1] as u16&63)<<6)|(v[i+2] as u16&63));i+=3;
            },
            _=>return Err(io::Error::new(io::ErrorKind::InvalidData,"malformed modified UTF-8")),
        }
    }
    String::from_utf16(&units).map_err(|_|io::Error::new(io::ErrorKind::InvalidData,"unpaired UTF-16 surrogate"))
}

fn write_named<W: Write>(w: &mut W, name: &str, t: &Tag) -> io::Result<()> {
    w.write_all(&[t.id()])?;
    write_utf(w, name)?;
    t.write_payload(w)
}

/// `NBTBase.writeTag` for a root compound (named "").
pub fn write_root<W: Write>(w: &mut W, root: &Compound) -> io::Result<()> {
    write_named(w, "", &Tag::Compound(root.clone()))
}

/// Canonical comparison fixtures sort compound names instead of Java HashMap order.
/// Production saves continue to use write_root.
pub fn write_root_canonical<W: Write>(w: &mut W, root: &Compound) -> io::Result<()> {
    fn payload<W: Write>(w: &mut W, tag: &Tag) -> io::Result<()> {
        match tag {
            Tag::Compound(compound) => {
                let mut entries: Vec<_> = compound.0.iter().collect();
                entries.sort_by(|a, b| a.0.encode_utf16().cmp(b.0.encode_utf16()));
                for (name, value) in entries {
                    w.write_all(&[value.id()])?;
                    write_utf(w, name)?;
                    payload(w, value)?;
                }
                w.write_all(&[0])
            }
            Tag::List(_, items) => {
                w.write_all(&[items.first().map_or(1, Tag::id)])?;
                w.write_all(&(items.len() as i32).to_be_bytes())?;
                for value in items { payload(w, value)?; }
                Ok(())
            }
            _ => tag.write_payload(w),
        }
    }
    w.write_all(&[10, 0, 0])?;
    payload(w, &Tag::Compound(root.clone()))
}

/// `CompressedStreamTools.func_1141_a`: the root must be a compound.
pub fn read_root<R: Read>(r: &mut R) -> io::Result<Compound> {
    // CompressedStreamTools reads the entire named tag before checking its type.
    // This preserves both stream position and EOF precedence for non-compounds.
    match NamedTag10::read(r)?.payload {
        Some(Tag::Compound(c)) => Ok(c),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Root tag must be a named compound tag",
        )),
    }
}

pub fn read_gzip(bytes: &[u8]) -> io::Result<Compound> {
    read_root(&mut flate2::read::GzDecoder::new(bytes))
}

pub fn write_gzip(root: &Compound) -> io::Result<Vec<u8>> {
    let mut e = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    write_root(&mut e, root)?;
    e.finish()
}

#[cfg(test)]
mod tests10 {
    use super::*;
    #[test]
    fn root_type_rejection_consumes_payload_and_preserves_eof_precedence() {
        let mut bytes = &b"\x03\x00\x01x\x00\x00\x00\x07\xaa"[..];
        let error = read_root(&mut bytes).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(error.to_string(), "Root tag must be a named compound tag");
        assert_eq!(bytes, [0xaa]);
        let error = read_root(&mut &b"\x03\x00\x01x\x00"[..]).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::UnexpectedEof);
    }
    #[test]
    fn modified_utf_uses_nul_and_surrogate_pairs_and_rejects_bad_sequences() {
        let mut bytes=vec![];write_utf(&mut bytes,"a\0é😀").unwrap();
        assert_eq!(bytes, [0,11,97,192,128,195,169,237,160,189,237,184,128]);
        assert_eq!(read_utf(&mut &bytes[..]).unwrap(),"a\0é😀");
        assert!(read_utf(&mut &[0,2,192,0][..]).is_err());
        assert!(write_utf(&mut vec![],&"\0".repeat(32768)).is_err());
    }
    #[test]
    fn compound_equality_ignores_insertion_order_and_copies_are_independent() {
        let mut a=Compound::new();a.put("a",Tag::Int(1));a.put("b",Tag::ByteArray(vec![2]));
        let mut b=Compound::new();b.put("b",Tag::ByteArray(vec![2]));b.put("a",Tag::Int(1));
        assert_eq!(a,b);
        let mut copy=a.copy10();copy.put("b",Tag::ByteArray(vec![3]));assert_ne!(a,copy);
        let mut bytes=vec![];Tag::Float(f32::from_bits(0x7f800001)).write_payload(&mut bytes).unwrap();
        assert_eq!(bytes,0x7fc00000u32.to_be_bytes());
    }
}
