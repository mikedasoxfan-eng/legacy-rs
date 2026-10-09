//! Release RenderGlobal geometry before the GL model-view transforms.
use super::Vertex;
use super::entity::fixed::{self, Op};

pub fn sun_quad() -> [Vertex; 6] {
    let points = [[-30.0,100.0,-30.0],[30.0,100.0,-30.0],[30.0,100.0,30.0],[-30.0,100.0,30.0]];
    let uv = [[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,1.0]];
    [0,1,2,0,2,3].map(|i| Vertex {pos: points[i],uv: uv[i],color:[255;4],light:0})
}

pub fn celestial_operations(angle: f32) -> [Op; 3] {
    [Op{code:0,args:[0.0;4]}, Op{code:2,args:[-90.0,0.0,1.0,0.0]},
     Op{code:2,args:[angle*360.0,1.0,0.0,0.0]}]
}

pub fn sunrise_operations(angle: f32) -> [Op; 3] {
    let z = if worldgen::math::sin(angle*std::f32::consts::PI*2.0)<0.0 {180.0}else{0.0};
    [Op{code:2,args:[90.0,1.0,0.0,0.0]}, Op{code:2,args:[z,0.0,0.0,1.0]},
     Op{code:2,args:[90.0,0.0,0.0,1.0]}]
}

pub fn end_operations(face: usize) -> Vec<Op> {
    match face {
        0 => vec![],
        1..=3 => vec![Op {code:2,args:[[90.0,-90.0,180.0][face-1],1.0,0.0,0.0]}],
        4..=5 => vec![Op {code:2,args:[if face==4 {90.0}else{-90.0},0.0,0.0,1.0]}],
        _ => unreachable!(),
    }
}

pub fn moon_phase(world_time: i64) -> i32 {
    (world_time / 24000) as i32 % 8
}

pub fn moon_quad(phase: i32) -> [Vertex; 6] {
    let col = phase % 4;
    let row = phase / 4 % 2;
    let u0 = col as f32 / 4.0;
    let v0 = row as f32 / 2.0;
    let u1 = (col + 1) as f32 / 4.0;
    let v1 = (row + 1) as f32 / 2.0;
    let pts = [
        [-20.0, -100.0, 20.0],
        [20.0, -100.0, 20.0],
        [20.0, -100.0, -20.0],
        [-20.0, -100.0, -20.0],
    ];
    let uv = [[u1, v1], [u0, v1], [u0, v0], [u1, v0]];
    [0, 1, 2, 0, 2, 3].map(|i| Vertex {
        pos: pts[i],
        uv: uv[i],
        color: [255; 4],
        light: 0,
    })
}

pub fn end_quad() -> [Vertex; 6] {
    let pts = [
        [-100.0, -100.0, -100.0],
        [-100.0, -100.0, 100.0],
        [100.0, -100.0, 100.0],
        [100.0, -100.0, -100.0],
    ];
    let uv = [[0.0, 0.0], [0.0, 16.0], [16.0, 16.0], [16.0, 0.0]];
    [0, 1, 2, 0, 2, 3].map(|i| Vertex {
        pos: pts[i],
        uv: uv[i],
        color: [24, 24, 24, 255],
        light: 0,
    })
}

/// The six independent push/rotate/pop calls, in source order.
pub fn end_rotation(face: usize) -> glam::Mat4 {
    fixed::matrix(&end_operations(face))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn moon_phase_keeps_java_division_and_int_narrowing() {
        assert_eq!(moon_phase(-23999), 0);
        assert_eq!(moon_phase(-24000), -1);
        assert_eq!(moon_phase(8 * 24000), 0);
        assert_eq!(moon_phase((i32::MAX as i64 + 1) * 24000), 0);
        for phase in 0..8 {
            let q = moon_quad(phase);
            assert_eq!(
                q[0].uv,
                [(phase % 4 + 1) as f32 / 4.0, (phase / 4 + 1) as f32 / 2.0]
            );
            assert_eq!(
                q[2].uv,
                [(phase % 4) as f32 / 4.0, (phase / 4) as f32 / 2.0]
            );
        }
    }
    #[test]
    fn end_faces_point_inward_and_repeat_the_tunnel_texture() {
        let q = end_quad();
        assert_eq!(q[0].color, [24, 24, 24, 255]);
        assert_eq!(q[2].uv, [16.0, 16.0]);
        let centers = [
            [0.0, -100.0, 0.0],
            [0.0, 0.0, -100.0],
            [0.0, 0.0, 100.0],
            [0.0, 100.0, 0.0],
            [100.0, 0.0, 0.0],
            [-100.0, 0.0, 0.0],
        ];
        for (i, center) in centers.into_iter().enumerate() {
            let rot = end_rotation(i);
            let p = rot.transform_point3(glam::Vec3::new(0.0, -100.0, 0.0));
            assert!((p - glam::Vec3::from_array(center)).length() < 0.0001);
            let a = rot.transform_point3(glam::Vec3::from_array(q[0].pos));
            let b = rot.transform_point3(glam::Vec3::from_array(q[1].pos));
            let c = rot.transform_point3(glam::Vec3::from_array(q[2].pos));
            assert!((b - a).cross(c - a).dot(-p) > 0.0);
        }
    }
}
