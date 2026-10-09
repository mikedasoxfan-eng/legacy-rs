use super::{font::FontRenderer, widgets::GuiButton, ScaledResolution};
/// Real screen callbacks without a window or GPU; only drawing resources are omitted.
pub(super) fn fixture_ui(saves_dir: std::path::PathBuf) -> super::Ui {
    let mut jar = crate::assets::Jar::open().unwrap();
    let font = FontRenderer::new(
        &jar.image("font/default.png").unwrap(),
        0,
        &jar.text("font.txt").unwrap(),
    );
    let mut lang = super::lang::Lang::default();
    for name in ["lang/en_US.lang"] {
        lang.load(jar.text(name).unwrap().as_bytes());
    }
    super::Ui {
        release: Default::default(),
        packs: crate::texture_packs::TexturePackList::new(&saves_dir, "Default"),
        pack_icons: Vec::new(),
        gl: super::gl::Gl::new(),
        font,
        textures: Default::default(),
        lang,
        settings: Default::default(),
        display_width: 854,
        display_height: 480,
        world_open: false,
        hide_quit_button: false,
        splashes: Vec::new(),
        saves_dir,
        actions: Vec::new(),
        millis: 0,
        shift_down: false,
        fixed_date: true,
        clipboard: None,
        mouse_down: false,
        world_saved: true,
        player_dead: false,
        player_score: 0,
        save_requests: Vec::new(),
        sounds: Vec::new(),
        stats: Default::default(),
        container_snapshot: None,
        container_commands: Vec::new(),
        sign_commands: Vec::new(),
        inventory_armor_state: Default::default(),
        inventory_redirect_pending: false,
    }
}

pub fn check(path: &str) -> bool {
    let mut out = Vec::new();
    let mut r = worldgen::JavaRandom::new(181);
    for _ in 0..10000 {
        let scale = r.next_int(7) - 1;
        let w = r.next_int(8193);
        let h = r.next_int(4097);
        let s = ScaledResolution::new(scale, w, h);
        for v in [s.width, s.height, s.scale_factor] {
            out.extend(v.to_le_bytes());
        }
        for v in [s.width_d, s.height_d] {
            out.extend(v.to_le_bytes());
        }
    }
    for c in 0..10000 {
        let x = r.next_int(1000) - 500;
        let y = r.next_int(1000) - 500;
        let w = r.next_int(250);
        let h = r.next_int(50);
        let mx = x + r.next_int(260) - 5;
        let my = y + r.next_int(60) - 5;
        let mut b = GuiButton::sized(c, x, y, w, h, "label");
        b.enabled = r.next_bool();
        b.visible = r.next_bool();
        out.push(b.inside(mx, my) as u8);
        out.extend(
            b.hover_state(mx >= x && my >= y && mx < x + w && my < y + h)
                .to_le_bytes(),
        );
    }
    let mut jar = crate::assets::Jar::open().unwrap();
    let image = jar.image("font/default.png").unwrap();
    let allowed = jar.text("font.txt").unwrap();
    let font = FontRenderer::new(&image, 0, &allowed);
    for text in [
        "",
        " ",
        "   ",
        "Minecraft Beta 1.8.1",
        "one two three four five",
        "one\n\ntwo\n",
        "\n",
        "\n\n",
        "longunbrokentextwithoutspaces",
        "§aGreen §rReset",
        "§",
        "§zbad",
        "1234567890 !?",
        "éßñå",
        "  leading trailing  ",
    ] {
        out.extend(font.get_string_width(text).to_le_bytes());
        for w in [10, 11, 20, 40, 80, 120, 200] {
            out.extend(font.split_string_height(text, w).to_le_bytes());
        }
    }
    for c in 0..256 {
        out.extend(
            font.get_string_width(&char::from_u32(c).unwrap().to_string())
                .to_le_bytes(),
        );
    }
    let expected = std::fs::read(path).unwrap();
    std::fs::write("target/guiutility181-rust.bin", &out).unwrap();
    if out == expected {
        println!("GUI UTILITIES EXACT: 10000 scales, 10000 buttons, 376 font cases");
        true
    } else {
        eprintln!(
            "GUI UTILITIES DIFFER byte {}",
            out.iter()
                .zip(expected)
                .position(|(a, b)| *a != b)
                .unwrap_or(out.len())
        );
        false
    }
}
/// Exhaustive original-bytecode width comparison for BMP and isolated UTF-16 units.
#[test]
#[ignore = "requires original 1.1 font draw and width fixtures"]
fn complete_font_primitives_match_original_11() {
    let base = std::env::var("FONT_PRIMITIVES11_DIRECTORY").unwrap();
    assert!(check_font_metrics11(&format!("{base}/fontmetrics11.bin")));
    assert!(check_font_draw11(&format!("{base}/fontdraw11.bin")));
}

pub fn check_font_metrics11(path: &str) -> bool {
    let mut jar = crate::assets::Jar::open().unwrap();
    let image = jar.image("font/default.png").unwrap();
    let allowed = jar.text("font.txt").unwrap();
    let mut font = FontRenderer::new(&image, 0, &allowed);
    let mut actual = Vec::new();
    for unicode in [false, true] {
        font.unicode = unicode;
        for unit in 0..=65535 { actual.extend_from_slice(&font.get_utf16_width(&[unit]).to_be_bytes()); }
        for text in ["", " ", "§", "§kAB§cC", "hello\u{0971}\u{09F7}\u{0C79}\u{1161}"] {
            actual.extend_from_slice(&font.get_string_width(text).to_be_bytes());
        }
        actual.extend_from_slice(&font.get_utf16_width(&[0xD800,0xDC00,0xD800]).to_be_bytes());
    }
    let original = std::fs::read(path).unwrap();
    if actual == original {
        println!("EXACT: 131072 original UTF-16 glyph widths plus formatting and surrogate strings"); true
    } else {
        println!("DIFFERS: font metrics byte {:?}; lengths {}/{}", actual.iter().zip(&original).position(|(a,b)|a!=b), actual.len(), original.len()); false
    }
}

/// Compare original 1.1 immediate font output for every UTF-16 unit and RNG-sensitive strings.
pub fn check_font_draw11(path: &str) -> bool {
    let mut jar=crate::assets::Jar::open().unwrap();
    let image=jar.image("font/default.png").unwrap();
    let allowed=jar.text("font.txt").unwrap();
    let mut font=FontRenderer::new(&image,0,&allowed);
    let mut out=Vec::new();
    fn run(font:&FontRenderer,out:&mut Vec<u8>,text:&[u16],x:i32,y:i32,color:i32,shadow:bool,seed:i64) {
        *font.random.lock().unwrap()=worldgen::JavaRandom::new(seed);
        let mut gl=super::gl::Gl::new();
        let end=font.render_utf16(&mut gl,text,x,y,color,shadow);
        out.extend(((gl.draws.len()*4) as i32).to_be_bytes());
        for draw in &gl.draws {
            assert_eq!(draw.count,6);
            let texture=match draw.state.texture.unwrap(){0=>256,t=>{assert_eq!(t&0xffff_ff00,0x8000_0000);(t&255) as i32}};
            for i in [0,1,2,5] {
                out.extend(texture.to_be_bytes());
                let v=&gl.verts[draw.first as usize+i];
                for f in v.pos.into_iter().chain(v.uv).chain(v.color) {out.extend(f.to_bits().to_be_bytes());}
            }
        }
        for f in gl.color {out.extend(f.to_bits().to_be_bytes());}
        out.extend(end.to_bits().to_be_bytes());
        let random_tail=font.random.lock().unwrap().next_long();
        out.extend(random_tail.to_be_bytes());
        if !shadow {
            *font.random.lock().unwrap()=worldgen::JavaRandom::new(seed);
            let mut fixed=crate::render::entity::fixed::Fixed::new(1.0);
            font.draw_fixed_utf16(&mut fixed,text,x,y,color);
            assert_eq!(fixed.draws.len(),gl.draws.len());
            for (f,d) in fixed.draws.iter().zip(&gl.draws) {
                assert_eq!(f.mode,5);
                assert_eq!(f.vertices.len(),4);
                assert_eq!(f.material,gl.verts[d.first as usize].color);
                assert_eq!(f.texture,match d.state.texture.unwrap(){0=>14,t=>t});
                for (v,i) in f.vertices.iter().zip([0,1,2,5]) {
                    let reference=&gl.verts[d.first as usize+i];
                    assert_eq!(v.pos,reference.pos);assert_eq!(v.uv,reference.uv);
                }
            }
            assert_eq!(fixed.material,gl.color);
            assert_eq!(font.random.lock().unwrap().next_long(),random_tail);
        }
    }
    for unicode in [false,true] {
        font.unicode=unicode;
        for c in 0..=u16::MAX {run(&font,&mut out,&[c],-123,37,0x02abcdef,false,c as i64);}
        for s in [""," ","ABC xyz","§kHELLO WORLD!","§aA§kBC§zD§","§KK§rE","اא日😀","\0§\0"] {
            for color in [0,0x01abcdef,0x02abcdef,0x03abcdef,0x04abcdef,0x7f102030,0xffabcdefu32 as i32] {
                for shadow in [false,true] {run(&font,&mut out,&s.encode_utf16().collect::<Vec<_>>(),i32::MAX,i32::MIN,color,shadow,4181);}
            }
        }
    }
    let expected=std::fs::read(path).unwrap();
    std::fs::write("target/r11/fontdraw11-rust.bin",&out).unwrap();
    if out==expected {println!("EXACT: 131072 UTF-16 font draws plus 224 formatting/alpha/RNG cases ({} bytes)",out.len());true}
    else {eprintln!("FONT DRAW DIFFERS at {} lengths {}/{}",out.iter().zip(&expected).position(|(a,b)|a!=b).unwrap_or(out.len().min(expected.len())),out.len(),expected.len());false}
}
