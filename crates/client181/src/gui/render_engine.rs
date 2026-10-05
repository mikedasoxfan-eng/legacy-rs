//! RenderEngine's caches and failure transactions, independent of the GL upload adapter.
//! Resource decoders return Java exception classes; only IOException is caught by
//! contents/refresh, while getTexture catches every ordinary exception.
use crate::render::gpu::Image;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use worldgen::world::java_hash::JavaHashSet;
pub type Error = &'static str;
pub type SharedImage = Arc<Mutex<Image>>;
pub type Contents = Arc<Mutex<Vec<i32>>>;
pub trait Backend {
    fn allocate(&mut self) -> i32;
    fn setup(&mut self, image: &Image, id: i32, blur: bool, clamp: bool) -> Result<(), Error>;
    fn delete(&mut self, id: i32);
}
fn copy(image: &Image) -> Image {
    Image {
        width: image.width,
        height: image.height,
        rgba: image.rgba.clone(),
    }
}
fn string_hash(s: &str) -> i32 {
    s.encode_utf16()
        .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
}
/// Replaying insertion reproduces HashMap state; these caches never remove keys.
/// Final UTF-16 lexical ranks preserve String.compareTo at every earlier insertion,
/// including tree bins whose keys have identical String.hashCode values.
fn ordered_names(keys: &[String]) -> Vec<String> {
    let mut indices: Vec<_> = (0..keys.len()).collect();
    indices.sort_by_key(|&i| keys[i].encode_utf16().collect::<Vec<_>>());
    let mut ranks = vec![0; keys.len()];
    for (rank, i) in indices.into_iter().enumerate() {
        ranks[i] = rank as i32;
    }
    let mut set = JavaHashSet::default();
    for (i, key) in keys.iter().enumerate() {
        set.insert_with_identity(string_hash(key), ranks[i], i);
    }
    set.iter().map(|i| keys[i].clone()).collect()
}
pub struct Engine {
    pub blur: bool,
    pub clamp: bool,
    pub anaglyph: bool,
    names: HashMap<String, i32>,
    name_keys: Vec<String>,
    contents: HashMap<String, Contents>,
    contents_keys: Vec<String>,
    dynamic: HashMap<i32, SharedImage>,
    // MCHash.func_35860_b is the existing Java HashSet<Integer> implementation.
    dynamic_keys: JavaHashSet<i32>,
    pub buffer_position: usize,
    pub buffer_limit: usize,
}
impl Default for Engine {
    fn default() -> Self {
        Self {
            blur: false,
            clamp: false,
            anaglyph: false,
            names: Default::default(),
            name_keys: Vec::new(),
            contents: Default::default(),
            contents_keys: Vec::new(),
            dynamic: Default::default(),
            dynamic_keys: Default::default(),
            buffer_position: 0,
            buffer_limit: 16_777_216,
        }
    }
}
impl Engine {
    fn flags<'a>(&mut self, name: &'a str, contents: bool, refresh: bool) -> (&'a str, bool) {
        if let Some(path) = name.strip_prefix("##") {
            return (path, true);
        }
        if let Some(path) = name.strip_prefix("%clamp%") {
            self.clamp = true;
            return (path, false);
        }
        if let Some(path) = name.strip_prefix("%blur%") {
            self.blur = true;
            if contents && !refresh {
                self.clamp = true;
            }
            return (path, false);
        }
        if !contents {
            if let Some(path) = name.strip_prefix("%blurclamp%") {
                self.blur = true;
                self.clamp = true;
                return (path, false);
            }
        }
        (name, false)
    }
    fn reset(&mut self, name: &str, contents: bool) {
        if name.starts_with("%clamp%") {
            self.clamp = false;
        } else if name.starts_with("%blur%") {
            self.blur = false;
            if contents {
                self.clamp = false;
            }
        } else if !contents && name.starts_with("%blurclamp%") {
            self.blur = false;
            self.clamp = false;
        }
    }
    fn read(
        &mut self,
        name: &str,
        contents: bool,
        refresh: bool,
        missing: &Image,
        read: &mut impl FnMut(&str, bool, bool) -> Result<Option<Image>, Error>,
    ) -> Result<Image, Error> {
        let (path, unwrap) = self.flags(name, contents, refresh);
        let prefixed = path != name;
        let image = match read(path, self.blur, self.clamp)? {
            Some(i) => i,
            None if !prefixed && !refresh => copy(missing),
            None => return Err("IllegalArgumentException"),
        };
        if unwrap {
            unwrap_columns(&image)
        } else {
            Ok(image)
        }
    }
    pub fn setup(&mut self, b: &mut impl Backend, image: &Image, id: i32) -> Result<(), Error> {
        // ByteBuffer.clear precedes put; overflow retains position=0, limit=capacity.
        self.buffer_position = 0;
        self.buffer_limit = 16_777_216;
        if image.rgba.len() > self.buffer_limit {
            return Err("BufferOverflowException");
        }
        self.buffer_limit = image.rgba.len();
        let mut image = copy(image);
        if self.anaglyph {
            crate::texture_packs::anaglyph(&mut image.rgba);
        }
        b.setup(&image, id, self.blur, self.clamp)
    }
    pub fn get_texture(
        &mut self,
        b: &mut impl Backend,
        name: &str,
        missing: &Image,
        mut read: impl FnMut(&str, bool, bool) -> Result<Option<Image>, Error>,
    ) -> Result<i32, Error> {
        if let Some(&id) = self.names.get(name) {
            return Ok(id);
        }
        let id = b.allocate();
        let result = self
            .read(name, false, false, missing, &mut read)
            .and_then(|image| self.setup(b, &image, id));
        let id = if result.is_ok() {
            self.reset(name, false);
            id
        } else {
            let id = b.allocate();
            self.setup(b, missing, id)?;
            id
        };
        self.name_keys.push(name.into());
        self.names.insert(name.into(), id);
        Ok(id)
    }
    pub fn get_contents(
        &mut self,
        name: &str,
        missing: &Image,
        mut read: impl FnMut(&str, bool, bool) -> Result<Option<Image>, Error>,
    ) -> Result<Contents, Error> {
        if let Some(c) = self.contents.get(name) {
            return Ok(c.clone());
        }
        let image = match self.read(name, true, false, missing, &mut read) {
            Ok(i) => {
                self.reset(name, true);
                i
            }
            Err("IOException") => copy(missing),
            Err(e) => return Err(e),
        };
        let out = Arc::new(Mutex::new(argb(&image)));
        self.contents_keys.push(name.into());
        self.contents.insert(name.into(), out.clone());
        Ok(out)
    }
    pub fn allocate(&mut self, b: &mut impl Backend, image: SharedImage) -> Result<i32, Error> {
        let id = b.allocate();
        self.setup(b, &image.lock().unwrap(), id)?;
        self.dynamic_keys.insert(id, id);
        self.dynamic.insert(id, image);
        Ok(id)
    }
    pub fn update_dynamic(&mut self, id: i32, image: &Image) {
        if let Some(old) = self.dynamic.get(&id) {
            *old.lock().unwrap() = copy(image);
        }
    }
    pub fn create_from_bytes(
        &mut self,
        b: &mut impl Backend,
        pixels: &[i32],
        width: i32,
        height: i32,
        id: i32,
    ) -> Result<(), Error> {
        let length = width.wrapping_mul(height).wrapping_mul(4);
        if length < 0 {
            return Err("NegativeArraySizeException");
        }
        if pixels.len() > length as usize / 4 {
            return Err("ArrayIndexOutOfBoundsException");
        }
        let mut image = Image {
            width: width as u32,
            height: height as u32,
            rgba: vec![0; length as usize],
        };
        for (c, &p) in image.rgba.chunks_exact_mut(4).zip(pixels) {
            c.copy_from_slice(&[(p >> 16) as u8, (p >> 8) as u8, p as u8, (p >> 24) as u8]);
        }
        self.setup(b, &image, id)
    }
    pub fn delete(&mut self, b: &mut impl Backend, id: i32) {
        self.dynamic_keys.remove(id, id);
        self.dynamic.remove(&id);
        b.delete(id);
    }
    pub fn refresh(
        &mut self,
        b: &mut impl Backend,
        missing: &Image,
        mut read: impl FnMut(&str, bool, bool) -> Result<Option<Image>, Error>,
    ) -> Result<(), Error> {
        for id in self.dynamic_keys.iter().collect::<Vec<_>>() {
            let image = self.dynamic[&id].clone();
            self.setup(b, &image.lock().unwrap(), id)?;
        }
        let keys = ordered_names(&self.name_keys);
        for name in keys {
            let result = self
                .read(&name, false, true, missing, &mut read)
                .and_then(|image| self.setup(b, &image, self.names[&name]));
            match result {
                Ok(()) => {
                    self.blur = false;
                    self.clamp = false;
                }
                Err("IOException") => {}
                Err(e) => return Err(e),
            }
        }
        let keys = ordered_names(&self.contents_keys);
        for name in keys {
            match self.read(&name, true, true, missing, &mut read) {
                Ok(image) => {
                    let values = argb(&image);
                    let mut dest = self.contents[&name].lock().unwrap();
                    for (i, v) in values.into_iter().enumerate() {
                        let d = dest.get_mut(i).ok_or("ArrayIndexOutOfBoundsException")?;
                        *d = v;
                    }
                    self.blur = false;
                    self.clamp = false;
                }
                Err("IOException") => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}
pub fn argb(image: &Image) -> Vec<i32> {
    image
        .rgba
        .chunks_exact(4)
        .map(|c| {
            ((c[3] as u32) << 24 | (c[0] as u32) << 16 | (c[1] as u32) << 8 | c[2] as u32) as i32
        })
        .collect()
}
pub fn unwrap_columns(image: &Image) -> Result<Image, Error> {
    let columns = image.width / 16;
    if columns == 0 {
        return Err("IllegalArgumentException");
    }
    let mut out = Image {
        width: 16,
        height: image.height * columns,
        rgba: vec![0; 16 * image.height as usize * columns as usize * 4],
    };
    for column in 0..columns as usize {
        for y in 0..image.height as usize {
            let src = (y * image.width as usize + column * 16) * 4;
            let dst = (column * image.height as usize + y) * 64;
            out.rgba[dst..dst + 64].copy_from_slice(&image.rgba[src..src + 64]);
        }
    }
    Ok(out)
}
