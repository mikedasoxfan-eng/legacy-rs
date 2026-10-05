//! The new uncompressed CompressedStreamTools file helpers.
use crate::nbt::{self, Compound};
use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};
fn java_delete(path: &Path) {
    if path.is_dir() {
        let _ = fs::remove_dir(path);
    } else {
        let _ = fs::remove_file(path);
    }
}
/// The original creates GZIPInputStream before entering its try/finally.
/// A constructor/header failure therefore leaves FileInputStream open. On
/// Windows its sharing flags prevent deletion/rename until Java finalizes it.
pub fn read_gzip_file(path: &Path) -> io::Result<Compound> {
    use std::io::Read;
    let mut options = fs::OpenOptions::new();
    options.read(true);
    #[cfg(windows)] {
        use std::os::windows::fs::OpenOptionsExt;
        options.share_mode(3); // FILE_SHARE_READ | FILE_SHARE_WRITE; no DELETE
    }
    let file = options.open(path)?;
    let mut decoder = flate2::read::GzDecoder::new(file);
    if decoder.header().is_none() {
        std::mem::forget(decoder.into_inner());
        return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid gzip header"));
    }
    let mut bytes = Vec::new();
    decoder.read_to_end(&mut bytes)?;
    nbt::read_root(&mut &bytes[..])
}

fn coordinate_parts(name:&str)->Option<(&str,&str)> {
    let s=name.strip_prefix("c.")?.strip_suffix(".dat")?;
    let (x,z)=s.split_once('.')?;
    fn valid(s:&str)->bool {let s=s.strip_prefix('-').unwrap_or(s);!s.is_empty()&&s.bytes().all(|b|b.is_ascii_digit()||b.is_ascii_lowercase())}
    (valid(x)&&valid(z)).then_some((x,z))
}
pub fn chunk_filename_matches(name:&str)->bool {coordinate_parts(name).is_some()}
pub fn chunk_folder_matches(name:&str,is_directory:bool)->bool {
    is_directory&&(1..=2).contains(&name.len())&&name.bytes().all(|b|b.is_ascii_digit()||b.is_ascii_lowercase())
}
/// ChunkFile defaults to (0,0) for unmatched names, but matched overflowing
/// base-36 integers throw NumberFormatException rather than being skipped.
pub struct ChunkFile181 {pub path:PathBuf,pub x:i32,pub z:i32}
impl ChunkFile181 {
    pub fn new(path:PathBuf)->Result<Self,std::num::ParseIntError> {
        let name=path.file_name().and_then(|n|n.to_str()).unwrap_or("");
        let (x,z)=match coordinate_parts(name){Some((x,z))=>(i32::from_str_radix(x,36)?,i32::from_str_radix(z,36)?),None=>(0,0)};
        Ok(Self{path,x,z})
    }
    pub fn compare(&self,other:&Self)->i32 {
        let x=(self.x>>5).wrapping_sub(other.x>>5);
        if x!=0{x}else{(self.z>>5).wrapping_sub(other.z>>5)}
    }
}
#[derive(Clone,Debug)]
pub struct SaveFormatComparator181 {
    pub file_name:String,pub display_name:String,pub last_played:i64,
    pub size_on_disk:i64,pub game_type:i32,pub requires_conversion:bool,
}
impl SaveFormatComparator181 {
    pub fn compare(&self,other:&Self)->i32 {
        if self.last_played<other.last_played{return 1;}
        if self.last_played>other.last_played{return -1;}
        let mut a=self.file_name.encode_utf16();let mut b=other.file_name.encode_utf16();
        loop {match(a.next(),b.next()) {
            (Some(x),Some(y)) if x!=y=>return x as i32-y as i32,
            (Some(_),Some(_))=>(),
            (None,None)=>return 0,
            (Some(_),None)=>return 1+a.count() as i32,
            (None,Some(_))=>return -1-b.count() as i32,
        }}
    }
}

pub fn read_uncompressed(path: &Path) -> io::Result<Option<Compound>> {
    if !path.exists() {
        return Ok(None);
    }
    nbt::read_root(&mut File::open(path)?).map(Some)
}
pub fn write_uncompressed(path: &Path, root: &Compound) -> io::Result<()> {
    nbt::write_root(&mut File::create(path)?, root)
}
/// Vanilla deletes the old target before renaming and ignores rename failure.
pub fn safe_write_uncompressed(path: &Path, root: &Compound) -> io::Result<()> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut tmp = absolute.as_os_str().to_os_string();
    tmp.push("_tmp");
    let tmp = PathBuf::from(tmp);
    if tmp.exists() {
        java_delete(&tmp);
    }
    write_uncompressed(&tmp, root)?;
    if path.exists() {
        java_delete(path);
    }
    if path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to delete {}", path.display()),
        ));
    }
    let _ = fs::rename(tmp, path);
    Ok(())
}
