//! Original RenderEngine observations use the production cache state machine.
use super::render_engine::{argb, Backend, Engine, Error};
use crate::render::gpu::Image;
use std::sync::{Arc, Mutex};
fn hash(values: &[i32]) -> i32 {
    values
        .iter()
        .fold(1i32, |h, &v| h.wrapping_mul(31).wrapping_add(v))
}
fn flag(v: bool) -> char {
    if v {
        '1'
    } else {
        '0'
    }
}
fn image(w: u32, h: u32, seed: i32) -> Image {
    let rgba = (0..w * h)
        .flat_map(|i| {
            let v = seed.wrapping_add(i as i32).wrapping_mul(3571) & 0xffffff;
            [(v >> 16) as u8, (v >> 8) as u8, v as u8, 255]
        })
        .collect();
    Image {
        width: w,
        height: h,
        rgba,
    }
}
struct Recorder {
    next: i32,
    fail: i32,
    events: Arc<Mutex<String>>,
}
impl Backend for Recorder {
    fn allocate(&mut self) -> i32 {
        let id = self.next;
        self.next += 1;
        id
    }
    fn setup(&mut self, image: &Image, id: i32, blur: bool, clamp: bool) -> Result<(), Error> {
        self.events.lock().unwrap().push_str(&format!(
            "U:{id}:{}x{}:{}:{}{};",
            image.width,
            image.height,
            hash(&argb(image)),
            flag(blur),
            flag(clamp)
        ));
        if self.fail > 0 {
            self.fail -= 1;
            Err("IllegalStateException")
        } else {
            Ok(())
        }
    }
    fn delete(&mut self, _: i32) {}
}
fn read(path: &str, behavior: i32, w: u32, h: u32, seed: i32) -> Result<Option<Image>, Error> {
    let _ = path;
    match behavior {
        1 => Ok(None),
        5 => Err("IOException"),
        2 | 3 => Err("NullPointerException"),
        4 => Err("IllegalStateException"),
        _ => Ok(Some(image(w, h, seed))),
    }
}
fn observed_read(
    events: &Arc<Mutex<String>>,
    blur: bool,
    clamp: bool,
    behavior: i32,
    w: u32,
    h: u32,
    seed: i32,
    path: &str,
) -> Result<Option<Image>, Error> {
    events
        .lock()
        .unwrap()
        .push_str(&format!("R:{path}:{}{};", flag(blur), flag(clamp)));
    read(path, behavior, w, h, seed)
}
struct Input<'a>(&'a [u8]);
impl Input<'_> {
    fn int(&mut self) -> i32 {
        let n = i32::from_be_bytes(self.0[..4].try_into().unwrap());
        self.0 = &self.0[4..];
        n
    }
    fn flag(&mut self) -> bool {
        let b = self.0[0] != 0;
        self.0 = &self.0[1..];
        b
    }
    fn text(&mut self) -> String {
        let n = self.int() as usize;
        let s = String::from_utf8(self.0[..n].to_vec()).unwrap();
        self.0 = &self.0[n..];
        s
    }
}
fn diff(label: &str, actual: &str, expected: &str, failures: &mut i32) {
    if actual != expected {
        if *failures < 12 {
            eprintln!("RENDERENGINE11 {label}: actual {actual:?}, expected {expected:?}");
        }
        *failures += 1;
    }
}
pub fn check(path: &str) -> bool {
    let bytes = std::fs::read(path).unwrap();
    let mut input = Input(&bytes);
    let missing = image(2, 2, 99);
    let mut failures = 0;
    let count = input.int();
    for c in 0..count {
        let behavior = input.int();
        let blur = input.flag();
        let clamp = input.flag();
        let contents = input.flag();
        let name = input.text();
        let fail = input.int();
        let expected = input.text();
        let events = Arc::new(Mutex::new(String::new()));
        let mut b = Recorder {
            next: 1,
            fail,
            events: events.clone(),
        };
        let mut e = Engine::default();
        e.blur = blur;
        e.clamp = clamp;
        let result = if contents {
            match e.get_contents(&name, &missing, |p, rb, rc| {
                observed_read(&events, rb, rc, behavior, 32, 3, 7, p)
            }) {
                Ok(first) => {
                    first.lock().unwrap()[0] = 123;
                    let again = e.get_contents(&name, &missing, |_, _, _| panic!("cache reread"));
                    format!(
                        "C:{}:{}",
                        Arc::ptr_eq(&first, &again.unwrap()),
                        hash(&first.lock().unwrap())
                    )
                }
                Err(ex) => format!("E:{ex}"),
            }
        } else {
            match e.get_texture(&mut b, &name, &missing, |p, rb, rc| {
                observed_read(&events, rb, rc, behavior, 32, 3, 7, p)
            }) {
                Ok(id) => {
                    let again = e
                        .get_texture(&mut b, &name, &missing, |_, _, _| panic!("cache reread"))
                        .unwrap();
                    format!("T:{id}:{}", id == again)
                }
                Err(ex) => format!("E:{ex}"),
            }
        };
        let actual = format!(
            "{}{result}:{}{}",
            events.lock().unwrap(),
            flag(e.blur),
            flag(e.clamp)
        );
        diff(&format!("API {c}"), &actual, &expected, &mut failures);
    }
    let expected = input.text();
    let events = Arc::new(Mutex::new(String::new()));
    let mut b = Recorder {
        next: 1,
        fail: 0,
        events: events.clone(),
    };
    let mut e = Engine::default();
    let mut images = Vec::new();
    let mut ids = Vec::new();
    for i in 0..180 {
        let im = Arc::new(Mutex::new(image(2, 2, i)));
        ids.push(e.allocate(&mut b, im.clone()).unwrap());
        images.push(im);
    }
    for i in (0..180).step_by(3) {
        e.delete(&mut b, ids[i]);
    }
    for (i, im) in images.iter().enumerate() {
        let v = 0xffabcdefu32 + i as u32;
        im.lock().unwrap().rgba[..4].copy_from_slice(&[
            (v >> 16) as u8,
            (v >> 8) as u8,
            v as u8,
            255,
        ]);
    }
    e.blur = true;
    e.clamp = true;
    e.refresh(&mut b, &missing, |_, _, _| panic!("dynamic reread"))
        .unwrap();
    diff("dynamic", &events.lock().unwrap(), &expected, &mut failures);
    let expected = input.text();
    let events = Arc::new(Mutex::new(String::new()));
    let mut b = Recorder {
        next: 1,
        fail: 0,
        events: events.clone(),
    };
    let mut e = Engine::default();
    let mut keys = Vec::new();
    for i in 0..64 {
        let h = (i + 1) * 1024;
        let key = h ^ (h >> 16);
        keys.push(key);
        b.next = key;
        e.allocate(&mut b, Arc::new(Mutex::new(image(2, 2, i))))
            .unwrap();
    }
    for t in 0..64 {
        let i = (t * 37 % 64) as usize;
        e.delete(&mut b, keys[i]);
        if t % 5 == 0 {
            b.next = keys[i];
            e.allocate(&mut b, Arc::new(Mutex::new(image(2, 2, 1000 + t))))
                .unwrap();
        }
        e.blur = t % 2 != 0;
        e.clamp = t % 3 == 0;
        e.refresh(&mut b, &missing, |_, _, _| panic!("dynamic reread"))
            .unwrap();
    }
    diff(
        "dynamic tree deletion",
        &events.lock().unwrap(),
        &expected,
        &mut failures,
    );
    let prefixes = [
        "",
        "##",
        "%clamp%",
        "%blur%",
        "%blurclamp%",
        "%blur%%clamp%",
        "%clamp%%blur%",
    ];
    let refresh = input.int();
    for _ in 0..refresh {
        let c = input.int();
        let expected = input.text();
        let events = Arc::new(Mutex::new(String::new()));
        let mut b = Recorder {
            next: 1,
            fail: 0,
            events: events.clone(),
        };
        let mut e = Engine::default();
        let name = format!("{}/cached.png", prefixes[c as usize % 7]);
        let cache = e
            .get_contents(&name, &missing, |p, rb, rc| {
                observed_read(&events, rb, rc, 0, 32, 3, 7, p)
            })
            .unwrap();
        let n = cache.lock().unwrap().len();
        cache.lock().unwrap()[n - 1] = 123;
        let w = if c % 4 == 0 {
            16
        } else if c % 4 == 1 {
            32
        } else {
            48
        };
        let h = if c % 4 == 2 { 4 } else { 2 };
        let result = e.refresh(&mut b, &missing, |p, rb, rc| {
            observed_read(&events, rb, rc, c / 4 % 5, w, h, 81, p)
        });
        let status = result.err().unwrap_or("ok");
        let actual = format!(
            "{}{status}:{}:{}{}",
            events.lock().unwrap(),
            hash(&cache.lock().unwrap()),
            flag(e.blur),
            flag(e.clamp)
        );
        diff(&format!("refresh {c}"), &actual, &expected, &mut failures);
    }
    let mixed = input.int();
    for _ in 0..mixed {
        let c = input.int();
        let expected = input.text();
        let events = Arc::new(Mutex::new(String::new()));
        let mut b = Recorder {
            next: 1,
            fail: 0,
            events: events.clone(),
        };
        let mut e = Engine::default();
        let mut caches = Vec::new();
        let mut ids = Vec::new();
        for i in 0..7 {
            let name = format!("{}/mixed{i}.png", prefixes[(i + c as usize) % 7]);
            ids.push(
                e.get_texture(&mut b, &name, &missing, |_, _, _| Ok(Some(image(32, 3, 7))))
                    .unwrap(),
            );
            caches.push(
                e.get_contents(&name, &missing, |_, _, _| Ok(Some(image(32, 3, 7))))
                    .unwrap(),
            );
        }
        for i in 0..3 {
            e.allocate(&mut b, Arc::new(Mutex::new(image(2, 2, i))))
                .unwrap();
        }
        e.delete(&mut b, ids[0]);
        events.lock().unwrap().clear();
        e.blur = c % 2 != 0;
        e.clamp = c % 4 >= 2;
        b.fail = c / 48;
        let w = if c / 24 % 2 == 0 { 16 } else { 32 };
        let result = e.refresh(&mut b, &missing, |p, rb, rc| {
            observed_read(&events, rb, rc, c / 4 % 6, w, 2, 87, p)
        });
        let mut state = String::new();
        for cache in &caches {
            state.push_str(&format!("{},", hash(&cache.lock().unwrap())));
        }
        let name = format!("{}/mixed0.png", prefixes[c as usize % 7]);
        let cached = e
            .get_texture(&mut b, &name, &missing, |_, _, _| {
                panic!("deleted named cache lost")
            })
            .unwrap();
        let actual = format!(
            "{}{}:{}{}:{state}:{}",
            events.lock().unwrap(),
            result.err().unwrap_or("ok"),
            flag(e.blur),
            flag(e.clamp),
            cached == ids[0]
        );
        diff(&format!("mixed {c}"), &actual, &expected, &mut failures);
    }
    let expected = input.text();
    let events = Arc::new(Mutex::new(String::new()));
    let mut b = Recorder {
        next: 1,
        fail: 0,
        events: events.clone(),
    };
    let mut e = Engine::default();
    for i in 0..32 {
        let mut name = String::from("/collision/");
        for bit in 0..5 {
            name.push_str(if i & (1 << bit) == 0 { "Aa" } else { "BB" });
        }
        name.push_str(".png");
        e.get_texture(&mut b, &name, &missing, |_, _, _| Ok(Some(image(32, 3, i))))
            .unwrap();
        e.get_contents(&name, &missing, |_, _, _| Ok(Some(image(32, 3, i))))
            .unwrap();
    }
    events.lock().unwrap().clear();
    e.refresh(&mut b, &missing, |p, rb, rc| {
        observed_read(&events, rb, rc, 0, 32, 3, 200, p)
    })
    .unwrap();
    diff(
        "string tree bins",
        &events.lock().unwrap(),
        &expected,
        &mut failures,
    );
    let buffers = input.int();
    let mut e = Engine::default();
    e.buffer_limit = 16_777_216;
    let events = Arc::new(Mutex::new(String::new()));
    let mut b = Recorder {
        next: 1,
        fail: 0,
        events,
    };
    for _ in 0..buffers {
        let c = input.int();
        let expected = input.text();
        let w = match c % 6 {
            0 => 2048,
            1 => 2049,
            2 => 1024,
            3 => 0,
            4 => -1,
            _ => 4,
        };
        let h = if c % 6 < 3 { 2048 } else { 4 };
        let n = if c % 6 == 5 { 17 } else { 0 };
        let result = if c < 6 {
            if w <= 0 {
                Err("NullPointerException")
            } else {
                e.setup(&mut b, &image(w as u32, h as u32, 3), 900)
            }
        } else {
            e.create_from_bytes(&mut b, &vec![0; n], w, h, 900)
        };
        let actual = format!(
            "{}:{}:{}",
            result.err().unwrap_or("ok"),
            e.buffer_position,
            e.buffer_limit
        );
        diff(&format!("buffer {c}"), &actual, &expected, &mut failures);
    }
    assert!(input.0.is_empty());
    println!("RENDERENGINE11: {count} cache/flag/failure cases, 180 mutable dynamic images plus 64 tree-bin deletion/reuse refreshes, {refresh} contents refresh, {mixed} mixed refresh, 32 colliding String keys, {buffers} buffer boundaries: {failures} differences");
    failures == 0
}
