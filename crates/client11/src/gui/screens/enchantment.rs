//! GuiEnchantment's GUI state. All offers and item mutations remain world-owned.
use worldgen::{container::ItemStack, JavaRandom};

pub struct EnchantmentAnimation {
    pub ticks: i32,
    pub page: f32,
    pub previous_page: f32,
    pub target: f32,
    pub velocity: f32,
    pub opening: f32,
    pub previous_opening: f32,
    previous_item: Option<ItemStack>,
    pub random: JavaRandom,
}
impl EnchantmentAnimation {
    pub fn new(seed: i64) -> Self {
        Self {
            ticks: 0,
            page: 0.0,
            previous_page: 0.0,
            target: 0.0,
            velocity: 0.0,
            opening: 0.0,
            previous_opening: 0.0,
            previous_item: None,
            random: JavaRandom::new(seed),
        }
    }
    pub fn tick(&mut self, item: Option<ItemStack>, costs: [i32; 3]) {
        if item
            .zip(self.previous_item)
            .is_some_and(|(a, b)| a.same_object(b))
        {
            self.previous_item = item;
        }
        if !stacks_equal(item, self.previous_item) {
            // Vanilla retains the stack reference, including subsequent mutations.
            self.previous_item = item;
            loop {
                self.target += (self.random.next_int(4) - self.random.next_int(4)) as f32;
                if !(self.page <= self.target + 1.0 && self.page >= self.target - 1.0) {
                    break;
                }
            }
        }
        self.ticks = self.ticks.wrapping_add(1);
        self.previous_page = self.page;
        self.previous_opening = self.opening;
        if costs.iter().any(|&c| c != 0) {
            self.opening += 0.2;
        } else {
            self.opening -= 0.2;
        }
        if self.opening < 0.0 {
            self.opening = 0.0;
        }
        if self.opening > 1.0 {
            self.opening = 1.0;
        }
        let mut change = (self.target - self.page) * 0.4;
        if change < -0.2 {
            change = -0.2;
        }
        if change > 0.2 {
            change = 0.2;
        }
        self.velocity += (change - self.velocity) * 0.9;
        self.page += self.velocity;
    }
    /// Parameters passed to the render worker's ModelBook.
    pub fn pose(&self, partial: f32) -> [f32; 3] {
        let opening = self.previous_opening + (self.opening - self.previous_opening) * partial;
        let page = self.previous_page + (self.page - self.previous_page) * partial;
        let fold = |offset: f32| {
            let x = page + offset;
            let x = (x - worldgen::math::floor_double(x as f64) as f32) * 1.6 - 0.3;
            x.clamp(0.0, 1.0)
        };
        [fold(0.25), fold(0.75), opening]
    }
}
fn stacks_equal(a: Option<ItemStack>, b: Option<ItemStack>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            a.id == b.id && a.count == b.count && a.damage == b.damage && a.tags_equal(&b)
        }
        _ => false,
    }
}
pub fn names(seed: i64) -> [String; 3] {
    let mut names = worldgen::enchantment::EnchantmentNameParts::new(seed);
    std::array::from_fn(|_| names.next_name())
}
pub fn offer(cost: i32, level: i32, hovered: bool) -> (i32, i32, i32) {
    if cost == 0 {
        (185, 0, 0)
    } else if level < cost {
        (185, (6839882 & 16711422) >> 1, 4226832)
    } else if hovered {
        (204, 16777088, 8453920)
    } else {
        (166, 6839882, 8453920)
    }
}
use glam::{Mat4, Vec3};
use crate::gui::Ui;

/// Original GuiEnchantment camera and model inputs. ModelBook is shared with the world renderer.
pub(crate) fn book_inputs(animation: &EnchantmentAnimation, partial: f32, scaled: crate::gui::ScaledResolution) -> ([i32; 4], Vec<[f32; 5]>, [f32; 6]) {
    let [a, b, opening] = animation.pose(partial);
    let viewport = [(scaled.width - 320) / 2 * scaled.scale_factor, (scaled.height - 240) / 2 * scaled.scale_factor, 320 * scaled.scale_factor, 240 * scaled.scale_factor];
    let ops = vec![
        [0.0, 0.0, 3.3, -16.0, 0.0],
        [1.0, 1.0, 1.0, 1.0, 0.0],
        [1.0, 5.0, 5.0, 5.0, 0.0],
        [2.0, 180.0, 0.0, 0.0, 1.0],
        [2.0, 20.0, 1.0, 0.0, 0.0],
        [0.0, (1.0-opening)*0.2, (1.0-opening)*0.1, (1.0-opening)*0.25, 0.0],
        [2.0, -(1.0-opening)*90.0-90.0, 0.0, 1.0, 0.0],
        [2.0, 180.0, 1.0, 0.0, 0.0],
    ];
    (viewport, ops, [0.0, a, b, opening, 0.0, 0.0625])
}
pub(crate) fn draw_book(ui: &mut Ui, animation: &EnchantmentAnimation, partial: f32) {
    use crate::render::entity::models::{Model, ModelKind};
    let (vp, ops, pose) = book_inputs(animation, partial, ui.scaled());
    let mut parent = Mat4::IDENTITY;
    for [op,a,b,c,d] in ops {
        let matrix = match op as i32 {
            0 => Mat4::from_translation(Vec3::new(a,b,c)),
            1 => Mat4::from_scale(Vec3::new(a,b,c)),
            _ => Mat4::from_axis_angle(Vec3::new(b,c,d), a*std::f32::consts::PI/180.0),
        };
        parent *= matrix;
    }
    // Express the original small GL viewport in the full framebuffer's clip coordinates.
    let width = ui.display_width as f32;
    let height = ui.display_height as f32;
    let viewport = Mat4::from_translation(Vec3::new((2*vp[0]+vp[2]) as f32/width-1.0, (2*vp[1]+vp[3]) as f32/height-1.0, 0.0))
        * Mat4::from_scale(Vec3::new(vp[2] as f32/width, vp[3] as f32/height, 1.0));
    let old_projection = ui.gl.proj;
    ui.gl.push_matrix();
    ui.gl.set_modelview(Mat4::IDENTITY);
    ui.gl.proj = viewport * Mat4::from_translation(Vec3::new(-0.34,0.23,0.0)) * Mat4::from_cols_array(&[0.75,0.0,0.0,0.0, 0.0,1.0,0.0,0.0, 0.0,0.0,-89.0/71.0,-1.0, 0.0,0.0,-1440.0/71.0,0.0]);
    let mut model = Model::new(ModelKind::Book);
    model.set_rotation_angles(pose);
    ui.gl.bind_texture(ui.texture("item/book.png"));
    let lights = [super::inventory_model::light(0.2,-0.7), super::inventory_model::light(-0.2,0.7)];
    for (part, rotation) in model.render_parts() { super::inventory_model::part(ui, part, parent, rotation, lights, None, true); }
    ui.gl.proj = old_projection;
    ui.gl.pop_matrix();
    ui.gl.color4f(1.0,1.0,1.0,1.0);
}
/// GuiEnchantment's ordered rectangle and font calls (galactic split text, then level labels).
pub(crate) fn offer_draws(seed:i64,costs:[i32;3],level:i32,x:i32,y:i32,mx:i32,my:i32,width:impl Fn(&str)->i32)->(Vec<[i32;6]>,Vec<(String,[i32;5])>) {
    let mut rectangles=Vec::new();let mut texts=Vec::new();
    for (n,name) in names(seed).into_iter().enumerate() {
        let row=19*n as i32;let cost=costs[n];let hovered=mx-x-60>=0&&mx-x-60<108&&my-y-14-row>=0&&my-y-14-row<19;
        let (v,color,level_color)=offer(cost,level,hovered);rectangles.push([x+60,y+14+row,0,v,108,19]);
        if cost!=0 {let text=cost.to_string();texts.push((name,[2,x+62,y+16+row,104,color]));texts.push((text.clone(),[1,x+166-width(&text),y+23+row,0,level_color]));}
    }
    (rectangles,texts)
}
