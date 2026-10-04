//! Diff the Rust generator against an oracle dump.
//!
//!   compare raw <seed> <oracle.bin>
//!   compare sin <oracle-sin.bin>

use std::process::ExitCode;
use worldgen::{math, OverworldGenerator};

const CHUNK_RECORD: usize = 8 + 32768 + 16384 + 256 + 256 * 16;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("raw") => raw(args[2].parse().expect("seed"), &args[3]),
        Some("sin") => sin(&args[2]),
        _ => {
            eprintln!("usage: compare raw <seed> <oracle.bin> | compare sin <oracle-sin.bin>");
            ExitCode::from(2)
        }
    }
}

fn i32_at(b: &[u8], o: usize) -> i32 {
    i32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

fn f64_at(b: &[u8], o: usize) -> f64 {
    f64::from_le_bytes(b[o..o + 8].try_into().unwrap())
}

fn raw(seed: i64, path: &str) -> ExitCode {
    let data = std::fs::read(path).expect("read oracle dump");
    assert_eq!(&data[..8], b"B173RAW1", "not a raw oracle dump");
    let body = &data[8..];
    assert_eq!(body.len() % CHUNK_RECORD, 0, "truncated dump");

    let mut gen = OverworldGenerator::new(seed);
    let (mut chunks, mut bad_chunks, mut bad_blocks) = (0, 0, 0usize);
    let (mut bad_biomes, mut bad_climate) = (0usize, 0usize);
    for rec in body.chunks(CHUNK_RECORD) {
        let (cx, cz) = (i32_at(rec, 0), i32_at(rec, 4));
        let blocks = &rec[8..8 + 32768];
        let biomes = &rec[8 + 32768 + 16384..8 + 32768 + 16384 + 256];
        let climate = &rec[8 + 32768 + 16384 + 256..];

        let chunk = gen.generate(cx, cz);
        chunks += 1;

        // Climate arrays still hold this chunk's 16x16 values right after generate().
        for i in 0..256 {
            let t = f64_at(climate, i * 8);
            let h = f64_at(climate, 2048 + i * 8);
            if t.to_bits() != gen.biome_source.temperature[i].to_bits()
                || h.to_bits() != gen.biome_source.humidity[i].to_bits()
            {
                if bad_climate == 0 {
                    eprintln!(
                        "climate mismatch chunk ({cx},{cz}) col {i}: java t={t:e} h={h:e}, rust t={:e} h={:e}",
                        gen.biome_source.temperature[i], gen.biome_source.humidity[i]
                    );
                }
                bad_climate += 1;
            }
            if biomes[i] != chunk.biomes[i] as u8 {
                bad_biomes += 1;
            }
        }

        let mut diff = 0;
        for (i, (&j, &r)) in blocks.iter().zip(chunk.blocks.iter()).enumerate() {
            if j != r {
                if diff < 5 {
                    let (x, z, y) = (i >> 11, (i >> 7) & 15, i & 127);
                    eprintln!("chunk ({cx},{cz}) block x={x} y={y} z={z}: java={j} rust={r}");
                }
                diff += 1;
            }
        }
        if diff > 0 {
            bad_chunks += 1;
            bad_blocks += diff;
        }
    }

    println!(
        "seed {seed}: {chunks} chunks, {bad_chunks} differ ({bad_blocks} blocks), \
         {bad_biomes} biome columns differ, {bad_climate} climate values differ"
    );
    if bad_chunks == 0 && bad_biomes == 0 && bad_climate == 0 {
        println!("EXACT MATCH");
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn sin(path: &str) -> ExitCode {
    let data = std::fs::read(path).expect("read sin dump");
    let mut bad = 0;
    for (i, r) in math::sin_table_bits().enumerate() {
        let j = u32::from_le_bytes(data[i * 4..i * 4 + 4].try_into().unwrap());
        if j != r {
            if bad < 5 {
                eprintln!("sin[{i}]: java={:e} rust={:e}", f32::from_bits(j), f32::from_bits(r));
            }
            bad += 1;
        }
    }
    println!("sin table: {bad} of 65536 entries differ");
    if bad == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
