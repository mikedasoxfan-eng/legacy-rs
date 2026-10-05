//! GuiEditSign, including its deliberate lack of Escape handling.
use crate::gui::{Screen, ScreenBase, Ui, widgets::GuiButton};
pub struct GuiEditSign {
    base: ScreenBase,
    pub editor: worldgen::sign::Editor,
    position: (i32, i32, i32),
    block: u8,
    metadata: i32,
}
impl GuiEditSign {
    pub fn new(
        position: (i32, i32, i32),
        block: u8,
        metadata: i32,
        sign: worldgen::sign::Sign,
    ) -> Self {
        Self {
            base: Default::default(),
            editor: worldgen::sign::Editor {
                sign,
                ..Default::default()
            },
            position,
            block,
            metadata,
        }
    }
}
impl Screen for GuiEditSign {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }
    fn init_gui(&mut self, _ui: &mut Ui) {
        self.base.buttons.clear();
        self.base.buttons.push(GuiButton::new(
            0,
            self.base.width / 2 - 100,
            self.base.height / 4 + 120,
            "Done",
        ));
    }
    fn update_screen(&mut self, _ui: &mut Ui) {
        self.editor.update_counter = self.editor.update_counter.wrapping_add(1);
    }
    fn key_typed(&mut self, ui: &mut Ui, c: char, key: i32) {
        self.editor.key_typed(c, key, ui.font_allows(c));
    }
    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if id == 0 {
            ui.display(None);
        }
    }
    fn on_gui_closed(&mut self, ui: &mut Ui) {
        ui.sign_commands
            .push((self.position, self.editor.sign.text.clone()));
    }
    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        ui.draw_centered_string("Edit sign message:", w / 2, 40, 16777215);
        ui.gl.push_matrix();
        ui.gl.translatef((w / 2) as f32, 0.0, 50.0);
        ui.gl.scalef(-93.75, -93.75, -93.75);
        ui.gl.rotatef(180.0, 0.0, 1.0, 0.0);
        ui.gl.rotatef(
            crate::render::sign::angle(self.block, self.metadata),
            0.0,
            1.0,
            0.0,
        );
        ui.gl.translatef(0.0, -1.0625, 0.0);
        let scale = 2.0f32 / 3.0;
        ui.gl.push_matrix();
        ui.gl.translatef(0.0, -0.75 + 12.0 / 16.0 * scale, 0.0);
        ui.gl.rotatef(
            -crate::render::sign::angle(self.block, self.metadata),
            0.0,
            1.0,
            0.0,
        );
        if self.block != 63 {
            ui.gl.translatef(0.0, -5.0 / 16.0, -7.0 / 16.0);
        }
        ui.gl.bind_texture(ui.texture("item/sign.png"));
        ui.gl.push_matrix();
        ui.gl.scalef(scale, -scale, -scale);
        for (i, part) in crate::render::sign::models().iter().enumerate() {
            if i == 1 && self.block != 63 {
                continue;
            }
            let mut vertices = Vec::new();
            for face in &part.faces {
                for i in [0, 1, 2, 0, 2, 3] {
                    let v = face.vertices[i];
                    vertices.push(crate::render::entity::living::EntityVertex {
                        pos: v.pos.map(|n| n as f32 / 16.0),
                        uv: v.uv,
                        color: ui.gl.color,
                    });
                }
            }
            ui.gl.model_triangles(&vertices, ui.gl.modelview_matrix());
        }
        ui.gl.pop_matrix();
        let s = (1.0f64 / 60.0) as f32 * scale;
        ui.gl.translatef(0.0, 0.5 * scale, 0.07 * scale);
        ui.gl.scalef(s, -s, s);
        ui.gl.depth_mask = false;
        for (i, text) in self.editor.sign.text.iter().enumerate() {
            let text = if i as i32 == self.editor.blinking_line() {
                format!("> {text} <")
            } else {
                text.clone()
            };
            ui.font.draw_string(
                &mut ui.gl,
                &text,
                -ui.font.get_string_width(&text) / 2,
                i as i32 * 10 - 20,
                0,
            );
        }
        ui.gl.depth_mask = true;
        ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
        ui.gl.pop_matrix();
        ui.gl.pop_matrix();
        self.draw_buttons(ui, mx, my);
    }
}
