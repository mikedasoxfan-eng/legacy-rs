//! RenderItem GUI entry points, ready for hotbar/inventory callers.
use super::{fixed::Fixed,inventory,item_table};
use worldgen::world::tile::ItemStack;
use crate::gui::Ui;

pub fn geometry(item:&ItemStack,x:i32,y:i32,tint:bool)->Fixed {
    let mut g=Fixed::new(1.0);let (icon,color,_)=item_table::properties(item.id,item.damage);
    if inventory::in_3d(item.id) {
        g.texture=1;g.push();g.translate((x-2) as f32,(y+3) as f32,-3.0);g.scale(10.0,10.0,10.0);
        g.translate(1.0,0.5,1.0);g.scale(1.0,1.0,-1.0);g.rotate(210.0,1.0,0.0,0.0);g.rotate(45.0,0.0,1.0,0.0);
        if tint {g.material=[(color>>16&255) as f32/255.0,(color>>8&255) as f32/255.0,(color&255) as f32/255.0,1.0];}
        g.rotate(-90.0,0.0,1.0,0.0);inventory::inventory(&mut g,item.id as u8,item.damage as i32,1.0,tint);g.pop();
    } else if icon>=0 {
        g.enable(1,false);g.texture=if item.id<256{1}else{2};
        if tint {g.material=[(color>>16&255) as f32/255.0,(color>>8&255) as f32/255.0,(color&255) as f32/255.0,1.0];}
        let u=icon%16*16;let v=icon/16*16;g.begin();
        g.quad([[x as f32,(y+16) as f32,0.0],[(x+16) as f32,(y+16) as f32,0.0],[(x+16) as f32,y as f32,0.0],[x as f32,y as f32,0.0]],
            [[u as f32/256.0,(v+16) as f32/256.0],[(u+16) as f32/256.0,(v+16) as f32/256.0],[(u+16) as f32/256.0,v as f32/256.0],[u as f32/256.0,v as f32/256.0]]);
        g.end();g.enable(1,true);
    }
    g
}
/// Vanilla's three durability rectangles. Counts use the GUI's existing FontRenderer.
pub fn durability(item:&ItemStack,x:i32,y:i32)->Fixed {
    let mut g=Fixed::new(1.0);let (_,_,max)=item_table::properties(item.id,item.damage);
    if item.damage>0 && max>0 {
        let damage=item.damage as f64;
        let width=(13.0-damage*13.0/max as f64+0.5).floor() as i32;
        let green=(255.0-damage*255.0/max as f64+0.5).floor() as i32;
        g.enable(1,false);g.enable(8,false);g.enable(256,false);
        for (w,h,c) in [(13,2,0),(12,1,((255-green)/4<<16)|16128),(width,1,((255-green)<<16)|(green<<8))] {
            g.begin();g.color([(c>>16&255) as f32/255.0,(c>>8&255) as f32/255.0,(c&255) as f32/255.0,1.0]);
            g.quad([[(x+2) as f32,(y+13) as f32,0.0],[(x+2) as f32,(y+13+h) as f32,0.0],[(x+2+w) as f32,(y+13+h) as f32,0.0],[(x+2+w) as f32,(y+13) as f32,0.0]],[[0.0;2];4]);g.untextured();g.end();
        }
    }
    g
}
fn submit(ui:&mut Ui,g:Fixed) {
    for d in &g.draws {
        let batch=d.batch();ui.gl.texture_2d=batch.texture.is_some();
        if let Some(tex)=batch.texture {ui.gl.bind_texture(ui.texture(tex));}
        ui.gl.triangles(&batch.vertices);
    }
    ui.gl.color=g.material;
}
#[allow(dead_code)]
pub fn render_item_into_gui(ui:&mut Ui,item:Option<&ItemStack>,x:i32,y:i32) {
    if let Some(item)=item {submit(ui,geometry(item,x,y,true));}
}
#[allow(dead_code)]
pub fn render_item_overlay_into_gui(ui:&mut Ui,item:Option<&ItemStack>,x:i32,y:i32) {
    if let Some(item)=item {
        if item.count>1 {
            let text=item.count.to_string();ui.gl.depth_test=false;
            ui.font.draw_string_with_shadow(&mut ui.gl,&text,x+17-ui.font.get_string_width(&text),y+9,0xffffff);
            ui.gl.depth_test=true;
        }
        let g=durability(item,x,y);
        if !g.draws.is_empty() {
            ui.gl.depth_test=false;submit(ui,g);ui.gl.texture_2d=true;ui.gl.depth_test=true;ui.gl.color=[1.0;4];
        }
    }
}
