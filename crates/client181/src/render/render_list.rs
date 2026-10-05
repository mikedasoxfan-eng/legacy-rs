//! RenderList: where a chunk's geometry lands relative to the camera. WorldRenderer groups
//! renderers by 1024-block region (posXMinus) and translates by the offset inside it
//! (posXClip); b1.8.1's RenderList keeps the camera position in double and rounds
//! `region - camera` to float once.

/// RenderList.callLists' glTranslatef for a renderer at block (x, y, z).
pub fn list_translate(x: i32, y: i32, z: i32, cam: [f64; 3]) -> [f32; 3] {
    let minus = |v: i32| v - (v & 1023);
    [
        (minus(x) as f64 - cam[0]) as f32,
        (minus(y) as f64 - cam[1]) as f32,
        (minus(z) as f64 - cam[2]) as f32,
    ]
}

/// The model-view translation GL composes for the renderer: the RenderList translate
/// followed by WorldRenderer's glTranslatef(posXClip, posYClip, posZClip), in float.
pub fn chunk_origin(x: i32, y: i32, z: i32, cam: [f64; 3]) -> [f32; 3] {
    let t = list_translate(x, y, z, cam);
    [t[0] + (x & 1023) as f32, t[1] + (y & 1023) as f32, t[2] + (z & 1023) as f32]
}

/// --renderlistcheck: compare list_translate with Mode_renderlist181's recorded translates.
pub fn check(path: &str) -> bool {
    let data = std::fs::read(path).expect("renderlist dump");
    let n = i32::from_be_bytes(data[0..4].try_into().unwrap()) as usize;
    let mut o = 4;
    let mut differences = 0;
    for _ in 0..n {
        let int = |o: usize| i32::from_be_bytes(data[o..o + 4].try_into().unwrap());
        let dbl = |o: usize| f64::from_be_bytes(data[o..o + 8].try_into().unwrap());
        let (x, y, z) = (int(o), int(o + 4), int(o + 8));
        let cam = [dbl(o + 12), dbl(o + 20), dbl(o + 28)];
        let java: Vec<u32> = (0..3)
            .map(|i| u32::from_be_bytes(data[o + 36 + i * 4..o + 40 + i * 4].try_into().unwrap()))
            .collect();
        let rust = list_translate(x, y, z, cam).map(f32::to_bits);
        if java != rust {
            if differences < 5 {
                eprintln!("renderer at {x} {y} {z}, camera {cam:?}: Java {java:08x?}, Rust {rust:08x?}");
            }
            differences += 1;
        }
        o += 48;
    }
    println!("render lists: {n} cases, {} raw-bit comparisons, {differences} differences", n * 3);
    if differences == 0 {
        println!("RENDER LIST TRANSLATES EXACT");
    }
    differences == 0
}
