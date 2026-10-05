//! Observable dependency callbacks of the original TileEntitySpecialRenderer.
use super::{manager::Error, tile_special::*};
use std::{cell::RefCell, rc::Rc};
fn i(out: &mut Vec<u8>, v: i32) {
    out.extend(v.to_le_bytes());
}
fn status(result: Result<(), Error>) -> i32 {
    match result {
        Ok(()) => 0,
        Err(Error::NullPointer) => 1,
        Err(Error::Callback) => 2,
    }
}
struct Engine {
    id: i32,
    texture: i32,
    fail: i32,
    output: Rc<RefCell<Vec<u8>>>,
}
impl TextureEngine for Engine {
    fn get_texture(&mut self, name: Option<&str>) -> Result<i32, Error> {
        let mut out = self.output.borrow_mut();
        i(&mut out, 0);
        i(&mut out, self.id);
        if let Some(name) = name {
            i(&mut out, name.len() as i32);
            out.extend(name.as_bytes());
        } else {
            i(&mut out, -1);
        }
        if self.fail == 2 {
            Err(Error::Callback)
        } else {
            Ok(self.texture)
        }
    }
    fn bind_texture(&mut self, texture: i32) -> Result<(), Error> {
        let mut out = self.output.borrow_mut();
        i(&mut out, 1);
        i(&mut out, self.id);
        i(&mut out, texture);
        if self.fail == 3 {
            Err(Error::Callback)
        } else {
            Ok(())
        }
    }
}
pub fn check(path: &str) -> bool {
    let out = Rc::new(RefCell::new(Vec::new()));
    let mut cases = 0;
    for manager in 0..3 {
        for engine in 0..4 {
            for has_font in [false, true] {
                for name in [
                    None,
                    Some(""),
                    Some("/item/chest.png"),
                    Some("/item/largechest.png"),
                    Some("a\0b"),
                    Some("%blur%/terrain.png"),
                ] {
                    for texture in [i32::MIN, -1, 0, 1, 31, i32::MAX] {
                        let mut m1 = Context {
                            render_engine: (engine != 0).then(|| Engine {
                                id: 1,
                                texture,
                                fail: engine,
                                output: out.clone(),
                            }),
                            font_renderer: has_font.then_some(1),
                        };
                        let mut m2 = Context {
                            render_engine: (engine != 0).then(|| Engine {
                                id: 2,
                                texture,
                                fail: engine,
                                output: out.clone(),
                            }),
                            font_renderer: has_font.then_some(2),
                        };
                        let mut second = Some(&mut m2);
                        let mut r = TileEntitySpecialRenderer::new();
                        r.set_tile_entity_renderer(match manager {
                            0 => None,
                            1 => Some(&mut m1),
                            _ => second.take(),
                        });
                        let result = r.bind_texture_by_name(name);
                        i(&mut out.borrow_mut(), status(result));
                        let result = r.get_font_renderer();
                        i(&mut out.borrow_mut(), result.is_err() as i32);
                        i(
                            &mut out.borrow_mut(),
                            result.ok().flatten().copied().unwrap_or(0),
                        );
                        r.world_changed(None::<()>);
                        r.world_changed(Some(()));
                        let saved = r.tile_entity_renderer.take();
                        if manager == 2 {
                            second = saved;
                        }
                        r.set_tile_entity_renderer(None);
                        let result = r.bind_texture_by_name(name);
                        i(&mut out.borrow_mut(), status(result));
                        let result = r.get_font_renderer();
                        i(&mut out.borrow_mut(), result.is_err() as i32);
                        r.set_tile_entity_renderer(second);
                        let result = r.bind_texture_by_name(name);
                        i(&mut out.borrow_mut(), status(result));
                        i(
                            &mut out.borrow_mut(),
                            r.get_font_renderer().unwrap().copied().unwrap_or(0),
                        );
                        cases += 1;
                    }
                }
            }
        }
    }
    let actual = out.borrow();
    let expected = std::fs::read(path).unwrap();
    if *actual != expected {
        let at = actual
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(actual.len().min(expected.len()));
        std::fs::write(
            std::path::Path::new(path).with_extension("rust.bin"),
            &*actual,
        )
        .unwrap();
        eprintln!(
            "DIFFER: TileEntitySpecialRenderer byte {at}, Rust {} bytes, Java {} bytes",
            actual.len(),
            expected.len()
        );
        return false;
    }
    println!(
        "EXACT: TileEntitySpecialRenderer ({cases} cases, {} bytes)",
        actual.len()
    );
    true
}
