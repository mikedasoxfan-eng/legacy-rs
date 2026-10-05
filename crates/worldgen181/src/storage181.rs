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
