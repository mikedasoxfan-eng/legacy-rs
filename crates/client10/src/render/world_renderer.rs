//! Release 1.0 `WorldRenderer.updateRenderer` for one 16-block section: two passes in y, z, x
//! order through RenderBlocks, the pass-skipping rules, the special tile-entity list and the
//! section-relative vertex origin. (1.0 only adds the write-only `bytesDrawn` total.)
use super::access::Access;
use super::blocks::{Colorizers, RenderBlocks};
use super::video::VideoSettings;
use super::{rprops, Tessellator, Vertex};
use std::cell::RefCell;
use worldgen::{BiomeSource, World};

pub struct Section {
    /// The compiled display list of each pass (`None`: no glNewList for the pass).
    pub lists: [Option<Vec<Vertex>>; 2],
    pub skip_render_pass: [bool; 2],
    /// `tileEntityRenderers`: container tiles with a TileEntitySpecialRenderer, in scan order.
    pub tile_entity_renderers: Vec<[i32; 3]>,
    /// `isChunkLit` (`Chunk.isLit`, never set in 1.0).
    pub chunk_lit: bool,
    /// `bytesDrawn`: written by 1.0, never read.
    pub bytes_drawn: i32,
}

/// Tile entities TileEntityRenderer has a renderer for (sign, spawner, piston, chest, book,
/// End portal); `Block.isBlockContainer` holds for all of them.
fn special_tile(id: u8) -> bool {
    matches!(id, 63 | 68 | 52 | 36 | 54 | 116 | 119)
}

pub fn update_renderer(
    world: &World,
    cx: i32,
    sy: i32,
    cz: i32,
    sky_sub: i32,
    biomes: &RefCell<BiomeSource>,
    col: &Colorizers,
    video: VideoSettings,
) -> Section {
    let mut access = Access::new(world, cx, cz, sky_sub, biomes);
    access.fancy = video.fancy;
    let origin = [cx as f64 * 16.0, sy as f64 * 16.0, cz as f64 * 16.0];
    let chunk = world.chunk(cx, cz);
    let mut s = Section { lists: [None, None], skip_render_pass: [true; 2], tile_entity_renderers: Vec::new(), chunk_lit: false, bytes_drawn: 0 };
    for pass in 0..2u8 {
        let mut other_pass = false;
        let mut rendered = false;
        let mut started = false;
        let mut t = Tessellator::new(origin);
        for y in sy * 16..sy * 16 + 16 {
            for z in 0..16 {
                for x in 0..16 {
                    let id = chunk.block(x, y, z);
                    if id == 0 {
                        continue;
                    }
                    started = true;
                    let (bx, bz) = (cx * 16 + x, cz * 16 + z);
                    if pass == 0 && special_tile(id) && (id != 36 || chunk.tile_entities.contains_key(&(x, y, z))) {
                        s.tile_entity_renderers.push([bx, y, bz]);
                    }
                    if rprops(id).pass != pass {
                        other_pass = true;
                    } else {
                        let mut rb = RenderBlocks::new(&access, &mut t, col);
                        rb.ao = video.ao;
                        rb.anaglyph = video.anaglyph;
                        rendered |= rb.render_block(id, bx, y, bz);
                    }
                }
            }
        }
        if started {
            s.bytes_drawn += t.bytes_drawn();
            s.lists[pass as usize] = Some(t.verts);
        } else {
            rendered = false;
        }
        if rendered {
            s.skip_render_pass[pass as usize] = false;
        }
        if !other_pass {
            break;
        }
    }
    s
}

/// `--worldrenderer4-check seed wrand fancy|fast on|off dump`: every section of the dump.
pub fn check(a: &[String]) -> bool {
    let seed: i64 = a[2].parse().unwrap();
    let wrand: i64 = a[3].parse().unwrap();
    let mut video = VideoSettings::default();
    video.fancy = a[4] != "fast";
    video.ao = a[5] != "off";
    let data = std::fs::read(&a[6]).expect("WorldRenderer dump");
    let mut at = 0usize;
    let mut next = || {
        let v = i32::from_le_bytes(data[at..at + 4].try_into().unwrap());
        at += 4;
        v
    };
    let assets = crate::assets::load().expect("assets");
    let col = Colorizers { grass: assets.grass_color, foliage: assets.foliage_color };
    let mut world = World::new_vanilla(seed, wrand);
    world.leaves_opaque = !video.fancy;
    let biomes = RefCell::new(BiomeSource::new(seed));
    assert_eq!(next(), 0x34525257);
    let sky_sub = next();
    let n = next();
    let (mut bad, mut checks, mut vertices) = (0usize, 0usize, 0usize);
    let mut same = |j: i32, r: i32, what: &str, bad: &mut usize| {
        checks += 1;
        if j != r {
            if *bad < 12 {
                eprintln!("{what}: java {j:08x}, rust {r:08x}");
            }
            *bad += 1;
        }
    };
    for _ in 0..n {
        let (cx, sy, cz) = (next(), next(), next());
        let s = update_renderer(&world, cx, sy, cz, sky_sub, &biomes, &col, video);
        let label = format!("section ({cx},{sy},{cz})");
        for pass in 0..2 {
            let present = next();
            same(present, s.lists[pass].is_some() as i32, &format!("{label} pass {pass} list"), &mut bad);
            same(next(), s.skip_render_pass[pass] as i32, &format!("{label} pass {pass} skipRenderPass"), &mut bad);
            let nv = next() as usize;
            let empty = Vec::new();
            let rust = s.lists[pass].as_ref().unwrap_or(&empty);
            same(nv as i32, rust.len() as i32, &format!("{label} pass {pass} vertices"), &mut bad);
            vertices += nv;
            for k in 0..nv {
                let j: Vec<i32> = (0..7).map(|_| next()).collect();
                if let Some(r) = rust.get(k) {
                    let rv = [r.pos[0].to_bits() as i32, r.pos[1].to_bits() as i32, r.pos[2].to_bits() as i32, r.uv[0].to_bits() as i32, r.uv[1].to_bits() as i32, i32::from_le_bytes(r.color), r.light as i32];
                    for (f, (jv, rv)) in j.iter().zip(rv).enumerate() {
                        same(*jv, rv, &format!("{label} pass {pass} vertex {k}[{f}]"), &mut bad);
                    }
                }
            }
        }
        let tiles = next() as usize;
        same(tiles as i32, s.tile_entity_renderers.len() as i32, &format!("{label} tileEntityRenderers"), &mut bad);
        for k in 0..tiles {
            let j = [next(), next(), next()];
            let r = s.tile_entity_renderers.get(k).copied().unwrap_or([i32::MIN; 3]);
            for (jv, rv) in j.into_iter().zip(r) {
                same(jv, rv, &format!("{label} tile {k}"), &mut bad);
            }
        }
        same(next(), tiles as i32, &format!("{label} added tile entities"), &mut bad);
        same(next(), s.chunk_lit as i32, &format!("{label} isChunkLit"), &mut bad);
    }
    assert_eq!(at, data.len(), "WorldRenderer dump EOF");
    println!("WORLD RENDERER10 {}: {n} sections, {vertices} vertices, {checks} comparisons, {bad} differences", if bad == 0 { "EXACT" } else { "DIFFER" });
    bad == 0
}
