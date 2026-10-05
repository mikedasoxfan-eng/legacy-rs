pub fn check(path: &str) -> bool {
    let expected = std::fs::read(path).unwrap();
    let mut input = expected.as_slice();
    fn int(data: &mut &[u8]) -> i32 {
        let n = i32::from_le_bytes(data[..4].try_into().unwrap());
        *data = &data[4..];
        n
    }
    fn string(data: &mut &[u8]) -> String {
        let n = int(data) as usize;
        let chars: Vec<_> = data[..n * 2]
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
            .collect();
        *data = &data[n * 2..];
        String::from_utf16(&chars).unwrap()
    }
    let count = int(&mut input);
    let mut failures = 0;
    for c in 0..count {
        let name = string(&mut input);
        let requested = string(&mut input);
        let (path, blur, clamp) = super::texture_options(&name);
        let flags = &input[..5];
        input = &input[5..];
        let uploads = int(&mut input);
        if path != requested.trim_start_matches('/')
            || flags[..4] != [blur as u8, clamp as u8, 0, 0]
            || flags[4] != 1
            || uploads != 1
        {
            eprintln!("RENDERENGINE10 prefix {c} differs");
            failures += 1;
        }
    }
    assert!(input.is_empty());
    println!(
        "RENDERENGINE10: {count} original texture-prefix/filter/reset/cache cases, {failures} differences"
    );
    failures == 0
}
