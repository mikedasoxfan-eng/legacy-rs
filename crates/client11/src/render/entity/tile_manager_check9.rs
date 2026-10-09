//! Original tile-manager bytecode with captured Class identity hash inputs.
use super::{manager::Error, manager_check9::pose, tile_manager::*};
use std::cell::RefCell;
const CLASSES: &[(&str, Option<&str>)] = &[
    ("TileEntity", Some("Object")),
    ("Object", None),
    ("TileEntitySign", Some("TileEntity")),
    ("TileEntityMobSpawner", Some("TileEntity")),
    ("TileEntityPiston", Some("TileEntity")),
    ("TileEntityChest", Some("TileEntity")),
    ("TileEntityFurnace", Some("TileEntity")),
    ("TileEntityDispenser", Some("TileEntity")),
    ("TileEntityNote", Some("TileEntity")),
    ("TileEntityRecordPlayer", Some("TileEntity")),
    ("ChildSign", Some("TileEntitySign")),
    ("GrandSign", Some("SignBase")),
    ("ChildPiston", Some("TileEntityPiston")),
    ("String", Some("Object")),
    ("Runnable", None),
    ("ProbeTile", Some("TileEntitySign")),
    ("Unknown", Some("ProbeTile")),
    ("SignBase", Some("TileEntitySign")),
    // 1.0 registers the enchanting-table book and End portal renderers too.
    ("TileEntityEnchantmentTable", Some("TileEntity")),
    ("TileEntityEndPortal", Some("TileEntity")),
];
fn i(out: &mut Vec<u8>, v: i32) {
    out.extend(v.to_le_bytes());
}
fn f(out: &mut Vec<u8>, v: f32) {
    out.extend(v.to_bits().to_le_bytes());
}
fn d(out: &mut Vec<u8>, v: f64) {
    out.extend(v.to_bits().to_le_bytes());
}
fn status(r: Result<(), Error>) -> i32 {
    match r {
        Ok(()) => 0,
        Err(Error::NullPointer) => 1,
        Err(Error::Callback) => 2,
    }
}
type Manager = TileEntityRenderer<i32, i32, i32, i32>;
fn manager(hashes: &[i32]) -> Manager {
    let hash = |name: &str| hashes[CLASSES.iter().position(|c| c.0 == name).unwrap()];
    let mut m = TileEntityRenderer::new(hash);
    for &(name, parent) in CLASSES {
        m.declare_class(name, parent, hash(name));
    }
    m
}
fn tile(class: &str) -> Tile<'_> {
    Tile {
        class,
        position: [-17, 127, 16],
    }
}
fn lookup(m: &mut Manager, class: Option<&str>, out: &mut Vec<u8>) {
    i(out, m.cached_count() as i32);
    i(out, class.is_some_and(|c| m.is_cached(c)) as i32);
    let r = m.get_special_renderer_for_class(class);
    i(out, r.is_err() as i32);
    i(out, r.ok().flatten().map_or(-1, |v| v as i32));
    i(out, m.cached_count() as i32);
    i(out, class.is_some_and(|c| m.is_cached(c)) as i32);
}
fn registry(hashes: &[i32], out: &mut Vec<u8>) {
    let mut m = manager(hashes);
    i(out, m.cached_count() as i32);
    // The fixture inspects the four Beta-era renderers; 1.0 adds two more to the map.
    for (j, &(_, name)) in RENDERERS.iter().take(4).enumerate() {
        i(out, name.len() as i32);
        out.extend(name.as_bytes());
        i(out, 1);
        for k in 0..j {
            i(out, (j == k) as i32);
        }
    }
    for c in [
        Some("TileEntity"),
        Some("TileEntityFurnace"),
        Some("TileEntityDispenser"),
        Some("TileEntityNote"),
        Some("TileEntityRecordPlayer"),
        Some("ChildSign"),
        Some("GrandSign"),
        Some("ChildPiston"),
        Some("Object"),
        Some("String"),
        Some("Runnable"),
        None,
    ] {
        lookup(&mut m, c, out);
        lookup(&mut m, c, out);
    }
    i(
        out,
        m.get_special_renderer_for_entity(None).unwrap().is_none() as i32,
    );
    i(out, m.has_special_renderer(None).unwrap() as i32);
    for x in [-64.0, -0.5, 0.0, 0.5, 63.5, 64.0] {
        d(out, tile("TileEntitySign").distance_from([x, -0.25, 16.5]));
    }
}
fn probes(hashes: &[i32]) -> Manager {
    let mut m = manager(hashes);
    for class in ["ChildSign", "GrandSign", "ChildPiston", "TileEntityFurnace"] {
        m.get_special_renderer_for_class(Some(class)).unwrap();
    }
    m
}
fn state(m: &Manager, out: &mut Vec<u8>) {
    for v in [
        m.world,
        m.resources.render_engine,
        m.get_font_renderer().copied(),
        m.player,
    ] {
        i(out, v.unwrap_or(0));
    }
    for v in m.view {
        f(out, v);
    }
    for v in m.camera {
        d(out, v);
    }
}
fn notify(out: &mut Vec<u8>, slot: usize, world: Option<&i32>, fault: i32) -> Result<(), Error> {
    // The book and End portal renderers keep TileEntitySpecialRenderer's empty onWorldChange.
    if slot >= 4 {
        return Ok(());
    }
    i(out, 30);
    i(out, slot as i32);
    i(out, world.copied().unwrap_or(0));
    if fault == slot as i32 {
        Err(Error::Callback)
    } else {
        Ok(())
    }
}
fn cache(hashes: &[i32], out: &mut Vec<u8>) {
    let mut m = probes(hashes);
    for scene in 0..3 {
        for resources in 0..4 {
            for player in 0..3 {
                for partial in [-0.25f32, 0.0, 0.375, 1.0, 1.25] {
                    for fault in -1..2 {
                        let mut camera = pose(resources);
                        camera.sleeping = player == 1;
                        let r = m.cache_active_render_info(
                            (scene != 0).then_some(scene),
                            (resources & 1 != 0).then_some(1),
                            (resources & 2 != 0).then_some(1),
                            (player != 2).then_some(1),
                            (player != 2).then_some(camera),
                            partial,
                            |id, world| notify(out, id, world, fault),
                        );
                        i(out, status(r));
                        state(&m, out);
                        let r = m.set_world((resources % 2 == 0).then_some(2), |id, world| {
                            notify(out, id, world, fault)
                        });
                        i(out, status(r));
                        state(&m, out);
                    }
                }
            }
        }
    }
}
#[derive(Clone, Copy)]
struct Gl {
    light: [f32; 2],
    color: [f32; 4],
}
impl Default for Gl {
    fn default() -> Self {
        Self {
            light: [123.25, -456.5],
            color: [0.25, 0.5, 0.75, 0.125],
        }
    }
}
fn gl(out: &mut Vec<u8>, state: Gl) {
    for v in state.light.into_iter().chain(state.color) {
        f(out, v);
    }
}
fn event(out: &mut Vec<u8>, state: &mut Gl, fault: i32, e: Event) -> Result<(), Error> {
    match e {
        Event::Lightmap(light) => state.light = light,
        Event::White => state.color = [1.0; 4],
        Event::Render(id, args) => {
            i(out, 42);
            i(out, id as i32);
            for v in args.position {
                d(out, v);
            }
            f(out, args.partial);
            gl(out, *state);
            if fault == 2 && id == 0 {
                return Err(Error::Callback);
            }
        }
    }
    Ok(())
}
fn dispatch(hashes: &[i32], out: &mut Vec<u8>) {
    for kind in 0..3 {
        for world in [false, true] {
            for fault in 0..3 {
                for distance in [-1.0, 0.0, 4095.999, 4096.0, f64::NAN, f64::INFINITY] {
                    for light in [0, -1, i32::MIN, i32::MAX, 15728880] {
                        let mut m = probes(hashes);
                        m.world = world.then_some(1);
                        m.camera = [-3.5, 68.0, 4.5];
                        m.render_origin = [19.125, -3.25, 7.75];
                        let t = match kind {
                            0 => Some(tile("ProbeTile")),
                            1 => Some(tile("Unknown")),
                            _ => None,
                        };
                        if kind == 1 {
                            for name in ["Unknown", "ProbeTile", "TileEntitySign"] {
                                m.set_class_renderer(name, None);
                            }
                        }
                        let output = RefCell::new(&mut *out);
                        let state = RefCell::new(Gl::default());
                        let r = m.render_tile_entity(
                            t,
                            0.375,
                            |camera| {
                                let mut o = output.borrow_mut();
                                i(&mut o, 40);
                                for v in camera {
                                    d(&mut o, v);
                                }
                                Ok(distance)
                            },
                            |world, xyz, min| {
                                let world = world.ok_or(Error::NullPointer)?;
                                let mut o = output.borrow_mut();
                                i(&mut o, 41);
                                i(&mut o, *world);
                                for v in xyz {
                                    i(&mut o, v);
                                }
                                i(&mut o, min);
                                if fault == 1 {
                                    Err(Error::Callback)
                                } else {
                                    Ok(light)
                                }
                            },
                            |e| event(&mut output.borrow_mut(), &mut state.borrow_mut(), fault, e),
                        );
                        i(&mut output.borrow_mut(), status(r));
                        gl(&mut output.borrow_mut(), *state.borrow());
                        *state.borrow_mut() = Gl::default();
                        let r = m.render_tile_entity_at(
                            t,
                            Arguments {
                                position: [-0.25, 10.5, 3.75],
                                partial: 0.375,
                            },
                            |e| event(&mut output.borrow_mut(), &mut state.borrow_mut(), fault, e),
                        );
                        i(&mut output.borrow_mut(), status(r));
                        gl(&mut output.borrow_mut(), *state.borrow());
                    }
                }
            }
        }
    }
}
pub fn check(path: &str) -> bool {
    let expected = std::fs::read(path).unwrap();
    assert_eq!(&expected[..4], &0x39383154i32.to_le_bytes());
    let count = i32::from_le_bytes(expected[4..8].try_into().unwrap()) as usize;
    assert_eq!(count, CLASSES.len());
    let hashes: Vec<_> = expected[8..8 + count * 4]
        .chunks_exact(4)
        .map(|v| i32::from_le_bytes(v.try_into().unwrap()))
        .collect();
    let mut actual = Vec::new();
    i(&mut actual, 0x39383154);
    i(&mut actual, count as i32);
    for &hash in &hashes {
        i(&mut actual, hash);
    }
    registry(&hashes, &mut actual);
    cache(&hashes, &mut actual);
    dispatch(&hashes, &mut actual);
    if actual != expected {
        let at = actual
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(actual.len().min(expected.len()));
        std::fs::write(
            std::path::Path::new(path).with_extension("rust.bin"),
            &actual,
        )
        .unwrap();
        eprintln!(
            "DIFFER: TileEntityRenderer byte {at}, Rust {} bytes, Java {} bytes",
            actual.len(),
            expected.len()
        );
        return false;
    }
    println!("EXACT: TileEntityRenderer (540 camera caches, 1080 dispatch calls, {} bytes; JVM Class hashes as external inputs)", actual.len());
    true
}
