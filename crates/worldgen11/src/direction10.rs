//! Release 1.0 Direction and Facing constant tables (renamed Beta helpers).
pub struct Direction;
impl Direction {
    pub const OFFSETS_X: [i32; 4] = [0, -1, 0, 1];
    pub const OFFSETS_Z: [i32; 4] = [1, 0, -1, 0];
    pub const HEAD_INVISIBLE_FACE: [i32; 4] = [3, 4, 2, 5];
    pub const SIDE_TO_DIRECTION: [i32; 6] = [-1, -1, 2, 0, 1, 3];
    pub const FOOT_INVISIBLE_FACE_REMAP: [i32; 4] = [2, 3, 0, 1];
    pub const CLOCKWISE: [i32; 4] = [1, 2, 3, 0];
    pub const COUNTERCLOCKWISE: [i32; 4] = [3, 0, 1, 2];
    pub const BED_DIRECTION: [[i32; 6]; 4] = [
        [1, 0, 3, 2, 5, 4], [1, 0, 5, 4, 2, 3],
        [1, 0, 2, 3, 4, 5], [1, 0, 4, 5, 3, 2],
    ];
}
pub struct Facing;
impl Facing {
    pub const OPPOSITE: [i32; 6] = [1, 0, 3, 2, 5, 4];
    pub const OFFSETS_X: [i32; 6] = [0, 0, 0, 0, -1, 1];
    pub const OFFSETS_Y: [i32; 6] = [-1, 1, 0, 0, 0, 0];
    pub const OFFSETS_Z: [i32; 6] = [0, 0, -1, 1, 0, 0];
}
