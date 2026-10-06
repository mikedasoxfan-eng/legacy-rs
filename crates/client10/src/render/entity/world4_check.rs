//! Direct comparisons of the two small geometry classes owned by world round four.
use super::geometry::{PositionTextureVertex, TextureOffset, TexturedQuad};
pub fn check(path: &str) -> bool {
    let mut out = vec![];
    let mut r = worldgen::JavaRandom::new(42);
    for _ in 0..64 {
        let (u, v) = (r.next_int_unbounded(), r.next_int_unbounded());
        for n in TextureOffset::new(u, v).coordinates() {
            out.extend(n.to_be_bytes());
        }
        for size in [[64., 32.], [256., 256.], [127., 93.]] {
            for flip in [false, true] {
                for reverse in [false, true] {
                    let vertices = [
                        [1.25, 2.5, -3.5],
                        [-1., 0., 4.],
                        [0.5, 3., 2.],
                        [7., -4., 0.],
                    ]
                    .map(|pos| PositionTextureVertex { pos, uv: [0.; 2] });
                    let mut q = TexturedQuad::new(
                        vertices,
                        u,
                        v,
                        u.wrapping_add(31),
                        v.wrapping_add(17),
                        size,
                    );
                    if flip {
                        q.vertices.reverse();
                    }
                    q.invert_normal = reverse;
                    for p in q.vertices {
                        for n in p.pos {
                            out.extend(n.to_be_bytes());
                        }
                        for n in p.uv {
                            out.extend(n.to_be_bytes());
                        }
                    }
                    out.extend(super::living::packed_normal(&q).to_be_bytes());
                }
            }
        }
    }
    let expected = std::fs::read(path).expect("read world4 geometry oracle");
    if out == expected {
        println!("EXACT MATCH ({} bytes)", out.len());
        true
    } else {
        eprintln!(
            "DIFFERS: geometry first {:?}, lengths {}/{}",
            out.iter().zip(&expected).position(|(a, b)| a != b),
            out.len(),
            expected.len()
        );
        false
    }
}
