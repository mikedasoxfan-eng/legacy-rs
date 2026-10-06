//! Java Deflater's default zlib algorithm, including Java's literal GZIP header.
//! miniz and zlib-ng produce different bytes even for ordinary game NBT trees.
use std::{io, os::raw::{c_int,c_ulong}, sync::OnceLock};
type Compress = unsafe extern "C" fn(*mut u8,*mut c_ulong,*const u8,c_ulong,c_int)->c_int;
type Bound = unsafe extern "C" fn(c_ulong)->c_ulong;
type Crc = unsafe extern "C" fn(c_ulong,*const u8,u32)->c_ulong;
struct Api { _library:libloading::Library,compress:Compress,bound:Bound,crc:Crc }
static API:OnceLock<Result<Api,String>>=OnceLock::new();
fn api()->io::Result<&'static Api> {
    API.get_or_init(|| {
        #[cfg(all(windows,target_arch="x86_64"))]
        let path={
            // Embed the licensed runtime so a relocated binary also works without Git.
            let bytes=include_bytes!("../../../assets/r10-zlib/zlib1.dll");
            let path=std::env::temp_dir().join("beta-rs-r10-zlib-1.3.1-x64.dll");
            if std::fs::read(&path).ok().as_deref()!=Some(bytes.as_slice()) {
                std::fs::write(&path,bytes).map_err(|e|e.to_string())?;
            }
            path
        };
        #[cfg(all(windows,not(target_arch="x86_64")))]
        let path=std::path::PathBuf::from("zlib1.dll");
        #[cfg(target_os="macos")]
        let path=std::path::PathBuf::from("libz.1.dylib");
        #[cfg(all(not(windows),not(target_os="macos")))]
        let path=std::path::PathBuf::from("libz.so.1");
        // SAFETY: the pinned Windows library (or platform zlib ABI) owns these C
        // functions, and stays loaded for the lifetime of the cached function pointers.
        unsafe {
            let library=libloading::Library::new(path).map_err(|e|e.to_string())?;
            let compress=*library.get::<Compress>(b"compress2\0").map_err(|e|e.to_string())?;
            let bound=*library.get::<Bound>(b"compressBound\0").map_err(|e|e.to_string())?;
            let crc=*library.get::<Crc>(b"crc32\0").map_err(|e|e.to_string())?;
            Ok(Api{_library:library,compress,bound,crc})
        }
    }).as_ref().map_err(|e|io::Error::other(e.clone()))
}
pub fn zlib(raw:&[u8])->io::Result<Vec<u8>> {
    let api=api()?;
    let length:c_ulong=raw.len().try_into().map_err(|_|io::Error::other("zlib input too large"))?;
    // SAFETY: compressBound guarantees output capacity; raw and output do not alias.
    unsafe {
        let mut capacity=(api.bound)(length);let mut out=vec![0;capacity as usize];
        let status=(api.compress)(out.as_mut_ptr(),&mut capacity,raw.as_ptr(),length,-1);
        if status!=0 {return Err(io::Error::other(format!("zlib compress2: {status}")));}
        out.truncate(capacity as usize);Ok(out)
    }
}
pub fn gzip(raw:&[u8])->io::Result<Vec<u8>> {
    let encoded=zlib(raw)?;let api=api()?;
    let length:u32=raw.len().try_into().map_err(|_|io::Error::other("gzip input too large"))?;
    // SAFETY: crc32 reads exactly the live input slice and retains no pointer.
    let crc=unsafe{(api.crc)(0,raw.as_ptr(),length)} as u32;
    let mut out=Vec::with_capacity(encoded.len()+12);
    out.extend([31,139,8,0,0,0,0,0,0,0]);
    out.extend_from_slice(&encoded[2..encoded.len()-4]);
    out.extend(crc.to_le_bytes());out.extend(length.to_le_bytes());Ok(out)
}
