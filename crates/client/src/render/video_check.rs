//! Settings-dependent ambient geometry, compared to original-bytecode draw calls.

pub fn clouds(a: &[String]) {
    let cam = [a[3].parse::<f64>().unwrap(), a[4].parse().unwrap(), a[5].parse().unwrap()];
    let tick = a[6].parse::<i32>().unwrap();
    let partial = a[7].parse::<f32>().unwrap();
    let time = a[8].parse().unwrap();
    let rain = a[9].parse().unwrap();
    let thunder = a[10].parse::<f32>().unwrap();
    let fancy = a[11] != "fast";
    let data = std::fs::read(&a[12]).expect("cloud oracle dump");
    assert_eq!(&data[..8], b"B173CLD1");
    let count = i32::from_le_bytes(data[8..12].try_into().unwrap()) as usize;
    let color = super::sky::cloud_color(super::sky::celestial_angle(time, partial), rain, rain * thunder);
    let offset = tick as f32 + partial;
    let verts = if fancy {
        super::sky::fancy_clouds(cam[0],cam[1],cam[2],offset,color)
    } else {
        super::sky::flat_clouds(cam[0],cam[1],cam[2],offset,color)
    };
    let passes = if fancy { 2 } else { 1 };
    assert_eq!(count, verts.len() * passes, "cloud vertex count");
    let mut offset = 12;
    for pass in 0..passes {
        for (i, v) in verts.iter().enumerate() {
            let fields = [2.0, v.pos[0],v.pos[1],v.pos[2],v.uv[0],v.uv[1],
                v.color[0] as f32/255.0, v.color[1] as f32/255.0, v.color[2] as f32/255.0, v.color[3] as f32/255.0];
            for (j, f) in fields.into_iter().enumerate() {
                let java = u32::from_le_bytes(data[offset..offset+4].try_into().unwrap());
                assert_eq!(f.to_bits(),java,"cloud pass {pass} vertex {i} field {j} ({f})");
                offset += 4;
            }
        }
    }
    assert_eq!(offset,data.len());
    println!("CLOUDS EXACT: {count} vertices ({})", if fancy { "Fancy" } else { "Fast" });
}
