fn main() {
    let a:Vec<String>=std::env::args().collect();
    let actual=worldgen::entity::player::verify181::replay(&a[1],a[2].parse().unwrap());
    std::fs::write(&a[3],&actual).unwrap();
    if let Some(path)=a.get(4) {
        let expected=std::fs::read(path).unwrap();
        if actual!=expected {
            let at=actual.iter().zip(&expected).position(|(a,b)|a!=b).unwrap_or(actual.len().min(expected.len()));
            let mut offset=0;let mut frame=0;
            while offset+4<=expected.len() {let len=i32::from_le_bytes(expected[offset..offset+4].try_into().unwrap()) as usize;if at<offset+4+len {break;}offset+=4+len;frame+=1;}
            eprintln!("{}: DIFFERS frame {} byte {} (frame byte {}), lengths {}/{}",a[1],frame,at,at-offset,actual.len(),expected.len());
            std::process::exit(1);
        }
        println!("{}: EXACT ({} ticks, {} bytes)",a[1],a[2],actual.len());
    }
}
