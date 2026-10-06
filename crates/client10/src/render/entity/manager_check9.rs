//! Independent original-bytecode RenderManager recordings.
use super::manager::*;
fn i(out: &mut Vec<u8>, value: i32) {
    out.extend(value.to_le_bytes());
}
fn f(out: &mut Vec<u8>, value: f32) {
    out.extend(value.to_bits().to_le_bytes());
}
fn d(out: &mut Vec<u8>, value: f64) {
    out.extend(value.to_bits().to_le_bytes());
}
fn text(out: &mut Vec<u8>, value: &str) {
    i(out, value.len() as i32);
    out.extend(value.as_bytes());
}
fn status(result: Result<(), Error>) -> i32 {
    match result {
        Ok(()) => 0,
        Err(Error::NullPointer) => 1,
        Err(Error::Callback) => 2,
    }
}
fn lookup(m: &mut RenderManager, class: Option<&str>, out: &mut Vec<u8>) {
    i(out, m.classes.len() as i32);
    i(out, class.is_some_and(|c| m.classes.contains_key(c)) as i32);
    let result = m.get_entity_class_render_object(class);
    i(out, result.is_err() as i32);
    i(out, result.ok().flatten().map_or(-1, |v| v as i32));
    i(out, m.classes.len() as i32);
    i(out, class.is_some_and(|c| m.classes.contains_key(c)) as i32);
}
fn registry(out: &mut Vec<u8>) {
    let mut m = RenderManager::default();
    m.declare_class("Child", Some("EntityPig"));
    m.declare_class("ChildBase", Some("EntityPig"));
    m.declare_class("Grandchild", Some("ChildBase"));
    m.declare_class("String", Some("Object"));
    m.declare_class("Runnable", None);
    i(out, m.classes.len() as i32);
    for (j, r) in RENDERERS.iter().enumerate() {
        text(out, r.renderer_class);
        f(out, r.shadow_size);
        f(out, r.shadow_opacity);
        i(out, 1); // The renderer slots are owned by this manager.
        for k in 0..j {
            i(out, (j == k) as i32);
        }
    }
    for c in [
        Some("EntityPigZombie"),
        Some("EntityMob"),
        Some("EntityAnimal"),
        Some("EntityWaterMob"),
        Some("EntityFlying"),
        Some("EntityWeatherEffect"),
        Some("EntityPlayerSP"),
        Some("EntityOtherPlayerMP"),
        Some("EntityFX"),
        Some("EntityPickupFX"),
        Some("Child"),
        Some("Grandchild"),
        Some("ChildBase"),
        Some("Entity"),
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
        (m.get_entity_class_render_object(Some("Child")).unwrap() == Some(2)) as i32,
    );
    i(out, status(m.get_entity_render_object(None).map(|_| ())));
    m.classes.insert("EntityPig".into(), Some(4));
    lookup(&mut m, Some("Child"), out);
    lookup(&mut m, Some("Grandchild"), out);
    m.classes.clear();
    m.classes.insert("Entity".into(), None);
    lookup(&mut m, Some("Grandchild"), out);
    lookup(&mut m, Some("Grandchild"), out);
    m.classes.insert("Entity".into(), Some(18));
    lookup(&mut m, Some("Grandchild"), out);
}
pub(super) fn pose(c: usize) -> CameraState {
    CameraState {
        last: [
            [-1e8, 85.1, -3.75],
            [-0.125, -0.25, 7.1],
            [1e16, -17.75, 2.1],
            [0.0; 3],
        ][c],
        position: [
            [0.125, 80.7, 7.1],
            [-17.01, 63.5, -0.1],
            [-1e16, 128.1, -2.1],
            [1.0, -1.0, 3.0],
        ][c],
        previous_rotation: [[-33.7, 11.3], [1e10, -1e10], [-0.0, 0.0], [179.0, -90.0]][c],
        rotation: [[123.5, -26.75], [-1e10, 1e10], [0.0, -0.0], [-179.0, 90.0]][c],
        sleeping: false,
    }
}
fn camera_values(m: &RenderManager<i32>, out: &mut Vec<u8>) {
    for v in m.view {
        f(out, v);
    }
    for v in m.camera {
        d(out, v);
    }
}
fn cache(out: &mut Vec<u8>) {
    let mut m = RenderManager::<i32>::default();
    m.view = [37.25, -21.5];
    m.camera = [19.125, -3.25, 7.75];
    for scene in 0..5 {
        for sleep in 0..3 {
            for resources in 0..8 {
                for partial in [-0.25f32, 0.0, 0.375, 1.0, 1.25] {
                    for c in 0..4 {
                        let world = (scene != 0).then_some(if scene % 2 == 1 { 1 } else { 2 });
                        let mut player = pose(c);
                        player.sleeping = sleep == 1;
                        let context = Context {
                            world,
                            engine: (resources & 1 != 0).then_some(1),
                            font: (resources & 2 != 0).then_some(1),
                            player: (sleep != 2).then_some(1),
                            options: (resources & 4 != 0).then_some(1),
                        };
                        let result = m.cache_active_render_info(
                            context,
                            (sleep != 2).then_some(player),
                            partial,
                            false,
                            |xyz| {
                                if world.is_none() {
                                    return Err(Error::NullPointer);
                                }
                                i(out, 10);
                                for v in xyz {
                                    i(out, v);
                                }
                                let id = if scene == 1 { 0 } else { 26 };
                                if id == 26 {
                                    i(out, 11);
                                    for v in xyz {
                                        i(out, v);
                                    }
                                }
                                Ok((id, [0, 0, 0, 2, 15][scene]))
                            },
                        );
                        i(out, status(result));
                        for v in [
                            m.context.world,
                            m.context.engine,
                            m.get_font_renderer().copied(),
                            m.context.player,
                            m.context.options,
                        ] {
                            i(out, v.unwrap_or(0));
                        }
                        camera_values(&m, out);
                        d(out, m.get_distance_to_camera([-3.5, 68.0, 4.5]));
                        m.set((c % 2 != 0).then_some(2));
                        i(out, m.context.world.unwrap_or(0));
                        camera_values(&m, out);
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
fn event(out: &mut Vec<u8>, state: &mut Gl, mode: i32, event: Event) -> Result<(), Error> {
    match event {
        Event::Lightmap(light) => state.light = light,
        Event::White => state.color = [1.0; 4],
        Event::Body(id, args) | Event::ShadowAndFire(id, args) => {
            let body = matches!(event, Event::Body(..));
            i(out, if body { 21 } else { 22 });
            i(out, id as i32);
            for v in args.position {
                d(out, v);
            }
            f(out, args.yaw);
            f(out, args.partial);
            gl(out, *state);
            if body {
                state.light = [-23.5, 9.25];
                state.color = [0.1, 0.2, 0.3, 0.4];
                if mode == 3 && id == 1 {
                    return Err(Error::Callback);
                }
            }
        }
    }
    Ok(())
}
fn dispatch(out: &mut Vec<u8>) {
    use std::cell::RefCell;
    for mode in 0..6 {
        for partial in [-0.25f32, 0.0, 0.375, 1.0, 1.25] {
            for light in [0, 1, 65535, 65536, 15728880, -1, i32::MIN, i32::MAX] {
                let mut m = RenderManager::<()>::default();
                m.classes.clear();
                m.classes.insert("Entity".into(), (mode != 2).then_some(0));
                if mode != 1 && mode != 2 {
                    m.classes.insert("EntityPig".into(), Some(1));
                }
                m.declare_class("Camera", Some("EntityPig"));
                m.render_origin = [19.125, -3.25, 7.75];
                let state = RefCell::new(Gl::default());
                let output = RefCell::new(&mut *out);
                let result = m.render_entity(
                    (mode != 5).then_some("Camera"),
                    (mode != 5).then_some(pose(0)),
                    partial,
                    |t| {
                        let mut o = output.borrow_mut();
                        i(&mut o, 20);
                        f(&mut o, t);
                        gl(&mut o, *state.borrow());
                        if mode == 4 {
                            Err(Error::Callback)
                        } else {
                            Ok(light)
                        }
                    },
                    |e| event(&mut output.borrow_mut(), &mut state.borrow_mut(), mode, e),
                );
                i(&mut output.borrow_mut(), status(result));
                gl(&mut output.borrow_mut(), *state.borrow());
                *state.borrow_mut() = Gl::default();
                let result = m.render_entity_with_pos_yaw(
                    (mode != 5).then_some("Camera"),
                    Arguments {
                        position: [-0.25, 10.5, 3.75],
                        yaw: -75.125,
                        partial,
                    },
                    |e| event(&mut output.borrow_mut(), &mut state.borrow_mut(), mode, e),
                );
                i(&mut output.borrow_mut(), status(result));
                gl(&mut output.borrow_mut(), *state.borrow());
            }
        }
    }
}
pub fn check(path: &str) -> bool {
    let mut actual = Vec::new();
    registry(&mut actual);
    cache(&mut actual);
    dispatch(&mut actual);
    let expected = std::fs::read(path).unwrap();
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
            "DIFFER: RenderManager at byte {at}: Rust {} bytes, Java {} bytes",
            actual.len(),
            expected.len()
        );
        return false;
    }
    println!(
        "EXACT: RenderManager (32 registrations, 2400 camera caches, 480 dispatches, {} bytes)",
        actual.len()
    );
    true
}
