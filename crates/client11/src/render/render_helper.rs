//! Release 1.0 `RenderHelper`: the standard item lights, the GUI variant `func_41089_c` and the
//! GL calls they make (GlLog10 numbering). Entity and GUI shading read `light_position`.
use super::opengl_helper::Call;

const LIGHTING: f32 = 2896.0;
const LIGHT0: f32 = 16384.0;
const LIGHT1: f32 = 16385.0;
const COLOR_MATERIAL: f32 = 2903.0;

/// `Vec3D.createVector(x, 1, z).normalize()`: MathHelper.sqrt_double rounds the length to float.
pub fn light_position(x: f32, z: f32) -> [f32; 3] {
    let (x, z) = (x as f64, z as f64);
    let len = ((x * x + 1.0 * 1.0 + z * z).sqrt() as f32) as f64;
    if len < 1.0e-4 {
        return [0.0; 3];
    }
    [(x / len) as f32, (1.0 / len) as f32, (z / len) as f32]
}

fn light(out: &mut Vec<Call>, light: f32, position: [f32; 3]) {
    out.push((21, [light, 4611.0, 0.0, 0.0]));
    out.push((22, [position[0], position[1], position[2], 0.0]));
    out.push((21, [light, 4609.0, 0.0, 0.0]));
    out.push((22, [0.6, 0.6, 0.6, 1.0]));
    out.push((21, [light, 4608.0, 0.0, 0.0]));
    out.push((22, [0.0, 0.0, 0.0, 1.0]));
    out.push((21, [light, 4610.0, 0.0, 0.0]));
    out.push((22, [0.0, 0.0, 0.0, 1.0]));
}

/// `enableStandardItemLighting`.
pub fn enable_standard_item_lighting(out: &mut Vec<Call>) {
    for cap in [LIGHTING, LIGHT0, LIGHT1, COLOR_MATERIAL] {
        out.push((6, [cap, 0.0, 0.0, 0.0]));
    }
    out.push((30, [1032.0, 5634.0, 0.0, 0.0]));
    light(out, LIGHT0, light_position(0.2, -0.7));
    light(out, LIGHT1, light_position(-0.2, 0.7));
    out.push((23, [2899.0, 0.0, 0.0, 0.0]));
    out.push((22, [0.4, 0.4, 0.4, 1.0]));
}

/// `func_41089_c`: the lights placed under rotate(-30, Y) then rotate(165, X) (1.0 GUI items).
pub const GUI_LIGHT_ROTATIONS: [[f32; 4]; 2] = [[-30.0, 0.0, 1.0, 0.0], [165.0, 1.0, 0.0, 0.0]];
pub fn gui_item_lighting(out: &mut Vec<Call>) {
    out.push((0, [0.0; 4]));
    for r in GUI_LIGHT_ROTATIONS {
        out.push((4, r));
    }
    enable_standard_item_lighting(out);
    out.push((1, [0.0; 4]));
}

/// `disableStandardItemLighting`.
pub fn disable_standard_item_lighting(out: &mut Vec<Call>) {
    for cap in [LIGHTING, LIGHT0, LIGHT1, COLOR_MATERIAL] {
        out.push((5, [cap, 0.0, 0.0, 0.0]));
    }
}
