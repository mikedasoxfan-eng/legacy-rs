//! Draw inputs for RenderGlobal's sky, using the production sky geometry.
use super::{entity::fixed::Fixed, sky, sky_release, Vertex};

fn positions(gl: &mut Fixed, points: &[[f32; 3]], separate: bool) {
    if !separate { gl.begin(); }
    for q in points.chunks_exact(6) {
        if separate { gl.begin(); }
        for i in [0, 1, 2, 5] { gl.vertex(q[i].map(f64::from), [0.0; 2]); }
        if separate { gl.untextured(); gl.end(); }
    }
    if !separate { gl.untextured(); gl.end(); }
}
fn quad(gl: &mut Fixed, vertices: [Vertex; 6], color: bool) {
    gl.begin();
    for i in [0, 1, 2, 5] {
        let v = vertices[i];
        if color { gl.vertex_color = Some(u32::from_le_bytes(v.color)); }
        gl.vertex(v.pos.map(f64::from), v.uv.map(f64::from));
    }
    gl.end();
}
pub fn draw(dimension: i32, anaglyph: bool, angle: f32, rain: f32, stars: f32,
            phase: i32, height: f64, rgb: [f32; 3]) -> Fixed {
    let mut gl = Fixed::new(1.0);
    if dimension == -1 { return gl; }
    if dimension == 1 {
        gl.enable(16, true); gl.src = 770; gl.dst = 771;
        gl.enable(1, false); gl.enable(32, false); gl.texture = 80;
        for face in 0..6 {
            gl.push();
            for op in sky_release::end_operations(face) {
                gl.ops.push(op);
            }
            quad(&mut gl, sky_release::end_quad(), true); gl.pop();
        }
        gl.enable(32, true); gl.enable(8, true);
        return gl;
    }
    gl.enable(8, false);
    let rgb = if anaglyph { super::stereo_color(rgb) } else { rgb };
    gl.material = [rgb[0], rgb[1], rgb[2], 1.0]; gl.enable(32, false);
    positions(&mut gl, &sky::sky_plane(16.0, true), true);
    gl.enable(8, true); gl.enable(16, true); gl.src = 770; gl.dst = 771; gl.enable(1, false);
    if let Some(fan) = sky::sunrise_fan(angle, anaglyph) {
        gl.enable(8, false); gl.push();
        gl.ops.extend(sky_release::sunrise_operations(angle));
        gl.begin(); gl.mode = 6;
        for v in std::iter::once(fan[0]).chain((0..16).map(|i| fan[i*3+1])).chain(std::iter::once(fan[47])) {
            gl.vertex_color = Some(u32::from_le_bytes(v.color));
            gl.vertex(v.pos.map(f64::from), [0.0; 2]);
        }
        gl.untextured(); gl.end(); gl.mode = 7; gl.pop();
    }
    gl.enable(8, true); gl.src = 770; gl.dst = 1;
    gl.push(); gl.material = [1.0, 1.0, 1.0, 1.0-rain];
    gl.ops.extend(sky_release::celestial_operations(angle));
    gl.texture = 81; quad(&mut gl, sky_release::sun_quad(), false);
    gl.texture = 82; quad(&mut gl, sky_release::moon_quad(phase), false);
    gl.enable(8, false);
    let brightness = stars * (1.0-rain);
    if brightness > 0.0 {
        gl.material = [brightness; 4]; positions(&mut gl, &sky::stars(), false);
    }
    gl.material = [1.0; 4]; gl.enable(16, false); gl.pop();
    gl.enable(8, false); gl.material = [0.0, 0.0, 0.0, 1.0];
    let depth = height - 63.0;
    if depth < 0.0 {
        gl.push(); gl.translate(0.0, 12.0, 0.0);
        positions(&mut gl, &sky::sky_plane(-16.0, false), false); gl.pop();
        let y = -((depth+65.0) as f32); let n = -1.0; let p = 1.0;
        let points = [[n,y,p],[p,y,p],[p,n,p],[n,n,p], [n,n,n],[p,n,n],[p,y,n],[n,y,n],
            [p,n,n],[p,n,p],[p,y,p],[p,y,n], [n,y,n],[n,y,p],[n,n,p],[n,n,n],
            [n,n,n],[n,n,p],[p,n,p],[p,n,n]];
        gl.begin(); gl.vertex_color = Some(0xff000000);
        for point in points { gl.vertex(point.map(f64::from), [0.0;2]); }
        gl.untextured(); gl.end();
    }
    gl.material = [rgb[0]*0.2+0.04, rgb[1]*0.2+0.04, rgb[2]*0.6+0.1, 1.0];
    gl.push(); gl.translate(0.0, -((depth-16.0) as f32), 0.0);
    positions(&mut gl, &sky::sky_plane(-16.0, false), false); gl.pop();
    gl.enable(8, true); gl.enable(32, true);
    gl
}
