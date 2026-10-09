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
    let color = if a.iter().any(|s| s == "anaglyph") { super::stereo_color(color) } else { color };
    let offset = tick as f32 + partial;
    let verts = if fancy {
        super::sky::fancy_clouds(cam[0], cam[1], cam[2], offset, color)
    } else {
        super::sky::flat_clouds(cam[0], cam[1], cam[2], offset, color)
    };
    let passes = if fancy { 2 } else { 1 };
    assert_eq!(count, verts.len() * passes, "cloud vertex count");
    let mut offset = 12;
    for pass in 0..passes {
        for (i, v) in verts.iter().enumerate() {
            let fields = [
                2.0,
                v.pos[0],
                v.pos[1],
                v.pos[2],
                v.uv[0],
                v.uv[1],
                v.color[0] as f32 / 255.0,
                v.color[1] as f32 / 255.0,
                v.color[2] as f32 / 255.0,
                v.color[3] as f32 / 255.0,
            ];
            for (j, f) in fields.into_iter().enumerate() {
                let java = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
                assert_eq!(f.to_bits(), java, "cloud pass {pass} vertex {i} field {j} ({f})");
                offset += 4;
            }
        }
    }
    assert_eq!(offset, data.len());
    println!("CLOUDS EXACT: {count} vertices ({})", if fancy { "Fancy" } else { "Fast" });
}

/// Actual RenderGlobal.renderSky bytecode: sky/void GL colors and sunrise Tessellator bits.
pub fn sky_draw(path: &str) {
    let bytes=std::fs::read(path).expect("sky oracle dump");
    assert_eq!(&bytes[..8],b"B173SKY1");
    let mut at=8;
    fn u(bytes: &[u8], at: &mut usize) -> u32 { let v=u32::from_le_bytes(bytes[*at..*at+4].try_into().unwrap());*at+=4;v }
    fn d(bytes: &[u8], at: &mut usize) -> f64 { let v=f64::from_le_bytes(bytes[*at..*at+8].try_into().unwrap());*at+=8;v }
    let cases=u(&bytes,&mut at);
    let biomes=worldgen::BiomeSource::new(42);
    let mut compared=0;
    for case in 0..cases {
        let x=d(&bytes,&mut at);let z=d(&bytes,&mut at);let time=u(&bytes,&mut at) as i32;let flash=u(&bytes,&mut at) as i32;
        let partial=f32::from_bits(u(&bytes,&mut at));let rain=f32::from_bits(u(&bytes,&mut at));let thunder=f32::from_bits(u(&bytes,&mut at));let ana=u(&bytes,&mut at)!=0;
        let angle=super::sky::celestial_angle(time as i64,partial);
        let temp=biomes.raw_temperature(x.floor() as i32,z.floor() as i32) as f32;
        let rgb=super::sky::sky_color(angle,temp,rain,rain*thunder,flash,partial);
        let rgb=if ana { super::stereo_color(rgb) } else { rgb };
        // b1.8.1 sets black before the void (the below-sea-level box uses it), then the void colour.
        assert_eq!(u(&bytes,&mut at),4);
        for expected in [rgb,rgb,[0.0,0.0,0.0],[rgb[0]*0.2+0.04,rgb[1]*0.2+0.04,rgb[2]*0.6+0.1]] {
            for value in expected { assert_eq!(value.to_bits(),u(&bytes,&mut at),"sky case {case}");compared+=1; }
        }
        let fan=super::sky::sunrise_fan(angle,ana);
        let count=u(&bytes,&mut at) as usize;
        assert_eq!(count,if fan.is_some(){26}else{8});
        for vertex in 0..count {
            let fields: [f32;10]=std::array::from_fn(|_| f32::from_bits(u(&bytes,&mut at)));
            if let Some(fan)=&fan { if vertex<18 {
                let index=if vertex==0 {0} else if vertex==17 {47} else {(vertex-1)*3+1};
                let v=fan[index];
                for (j,value) in v.pos.into_iter().enumerate(){ assert_eq!(value.to_bits(),fields[j+1].to_bits(),"sunrise case {case} vertex {vertex}");compared+=1; }
                for (j,value) in v.color.into_iter().enumerate(){ assert_eq!((value as f32/255.0).to_bits(),fields[j+6].to_bits(),"sunrise color case {case} vertex {vertex}");compared+=1; }
                continue;
            } }
            for (j,value) in [1.0,1.0,1.0,1.0-rain].into_iter().enumerate(){ assert_eq!(value.to_bits(),fields[j+6].to_bits(),"sun/moon color case {case}");compared+=1; }
        }
    }
    assert_eq!(at,bytes.len());
    println!("SKY DRAWS EXACT: {cases} sky/void/sunrise cases, {compared} float/vertex comparisons (anaglyph on/off)");
}
