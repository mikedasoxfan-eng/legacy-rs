// Literal destruction-only RenderBlocks geometry. The oracle checks raw bits.
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_parens,
    unused_assignments
)]
impl<'a, 'b> RenderBlocks<'a, 'b> {
    fn damage_bounds(&mut self, b: [f32; 6]) {
        self.b = Bounds {
            min_x: b[0] as f64,
            min_y: b[1] as f64,
            min_z: b[2] as f64,
            max_x: b[3] as f64,
            max_y: b[4] as f64,
            max_z: b[5] as f64,
        };
    }
    fn damage_native_texture(&self, id: u8, x: i32, y: i32, z: i32, side: i32) -> i32 {
        let m = self.w.meta(x, y, z);
        if matches!(id, 64 | 71) {
            let base = if id == 64 { 97 } else { 98 };
            if side < 2 {
                return base;
            }
            let state = if m & 4 == 0 { (m - 1) & 3 } else { m & 3 };
            if ((state == 0 || state == 2) ^ (side <= 3)) {
                return base;
            }
            let flip = state / 2 + ((side & 1) ^ state) + (m & 4) / 4;
            let tex = base - (m & 8) * 2;
            return if flip & 1 != 0 { -tex } else { tex };
        }
        rprops(id).textures[m as usize][side as usize] as i32
    }
    fn damage_wire_connect(&self, x: i32, y: i32, z: i32, side: i32) -> bool {
        let id = self.w.block(x, y, z);
        if id == 55 || worldgen::entity::block_hooks::provides_power(id) {
            return true;
        }
        if matches!(id, 93 | 94) {
            return side == [2, 3, 0, 1][(self.w.meta(x, y, z) & 3) as usize];
        }
        false
    }
    fn damage_torch(&mut self, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) -> bool {
        let mut var5: i32 = self.w.meta(var2, var3, var4);

        let mut var7: f32 = 1.0f32;
        if ((worldgen::block::light_value(var1 as u8) as i32) > 0) {
            var7 = 1.0f32;
        }
        ();
        let mut var8: f64 = (0.4f32 as f64);
        let mut var10: f64 = (0.5f64 - var8);
        let mut var12: f64 = (0.2f32 as f64);
        if (var5 == 1) {
            self.damage_torch_angle(
                var1,
                ((var2 as f64) - var10),
                ((var3 as f64) + var12),
                (var4 as f64),
                (-var8),
                0.0f64,
            );
        } else if (var5 == 2) {
            self.damage_torch_angle(
                var1,
                ((var2 as f64) + var10),
                ((var3 as f64) + var12),
                (var4 as f64),
                var8,
                0.0f64,
            );
        } else if (var5 == 3) {
            self.damage_torch_angle(
                var1,
                (var2 as f64),
                ((var3 as f64) + var12),
                ((var4 as f64) - var10),
                0.0f64,
                (-var8),
            );
        } else if (var5 == 4) {
            self.damage_torch_angle(
                var1,
                (var2 as f64),
                ((var3 as f64) + var12),
                ((var4 as f64) + var10),
                0.0f64,
                var8,
            );
        } else {
            self.damage_torch_angle(
                var1,
                (var2 as f64),
                (var3 as f64),
                (var4 as f64),
                0.0f64,
                0.0f64,
            );
        }
        return true;
    }
    fn damage_torch_angle(
        &mut self,
        mut var1: i32,
        mut var2: f64,
        mut var4: f64,
        mut var6: f64,
        mut var8: f64,
        mut var10: f64,
    ) {
        let mut var13: i32 = (rprops(var1 as u8).textures[0][0 as usize] as i32);
        if (self.override_texture.unwrap() >= 0) {
            var13 = self.override_texture.unwrap();
        }
        let mut var14: i32 = ((var13 & 15) << 4);
        let mut var15: i32 = (var13 & 240);
        let mut var16: f32 = ((var14 as f32) / 256.0f32);
        let mut var17: f32 = (((var14 as f32) + 15.99f32) / 256.0f32);
        let mut var18: f32 = ((var15 as f32) / 256.0f32);
        let mut var19: f32 = (((var15 as f32) + 15.99f32) / 256.0f32);
        let mut var20: f64 = ((var16 as f64) + (1.75f64 / 64.0f64));
        let mut var22: f64 = ((var18 as f64) + (6.0f64 / 256.0f64));
        let mut var24: f64 = ((var16 as f64) + (9.0f64 / 256.0f64));
        let mut var26: f64 = ((var18 as f64) + (1.0f64 / 32.0f64));
        var2 += 0.5f64;
        var6 += 0.5f64;
        let mut var28: f64 = (var2 - 0.5f64);
        let mut var30: f64 = (var2 + 0.5f64);
        let mut var32: f64 = (var6 - 0.5f64);
        let mut var34: f64 = (var6 + 0.5f64);
        let mut var36: f64 = (1.0f64 / 16.0f64);
        let mut var38: f64 = 0.625f64;
        self.t.vertex_uv(
            ((var2 + (var8 * (1.0f64 - var38))) - var36),
            (var4 + var38),
            ((var6 + (var10 * (1.0f64 - var38))) - var36),
            var20,
            var22,
        );
        self.t.vertex_uv(
            ((var2 + (var8 * (1.0f64 - var38))) - var36),
            (var4 + var38),
            ((var6 + (var10 * (1.0f64 - var38))) + var36),
            var20,
            var26,
        );
        self.t.vertex_uv(
            ((var2 + (var8 * (1.0f64 - var38))) + var36),
            (var4 + var38),
            ((var6 + (var10 * (1.0f64 - var38))) + var36),
            var24,
            var26,
        );
        self.t.vertex_uv(
            ((var2 + (var8 * (1.0f64 - var38))) + var36),
            (var4 + var38),
            ((var6 + (var10 * (1.0f64 - var38))) - var36),
            var24,
            var22,
        );
        self.t.vertex_uv(
            (var2 - var36),
            (var4 + 1.0f64),
            var32,
            (var16 as f64),
            (var18 as f64),
        );
        self.t.vertex_uv(
            ((var2 - var36) + var8),
            (var4 + 0.0f64),
            (var32 + var10),
            (var16 as f64),
            (var19 as f64),
        );
        self.t.vertex_uv(
            ((var2 - var36) + var8),
            (var4 + 0.0f64),
            (var34 + var10),
            (var17 as f64),
            (var19 as f64),
        );
        self.t.vertex_uv(
            (var2 - var36),
            (var4 + 1.0f64),
            var34,
            (var17 as f64),
            (var18 as f64),
        );
        self.t.vertex_uv(
            (var2 + var36),
            (var4 + 1.0f64),
            var34,
            (var16 as f64),
            (var18 as f64),
        );
        self.t.vertex_uv(
            ((var2 + var8) + var36),
            (var4 + 0.0f64),
            (var34 + var10),
            (var16 as f64),
            (var19 as f64),
        );
        self.t.vertex_uv(
            ((var2 + var8) + var36),
            (var4 + 0.0f64),
            (var32 + var10),
            (var17 as f64),
            (var19 as f64),
        );
        self.t.vertex_uv(
            (var2 + var36),
            (var4 + 1.0f64),
            var32,
            (var17 as f64),
            (var18 as f64),
        );
        self.t.vertex_uv(
            var28,
            (var4 + 1.0f64),
            (var6 + var36),
            (var16 as f64),
            (var18 as f64),
        );
        self.t.vertex_uv(
            (var28 + var8),
            (var4 + 0.0f64),
            ((var6 + var36) + var10),
            (var16 as f64),
            (var19 as f64),
        );
        self.t.vertex_uv(
            (var30 + var8),
            (var4 + 0.0f64),
            ((var6 + var36) + var10),
            (var17 as f64),
            (var19 as f64),
        );
        self.t.vertex_uv(
            var30,
            (var4 + 1.0f64),
            (var6 + var36),
            (var17 as f64),
            (var18 as f64),
        );
        self.t.vertex_uv(
            var30,
            (var4 + 1.0f64),
            (var6 - var36),
            (var16 as f64),
            (var18 as f64),
        );
        self.t.vertex_uv(
            (var30 + var8),
            (var4 + 0.0f64),
            ((var6 - var36) + var10),
            (var16 as f64),
            (var19 as f64),
        );
        self.t.vertex_uv(
            (var28 + var8),
            (var4 + 0.0f64),
            ((var6 - var36) + var10),
            (var17 as f64),
            (var19 as f64),
        );
        self.t.vertex_uv(
            var28,
            (var4 + 1.0f64),
            (var6 - var36),
            (var17 as f64),
            (var18 as f64),
        );
    }
    fn damage_ladder(
        &mut self,
        mut var1: i32,
        mut var2: i32,
        mut var3: i32,
        mut var4: i32,
    ) -> bool {
        let mut var6: i32 = (rprops(var1 as u8).textures[0][0 as usize] as i32);
        if (self.override_texture.unwrap() >= 0) {
            var6 = self.override_texture.unwrap();
        }
        let mut var7: f32 = 1.0f32;
        ();
        let mut var8: i32 = ((var6 & 15) << 4);
        let mut var9: i32 = (var6 & 240);
        let mut var10: f64 = (((var8 as f32) / 256.0f32) as f64);
        let mut var12: f64 = ((((var8 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var14: f64 = (((var9 as f32) / 256.0f32) as f64);
        let mut var16: f64 = ((((var9 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var18: i32 = self.w.meta(var2, var3, var4);
        let mut var19: f32 = 0.0f32;
        let mut var20: f32 = 0.05f32;
        if (var18 == 5) {
            self.t.vertex_uv(
                (((var2 as f32) + var20) as f64),
                ((((var3 + 1) as f32) + var19) as f64),
                ((((var4 + 1) as f32) + var19) as f64),
                var10,
                var14,
            );
            self.t.vertex_uv(
                (((var2 as f32) + var20) as f64),
                ((((var3 + 0) as f32) - var19) as f64),
                ((((var4 + 1) as f32) + var19) as f64),
                var10,
                var16,
            );
            self.t.vertex_uv(
                (((var2 as f32) + var20) as f64),
                ((((var3 + 0) as f32) - var19) as f64),
                ((((var4 + 0) as f32) - var19) as f64),
                var12,
                var16,
            );
            self.t.vertex_uv(
                (((var2 as f32) + var20) as f64),
                ((((var3 + 1) as f32) + var19) as f64),
                ((((var4 + 0) as f32) - var19) as f64),
                var12,
                var14,
            );
        }
        if (var18 == 4) {
            self.t.vertex_uv(
                ((((var2 + 1) as f32) - var20) as f64),
                ((((var3 + 0) as f32) - var19) as f64),
                ((((var4 + 1) as f32) + var19) as f64),
                var12,
                var16,
            );
            self.t.vertex_uv(
                ((((var2 + 1) as f32) - var20) as f64),
                ((((var3 + 1) as f32) + var19) as f64),
                ((((var4 + 1) as f32) + var19) as f64),
                var12,
                var14,
            );
            self.t.vertex_uv(
                ((((var2 + 1) as f32) - var20) as f64),
                ((((var3 + 1) as f32) + var19) as f64),
                ((((var4 + 0) as f32) - var19) as f64),
                var10,
                var14,
            );
            self.t.vertex_uv(
                ((((var2 + 1) as f32) - var20) as f64),
                ((((var3 + 0) as f32) - var19) as f64),
                ((((var4 + 0) as f32) - var19) as f64),
                var10,
                var16,
            );
        }
        if (var18 == 3) {
            self.t.vertex_uv(
                ((((var2 + 1) as f32) + var19) as f64),
                ((((var3 + 0) as f32) - var19) as f64),
                (((var4 as f32) + var20) as f64),
                var12,
                var16,
            );
            self.t.vertex_uv(
                ((((var2 + 1) as f32) + var19) as f64),
                ((((var3 + 1) as f32) + var19) as f64),
                (((var4 as f32) + var20) as f64),
                var12,
                var14,
            );
            self.t.vertex_uv(
                ((((var2 + 0) as f32) - var19) as f64),
                ((((var3 + 1) as f32) + var19) as f64),
                (((var4 as f32) + var20) as f64),
                var10,
                var14,
            );
            self.t.vertex_uv(
                ((((var2 + 0) as f32) - var19) as f64),
                ((((var3 + 0) as f32) - var19) as f64),
                (((var4 as f32) + var20) as f64),
                var10,
                var16,
            );
        }
        if (var18 == 2) {
            self.t.vertex_uv(
                ((((var2 + 1) as f32) + var19) as f64),
                ((((var3 + 1) as f32) + var19) as f64),
                ((((var4 + 1) as f32) - var20) as f64),
                var10,
                var14,
            );
            self.t.vertex_uv(
                ((((var2 + 1) as f32) + var19) as f64),
                ((((var3 + 0) as f32) - var19) as f64),
                ((((var4 + 1) as f32) - var20) as f64),
                var10,
                var16,
            );
            self.t.vertex_uv(
                ((((var2 + 0) as f32) - var19) as f64),
                ((((var3 + 0) as f32) - var19) as f64),
                ((((var4 + 1) as f32) - var20) as f64),
                var12,
                var16,
            );
            self.t.vertex_uv(
                ((((var2 + 0) as f32) - var19) as f64),
                ((((var3 + 1) as f32) + var19) as f64),
                ((((var4 + 1) as f32) - var20) as f64),
                var12,
                var14,
            );
        }
        return true;
    }
    fn damage_rail(&mut self, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) -> bool {
        let mut var6: i32 = self.w.meta(var2, var3, var4);
        let mut var7: i32 = (rprops(var1 as u8).textures[var6 as usize & 15][0 as usize] as i32);
        if (self.override_texture.unwrap() >= 0) {
            var7 = self.override_texture.unwrap();
        }
        if matches!(var1, 27 | 28) {
            var6 &= 7;
        }
        let mut var8: f32 = 1.0f32;
        ();
        let mut var9: i32 = ((var7 & 15) << 4);
        let mut var10: i32 = (var7 & 240);
        let mut var11: f64 = (((var9 as f32) / 256.0f32) as f64);
        let mut var13: f64 = ((((var9 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var15: f64 = (((var10 as f32) / 256.0f32) as f64);
        let mut var17: f64 = ((((var10 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var19: f32 = (1.0f32 / 16.0f32);
        let mut var20: f32 = ((var2 + 1) as f32);
        let mut var21: f32 = ((var2 + 1) as f32);
        let mut var22: f32 = ((var2 + 0) as f32);
        let mut var23: f32 = ((var2 + 0) as f32);
        let mut var24: f32 = ((var4 + 0) as f32);
        let mut var25: f32 = ((var4 + 1) as f32);
        let mut var26: f32 = ((var4 + 1) as f32);
        let mut var27: f32 = ((var4 + 0) as f32);
        let mut var28: f32 = ((var3 as f32) + var19);
        let mut var29: f32 = ((var3 as f32) + var19);
        let mut var30: f32 = ((var3 as f32) + var19);
        let mut var31: f32 = ((var3 as f32) + var19);
        if ((((var6 != 1) && (var6 != 2)) && (var6 != 3)) && (var6 != 7)) {
            if (var6 == 8) {
                var21 = ((var2 + 0) as f32);
                var20 = var21;
                var23 = ((var2 + 1) as f32);
                var22 = var23;
                var27 = ((var4 + 1) as f32);
                var24 = var27;
                var26 = ((var4 + 0) as f32);
                var25 = var26;
            } else if (var6 == 9) {
                var23 = ((var2 + 0) as f32);
                var20 = var23;
                var22 = ((var2 + 1) as f32);
                var21 = var22;
                var25 = ((var4 + 0) as f32);
                var24 = var25;
                var27 = ((var4 + 1) as f32);
                var26 = var27;
            }
        } else {
            var23 = ((var2 + 1) as f32);
            var20 = var23;
            var22 = ((var2 + 0) as f32);
            var21 = var22;
            var25 = ((var4 + 1) as f32);
            var24 = var25;
            var27 = ((var4 + 0) as f32);
            var26 = var27;
        }
        if ((var6 != 2) && (var6 != 4)) {
            if ((var6 == 3) || (var6 == 5)) {
                var29 += 1.0;
                var30 += 1.0;
            }
        } else {
            var28 += 1.0;
            var31 += 1.0;
        }
        self.t
            .vertex_uv((var20 as f64), (var28 as f64), (var24 as f64), var13, var15);
        self.t
            .vertex_uv((var21 as f64), (var29 as f64), (var25 as f64), var13, var17);
        self.t
            .vertex_uv((var22 as f64), (var30 as f64), (var26 as f64), var11, var17);
        self.t
            .vertex_uv((var23 as f64), (var31 as f64), (var27 as f64), var11, var15);
        self.t
            .vertex_uv((var23 as f64), (var31 as f64), (var27 as f64), var11, var15);
        self.t
            .vertex_uv((var22 as f64), (var30 as f64), (var26 as f64), var11, var17);
        self.t
            .vertex_uv((var21 as f64), (var29 as f64), (var25 as f64), var13, var17);
        self.t
            .vertex_uv((var20 as f64), (var28 as f64), (var24 as f64), var13, var15);
        return true;
    }
    fn damage_crops(&mut self, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) -> bool {
        let mut var6: f32 = 1.0f32;
        ();
        self.damage_crop_planes(
            var1,
            self.w.meta(var2, var3, var4),
            (var2 as f64),
            (((var3 as f32) - (1.0f32 / 16.0f32)) as f64),
            (var4 as f64),
        );
        return true;
    }
    fn damage_crop_planes(
        &mut self,
        mut var1: i32,
        mut var2: i32,
        mut var3: f64,
        mut var5: f64,
        mut var7: f64,
    ) {
        let mut var10: i32 = (rprops(var1 as u8).textures[var2 as usize & 15][0 as usize] as i32);
        if (self.override_texture.unwrap() >= 0) {
            var10 = self.override_texture.unwrap();
        }
        let mut var11: i32 = ((var10 & 15) << 4);
        let mut var12: i32 = (var10 & 240);
        let mut var13: f64 = (((var11 as f32) / 256.0f32) as f64);
        let mut var15: f64 = ((((var11 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var17: f64 = (((var12 as f32) / 256.0f32) as f64);
        let mut var19: f64 = ((((var12 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var21: f64 = ((var3 + 0.5f64) - 0.25f64);
        let mut var23: f64 = ((var3 + 0.5f64) + 0.25f64);
        let mut var25: f64 = ((var7 + 0.5f64) - 0.5f64);
        let mut var27: f64 = ((var7 + 0.5f64) + 0.5f64);
        self.t
            .vertex_uv(var21, (var5 + 1.0f64), var25, var13, var17);
        self.t
            .vertex_uv(var21, (var5 + 0.0f64), var25, var13, var19);
        self.t
            .vertex_uv(var21, (var5 + 0.0f64), var27, var15, var19);
        self.t
            .vertex_uv(var21, (var5 + 1.0f64), var27, var15, var17);
        self.t
            .vertex_uv(var21, (var5 + 1.0f64), var27, var13, var17);
        self.t
            .vertex_uv(var21, (var5 + 0.0f64), var27, var13, var19);
        self.t
            .vertex_uv(var21, (var5 + 0.0f64), var25, var15, var19);
        self.t
            .vertex_uv(var21, (var5 + 1.0f64), var25, var15, var17);
        self.t
            .vertex_uv(var23, (var5 + 1.0f64), var27, var13, var17);
        self.t
            .vertex_uv(var23, (var5 + 0.0f64), var27, var13, var19);
        self.t
            .vertex_uv(var23, (var5 + 0.0f64), var25, var15, var19);
        self.t
            .vertex_uv(var23, (var5 + 1.0f64), var25, var15, var17);
        self.t
            .vertex_uv(var23, (var5 + 1.0f64), var25, var13, var17);
        self.t
            .vertex_uv(var23, (var5 + 0.0f64), var25, var13, var19);
        self.t
            .vertex_uv(var23, (var5 + 0.0f64), var27, var15, var19);
        self.t
            .vertex_uv(var23, (var5 + 1.0f64), var27, var15, var17);
        var21 = ((var3 + 0.5f64) - 0.5f64);
        var23 = ((var3 + 0.5f64) + 0.5f64);
        var25 = ((var7 + 0.5f64) - 0.25f64);
        var27 = ((var7 + 0.5f64) + 0.25f64);
        self.t
            .vertex_uv(var21, (var5 + 1.0f64), var25, var13, var17);
        self.t
            .vertex_uv(var21, (var5 + 0.0f64), var25, var13, var19);
        self.t
            .vertex_uv(var23, (var5 + 0.0f64), var25, var15, var19);
        self.t
            .vertex_uv(var23, (var5 + 1.0f64), var25, var15, var17);
        self.t
            .vertex_uv(var23, (var5 + 1.0f64), var25, var13, var17);
        self.t
            .vertex_uv(var23, (var5 + 0.0f64), var25, var13, var19);
        self.t
            .vertex_uv(var21, (var5 + 0.0f64), var25, var15, var19);
        self.t
            .vertex_uv(var21, (var5 + 1.0f64), var25, var15, var17);
        self.t
            .vertex_uv(var23, (var5 + 1.0f64), var27, var13, var17);
        self.t
            .vertex_uv(var23, (var5 + 0.0f64), var27, var13, var19);
        self.t
            .vertex_uv(var21, (var5 + 0.0f64), var27, var15, var19);
        self.t
            .vertex_uv(var21, (var5 + 1.0f64), var27, var15, var17);
        self.t
            .vertex_uv(var21, (var5 + 1.0f64), var27, var13, var17);
        self.t
            .vertex_uv(var21, (var5 + 0.0f64), var27, var13, var19);
        self.t
            .vertex_uv(var23, (var5 + 0.0f64), var27, var15, var19);
        self.t
            .vertex_uv(var23, (var5 + 1.0f64), var27, var15, var17);
    }
    fn damage_fence(&mut self, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) -> bool {
        let mut var5: bool = false;
        let mut var6: f32 = (6.0f32 / 16.0f32);
        let mut var7: f32 = (10.0f32 / 16.0f32);
        self.damage_bounds([var6, 0.0f32, var6, var7, 1.0f32, var7]);
        self.render_standard_block(var1 as u8, var2, var3, var4);
        var5 = true;
        let mut var8: bool = false;
        let mut var9: bool = false;
        if (((self.w.block((var2 - 1), var3, var4) as i32) == var1)
            || ((self.w.block((var2 + 1), var3, var4) as i32) == var1))
        {
            var8 = true;
        }
        if (((self.w.block(var2, var3, (var4 - 1)) as i32) == var1)
            || ((self.w.block(var2, var3, (var4 + 1)) as i32) == var1))
        {
            var9 = true;
        }
        let mut var10: bool = ((self.w.block((var2 - 1), var3, var4) as i32) == var1);
        let mut var11: bool = ((self.w.block((var2 + 1), var3, var4) as i32) == var1);
        let mut var12: bool = ((self.w.block(var2, var3, (var4 - 1)) as i32) == var1);
        let mut var13: bool = ((self.w.block(var2, var3, (var4 + 1)) as i32) == var1);
        if ((!var8) && (!var9)) {
            var8 = true;
        }
        var6 = (7.0f32 / 16.0f32);
        var7 = (9.0f32 / 16.0f32);
        let mut var14: f32 = (12.0f32 / 16.0f32);
        let mut var15: f32 = (15.0f32 / 16.0f32);
        let mut var16: f32 = (if var10 { 0.0f32 } else { var6 });
        let mut var17: f32 = (if var11 { 1.0f32 } else { var7 });
        let mut var18: f32 = (if var12 { 0.0f32 } else { var6 });
        let mut var19: f32 = (if var13 { 1.0f32 } else { var7 });
        if var8 {
            self.damage_bounds([var16, var14, var6, var17, var15, var7]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            var5 = true;
        }
        if var9 {
            self.damage_bounds([var6, var14, var18, var7, var15, var19]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            var5 = true;
        }
        var14 = (6.0f32 / 16.0f32);
        var15 = (9.0f32 / 16.0f32);
        if var8 {
            self.damage_bounds([var16, var14, var6, var17, var15, var7]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            var5 = true;
        }
        if var9 {
            self.damage_bounds([var6, var14, var18, var7, var15, var19]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            var5 = true;
        }
        self.damage_bounds([0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32]);
        return var5;
    }
    fn damage_stairs(
        &mut self,
        mut var1: i32,
        mut var2: i32,
        mut var3: i32,
        mut var4: i32,
    ) -> bool {
        let mut var5: bool = false;
        let mut var6: i32 = self.w.meta(var2, var3, var4);
        if (var6 == 0) {
            self.damage_bounds([0.0f32, 0.0f32, 0.0f32, 0.5f32, 0.5f32, 1.0f32]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            self.damage_bounds([0.5f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            var5 = true;
        } else if (var6 == 1) {
            self.damage_bounds([0.0f32, 0.0f32, 0.0f32, 0.5f32, 1.0f32, 1.0f32]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            self.damage_bounds([0.5f32, 0.0f32, 0.0f32, 1.0f32, 0.5f32, 1.0f32]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            var5 = true;
        } else if (var6 == 2) {
            self.damage_bounds([0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.5f32, 0.5f32]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            self.damage_bounds([0.0f32, 0.0f32, 0.5f32, 1.0f32, 1.0f32, 1.0f32]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            var5 = true;
        } else if (var6 == 3) {
            self.damage_bounds([0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 0.5f32]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            self.damage_bounds([0.0f32, 0.0f32, 0.5f32, 1.0f32, 0.5f32, 1.0f32]);
            self.render_standard_block(var1 as u8, var2, var3, var4);
            var5 = true;
        }
        self.damage_bounds([0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32]);
        return var5;
    }
    fn damage_bottom_face(
        &mut self,
        mut var1: i32,
        mut var2: f64,
        mut var4: f64,
        mut var6: f64,
        mut var8: i32,
    ) {
        if (self.override_texture.unwrap() >= 0) {
            var8 = self.override_texture.unwrap();
        }
        let mut var10: i32 = ((var8 & 15) << 4);
        let mut var11: i32 = (var8 & 240);
        let mut var12: f64 = (((var10 as f64) + (self.b.min_x * 16.0f64)) / 256.0f64);
        let mut var14: f64 = ((((var10 as f64) + (self.b.max_x * 16.0f64)) - 0.01f64) / 256.0f64);
        let mut var16: f64 = (((var11 as f64) + (self.b.min_z * 16.0f64)) / 256.0f64);
        let mut var18: f64 = ((((var11 as f64) + (self.b.max_z * 16.0f64)) - 0.01f64) / 256.0f64);
        if ((self.b.min_x < 0.0f64) || (self.b.max_x > 1.0f64)) {
            var12 = ((((var10 as f32) + 0.0f32) / 256.0f32) as f64);
            var14 = ((((var10 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        if ((self.b.min_z < 0.0f64) || (self.b.max_z > 1.0f64)) {
            var16 = ((((var11 as f32) + 0.0f32) / 256.0f32) as f64);
            var18 = ((((var11 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        let mut var20: f64 = var14;
        let mut var22: f64 = var12;
        let mut var24: f64 = var16;
        let mut var26: f64 = var18;
        if (self.damage_uv[5] == 2) {
            var12 = (((var10 as f64) + (self.b.min_z * 16.0f64)) / 256.0f64);
            var16 = ((((var11 + 16) as f64) - (self.b.max_x * 16.0f64)) / 256.0f64);
            var14 = (((var10 as f64) + (self.b.max_z * 16.0f64)) / 256.0f64);
            var18 = ((((var11 + 16) as f64) - (self.b.min_x * 16.0f64)) / 256.0f64);
            var24 = var16;
            var26 = var18;
            var20 = var12;
            var22 = var14;
            var16 = var18;
            var18 = var24;
        } else if (self.damage_uv[5] == 1) {
            var12 = ((((var10 + 16) as f64) - (self.b.max_z * 16.0f64)) / 256.0f64);
            var16 = (((var11 as f64) + (self.b.min_x * 16.0f64)) / 256.0f64);
            var14 = ((((var10 + 16) as f64) - (self.b.min_z * 16.0f64)) / 256.0f64);
            var18 = (((var11 as f64) + (self.b.max_x * 16.0f64)) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var12 = var14;
            var14 = var22;
            var24 = var18;
            var26 = var16;
        } else if (self.damage_uv[5] == 3) {
            var12 = ((((var10 + 16) as f64) - (self.b.min_x * 16.0f64)) / 256.0f64);
            var14 = (((((var10 + 16) as f64) - (self.b.max_x * 16.0f64)) - 0.01f64) / 256.0f64);
            var16 = ((((var11 + 16) as f64) - (self.b.min_z * 16.0f64)) / 256.0f64);
            var18 = (((((var11 + 16) as f64) - (self.b.max_z * 16.0f64)) - 0.01f64) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var24 = var16;
            var26 = var18;
        }
        let mut var28: f64 = (var2 + self.b.min_x);
        let mut var30: f64 = (var2 + self.b.max_x);
        let mut var32: f64 = (var4 + self.b.min_y);
        let mut var34: f64 = (var6 + self.b.min_z);
        let mut var36: f64 = (var6 + self.b.max_z);
        if false {
            ();
            self.t.vertex_uv(var28, var32, var36, var22, var26);
            ();
            self.t.vertex_uv(var28, var32, var34, var12, var16);
            ();
            self.t.vertex_uv(var30, var32, var34, var20, var24);
            ();
            self.t.vertex_uv(var30, var32, var36, var14, var18);
        } else {
            self.t.vertex_uv(var28, var32, var36, var22, var26);
            self.t.vertex_uv(var28, var32, var34, var12, var16);
            self.t.vertex_uv(var30, var32, var34, var20, var24);
            self.t.vertex_uv(var30, var32, var36, var14, var18);
        }
    }
    fn damage_top_face(
        &mut self,
        mut var1: i32,
        mut var2: f64,
        mut var4: f64,
        mut var6: f64,
        mut var8: i32,
    ) {
        if (self.override_texture.unwrap() >= 0) {
            var8 = self.override_texture.unwrap();
        }
        let mut var10: i32 = ((var8 & 15) << 4);
        let mut var11: i32 = (var8 & 240);
        let mut var12: f64 = (((var10 as f64) + (self.b.min_x * 16.0f64)) / 256.0f64);
        let mut var14: f64 = ((((var10 as f64) + (self.b.max_x * 16.0f64)) - 0.01f64) / 256.0f64);
        let mut var16: f64 = (((var11 as f64) + (self.b.min_z * 16.0f64)) / 256.0f64);
        let mut var18: f64 = ((((var11 as f64) + (self.b.max_z * 16.0f64)) - 0.01f64) / 256.0f64);
        if ((self.b.min_x < 0.0f64) || (self.b.max_x > 1.0f64)) {
            var12 = ((((var10 as f32) + 0.0f32) / 256.0f32) as f64);
            var14 = ((((var10 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        if ((self.b.min_z < 0.0f64) || (self.b.max_z > 1.0f64)) {
            var16 = ((((var11 as f32) + 0.0f32) / 256.0f32) as f64);
            var18 = ((((var11 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        let mut var20: f64 = var14;
        let mut var22: f64 = var12;
        let mut var24: f64 = var16;
        let mut var26: f64 = var18;
        if (self.damage_uv[4] == 1) {
            var12 = (((var10 as f64) + (self.b.min_z * 16.0f64)) / 256.0f64);
            var16 = ((((var11 + 16) as f64) - (self.b.max_x * 16.0f64)) / 256.0f64);
            var14 = (((var10 as f64) + (self.b.max_z * 16.0f64)) / 256.0f64);
            var18 = ((((var11 + 16) as f64) - (self.b.min_x * 16.0f64)) / 256.0f64);
            var24 = var16;
            var26 = var18;
            var20 = var12;
            var22 = var14;
            var16 = var18;
            var18 = var24;
        } else if (self.damage_uv[4] == 2) {
            var12 = ((((var10 + 16) as f64) - (self.b.max_z * 16.0f64)) / 256.0f64);
            var16 = (((var11 as f64) + (self.b.min_x * 16.0f64)) / 256.0f64);
            var14 = ((((var10 + 16) as f64) - (self.b.min_z * 16.0f64)) / 256.0f64);
            var18 = (((var11 as f64) + (self.b.max_x * 16.0f64)) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var12 = var14;
            var14 = var22;
            var24 = var18;
            var26 = var16;
        } else if (self.damage_uv[4] == 3) {
            var12 = ((((var10 + 16) as f64) - (self.b.min_x * 16.0f64)) / 256.0f64);
            var14 = (((((var10 + 16) as f64) - (self.b.max_x * 16.0f64)) - 0.01f64) / 256.0f64);
            var16 = ((((var11 + 16) as f64) - (self.b.min_z * 16.0f64)) / 256.0f64);
            var18 = (((((var11 + 16) as f64) - (self.b.max_z * 16.0f64)) - 0.01f64) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var24 = var16;
            var26 = var18;
        }
        let mut var28: f64 = (var2 + self.b.min_x);
        let mut var30: f64 = (var2 + self.b.max_x);
        let mut var32: f64 = (var4 + self.b.max_y);
        let mut var34: f64 = (var6 + self.b.min_z);
        let mut var36: f64 = (var6 + self.b.max_z);
        if false {
            ();
            self.t.vertex_uv(var30, var32, var36, var14, var18);
            ();
            self.t.vertex_uv(var30, var32, var34, var20, var24);
            ();
            self.t.vertex_uv(var28, var32, var34, var12, var16);
            ();
            self.t.vertex_uv(var28, var32, var36, var22, var26);
        } else {
            self.t.vertex_uv(var30, var32, var36, var14, var18);
            self.t.vertex_uv(var30, var32, var34, var20, var24);
            self.t.vertex_uv(var28, var32, var34, var12, var16);
            self.t.vertex_uv(var28, var32, var36, var22, var26);
        }
    }
    fn damage_east_face(
        &mut self,
        mut var1: i32,
        mut var2: f64,
        mut var4: f64,
        mut var6: f64,
        mut var8: i32,
    ) {
        if (self.override_texture.unwrap() >= 0) {
            var8 = self.override_texture.unwrap();
        }
        let mut var10: i32 = ((var8 & 15) << 4);
        let mut var11: i32 = (var8 & 240);
        let mut var12: f64 = (((var10 as f64) + (self.b.min_x * 16.0f64)) / 256.0f64);
        let mut var14: f64 = ((((var10 as f64) + (self.b.max_x * 16.0f64)) - 0.01f64) / 256.0f64);
        let mut var16: f64 = ((((var11 + 16) as f64) - (self.b.max_y * 16.0f64)) / 256.0f64);
        let mut var18: f64 =
            (((((var11 + 16) as f64) - (self.b.min_y * 16.0f64)) - 0.01f64) / 256.0f64);
        let mut var20: f64 = 0.0;
        if self.damage_flip {
            var20 = var12;
            var12 = var14;
            var14 = var20;
        }
        if ((self.b.min_x < 0.0f64) || (self.b.max_x > 1.0f64)) {
            var12 = ((((var10 as f32) + 0.0f32) / 256.0f32) as f64);
            var14 = ((((var10 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        if ((self.b.min_y < 0.0f64) || (self.b.max_y > 1.0f64)) {
            var16 = ((((var11 as f32) + 0.0f32) / 256.0f32) as f64);
            var18 = ((((var11 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        var20 = var14;
        let mut var22: f64 = var12;
        let mut var24: f64 = var16;
        let mut var26: f64 = var18;
        if (self.damage_uv[0] == 2) {
            var12 = (((var10 as f64) + (self.b.min_y * 16.0f64)) / 256.0f64);
            var16 = ((((var11 + 16) as f64) - (self.b.min_x * 16.0f64)) / 256.0f64);
            var14 = (((var10 as f64) + (self.b.max_y * 16.0f64)) / 256.0f64);
            var18 = ((((var11 + 16) as f64) - (self.b.max_x * 16.0f64)) / 256.0f64);
            var24 = var16;
            var26 = var18;
            var20 = var12;
            var22 = var14;
            var16 = var18;
            var18 = var24;
        } else if (self.damage_uv[0] == 1) {
            var12 = ((((var10 + 16) as f64) - (self.b.max_y * 16.0f64)) / 256.0f64);
            var16 = (((var11 as f64) + (self.b.max_x * 16.0f64)) / 256.0f64);
            var14 = ((((var10 + 16) as f64) - (self.b.min_y * 16.0f64)) / 256.0f64);
            var18 = (((var11 as f64) + (self.b.min_x * 16.0f64)) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var12 = var14;
            var14 = var22;
            var24 = var18;
            var26 = var16;
        } else if (self.damage_uv[0] == 3) {
            var12 = ((((var10 + 16) as f64) - (self.b.min_x * 16.0f64)) / 256.0f64);
            var14 = (((((var10 + 16) as f64) - (self.b.max_x * 16.0f64)) - 0.01f64) / 256.0f64);
            var16 = (((var11 as f64) + (self.b.max_y * 16.0f64)) / 256.0f64);
            var18 = ((((var11 as f64) + (self.b.min_y * 16.0f64)) - 0.01f64) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var24 = var16;
            var26 = var18;
        }
        let mut var28: f64 = (var2 + self.b.min_x);
        let mut var30: f64 = (var2 + self.b.max_x);
        let mut var32: f64 = (var4 + self.b.min_y);
        let mut var34: f64 = (var4 + self.b.max_y);
        let mut var36: f64 = (var6 + self.b.min_z);
        if false {
            ();
            self.t.vertex_uv(var28, var34, var36, var20, var24);
            ();
            self.t.vertex_uv(var30, var34, var36, var12, var16);
            ();
            self.t.vertex_uv(var30, var32, var36, var22, var26);
            ();
            self.t.vertex_uv(var28, var32, var36, var14, var18);
        } else {
            self.t.vertex_uv(var28, var34, var36, var20, var24);
            self.t.vertex_uv(var30, var34, var36, var12, var16);
            self.t.vertex_uv(var30, var32, var36, var22, var26);
            self.t.vertex_uv(var28, var32, var36, var14, var18);
        }
    }
    fn damage_west_face(
        &mut self,
        mut var1: i32,
        mut var2: f64,
        mut var4: f64,
        mut var6: f64,
        mut var8: i32,
    ) {
        if (self.override_texture.unwrap() >= 0) {
            var8 = self.override_texture.unwrap();
        }
        let mut var10: i32 = ((var8 & 15) << 4);
        let mut var11: i32 = (var8 & 240);
        let mut var12: f64 = (((var10 as f64) + (self.b.min_x * 16.0f64)) / 256.0f64);
        let mut var14: f64 = ((((var10 as f64) + (self.b.max_x * 16.0f64)) - 0.01f64) / 256.0f64);
        let mut var16: f64 = ((((var11 + 16) as f64) - (self.b.max_y * 16.0f64)) / 256.0f64);
        let mut var18: f64 =
            (((((var11 + 16) as f64) - (self.b.min_y * 16.0f64)) - 0.01f64) / 256.0f64);
        let mut var20: f64 = 0.0;
        if self.damage_flip {
            var20 = var12;
            var12 = var14;
            var14 = var20;
        }
        if ((self.b.min_x < 0.0f64) || (self.b.max_x > 1.0f64)) {
            var12 = ((((var10 as f32) + 0.0f32) / 256.0f32) as f64);
            var14 = ((((var10 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        if ((self.b.min_y < 0.0f64) || (self.b.max_y > 1.0f64)) {
            var16 = ((((var11 as f32) + 0.0f32) / 256.0f32) as f64);
            var18 = ((((var11 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        var20 = var14;
        let mut var22: f64 = var12;
        let mut var24: f64 = var16;
        let mut var26: f64 = var18;
        if (self.damage_uv[1] == 1) {
            var12 = (((var10 as f64) + (self.b.min_y * 16.0f64)) / 256.0f64);
            var18 = ((((var11 + 16) as f64) - (self.b.min_x * 16.0f64)) / 256.0f64);
            var14 = (((var10 as f64) + (self.b.max_y * 16.0f64)) / 256.0f64);
            var16 = ((((var11 + 16) as f64) - (self.b.max_x * 16.0f64)) / 256.0f64);
            var24 = var16;
            var26 = var18;
            var20 = var12;
            var22 = var14;
            var16 = var18;
            var18 = var24;
        } else if (self.damage_uv[1] == 2) {
            var12 = ((((var10 + 16) as f64) - (self.b.max_y * 16.0f64)) / 256.0f64);
            var16 = (((var11 as f64) + (self.b.min_x * 16.0f64)) / 256.0f64);
            var14 = ((((var10 + 16) as f64) - (self.b.min_y * 16.0f64)) / 256.0f64);
            var18 = (((var11 as f64) + (self.b.max_x * 16.0f64)) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var12 = var14;
            var14 = var22;
            var24 = var18;
            var26 = var16;
        } else if (self.damage_uv[1] == 3) {
            var12 = ((((var10 + 16) as f64) - (self.b.min_x * 16.0f64)) / 256.0f64);
            var14 = (((((var10 + 16) as f64) - (self.b.max_x * 16.0f64)) - 0.01f64) / 256.0f64);
            var16 = (((var11 as f64) + (self.b.max_y * 16.0f64)) / 256.0f64);
            var18 = ((((var11 as f64) + (self.b.min_y * 16.0f64)) - 0.01f64) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var24 = var16;
            var26 = var18;
        }
        let mut var28: f64 = (var2 + self.b.min_x);
        let mut var30: f64 = (var2 + self.b.max_x);
        let mut var32: f64 = (var4 + self.b.min_y);
        let mut var34: f64 = (var4 + self.b.max_y);
        let mut var36: f64 = (var6 + self.b.max_z);
        if false {
            ();
            self.t.vertex_uv(var28, var34, var36, var12, var16);
            ();
            self.t.vertex_uv(var28, var32, var36, var22, var26);
            ();
            self.t.vertex_uv(var30, var32, var36, var14, var18);
            ();
            self.t.vertex_uv(var30, var34, var36, var20, var24);
        } else {
            self.t.vertex_uv(var28, var34, var36, var12, var16);
            self.t.vertex_uv(var28, var32, var36, var22, var26);
            self.t.vertex_uv(var30, var32, var36, var14, var18);
            self.t.vertex_uv(var30, var34, var36, var20, var24);
        }
    }
    fn damage_north_face(
        &mut self,
        mut var1: i32,
        mut var2: f64,
        mut var4: f64,
        mut var6: f64,
        mut var8: i32,
    ) {
        if (self.override_texture.unwrap() >= 0) {
            var8 = self.override_texture.unwrap();
        }
        let mut var10: i32 = ((var8 & 15) << 4);
        let mut var11: i32 = (var8 & 240);
        let mut var12: f64 = (((var10 as f64) + (self.b.min_z * 16.0f64)) / 256.0f64);
        let mut var14: f64 = ((((var10 as f64) + (self.b.max_z * 16.0f64)) - 0.01f64) / 256.0f64);
        let mut var16: f64 = ((((var11 + 16) as f64) - (self.b.max_y * 16.0f64)) / 256.0f64);
        let mut var18: f64 =
            (((((var11 + 16) as f64) - (self.b.min_y * 16.0f64)) - 0.01f64) / 256.0f64);
        let mut var20: f64 = 0.0;
        if self.damage_flip {
            var20 = var12;
            var12 = var14;
            var14 = var20;
        }
        if ((self.b.min_z < 0.0f64) || (self.b.max_z > 1.0f64)) {
            var12 = ((((var10 as f32) + 0.0f32) / 256.0f32) as f64);
            var14 = ((((var10 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        if ((self.b.min_y < 0.0f64) || (self.b.max_y > 1.0f64)) {
            var16 = ((((var11 as f32) + 0.0f32) / 256.0f32) as f64);
            var18 = ((((var11 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        var20 = var14;
        let mut var22: f64 = var12;
        let mut var24: f64 = var16;
        let mut var26: f64 = var18;
        if (self.damage_uv[3] == 1) {
            var12 = (((var10 as f64) + (self.b.min_y * 16.0f64)) / 256.0f64);
            var16 = ((((var11 + 16) as f64) - (self.b.max_z * 16.0f64)) / 256.0f64);
            var14 = (((var10 as f64) + (self.b.max_y * 16.0f64)) / 256.0f64);
            var18 = ((((var11 + 16) as f64) - (self.b.min_z * 16.0f64)) / 256.0f64);
            var24 = var16;
            var26 = var18;
            var20 = var12;
            var22 = var14;
            var16 = var18;
            var18 = var24;
        } else if (self.damage_uv[3] == 2) {
            var12 = ((((var10 + 16) as f64) - (self.b.max_y * 16.0f64)) / 256.0f64);
            var16 = (((var11 as f64) + (self.b.min_z * 16.0f64)) / 256.0f64);
            var14 = ((((var10 + 16) as f64) - (self.b.min_y * 16.0f64)) / 256.0f64);
            var18 = (((var11 as f64) + (self.b.max_z * 16.0f64)) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var12 = var14;
            var14 = var22;
            var24 = var18;
            var26 = var16;
        } else if (self.damage_uv[3] == 3) {
            var12 = ((((var10 + 16) as f64) - (self.b.min_z * 16.0f64)) / 256.0f64);
            var14 = (((((var10 + 16) as f64) - (self.b.max_z * 16.0f64)) - 0.01f64) / 256.0f64);
            var16 = (((var11 as f64) + (self.b.max_y * 16.0f64)) / 256.0f64);
            var18 = ((((var11 as f64) + (self.b.min_y * 16.0f64)) - 0.01f64) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var24 = var16;
            var26 = var18;
        }
        let mut var28: f64 = (var2 + self.b.min_x);
        let mut var30: f64 = (var4 + self.b.min_y);
        let mut var32: f64 = (var4 + self.b.max_y);
        let mut var34: f64 = (var6 + self.b.min_z);
        let mut var36: f64 = (var6 + self.b.max_z);
        if false {
            ();
            self.t.vertex_uv(var28, var32, var36, var20, var24);
            ();
            self.t.vertex_uv(var28, var32, var34, var12, var16);
            ();
            self.t.vertex_uv(var28, var30, var34, var22, var26);
            ();
            self.t.vertex_uv(var28, var30, var36, var14, var18);
        } else {
            self.t.vertex_uv(var28, var32, var36, var20, var24);
            self.t.vertex_uv(var28, var32, var34, var12, var16);
            self.t.vertex_uv(var28, var30, var34, var22, var26);
            self.t.vertex_uv(var28, var30, var36, var14, var18);
        }
    }
    fn damage_south_face(
        &mut self,
        mut var1: i32,
        mut var2: f64,
        mut var4: f64,
        mut var6: f64,
        mut var8: i32,
    ) {
        if (self.override_texture.unwrap() >= 0) {
            var8 = self.override_texture.unwrap();
        }
        let mut var10: i32 = ((var8 & 15) << 4);
        let mut var11: i32 = (var8 & 240);
        let mut var12: f64 = (((var10 as f64) + (self.b.min_z * 16.0f64)) / 256.0f64);
        let mut var14: f64 = ((((var10 as f64) + (self.b.max_z * 16.0f64)) - 0.01f64) / 256.0f64);
        let mut var16: f64 = ((((var11 + 16) as f64) - (self.b.max_y * 16.0f64)) / 256.0f64);
        let mut var18: f64 =
            (((((var11 + 16) as f64) - (self.b.min_y * 16.0f64)) - 0.01f64) / 256.0f64);
        let mut var20: f64 = 0.0;
        if self.damage_flip {
            var20 = var12;
            var12 = var14;
            var14 = var20;
        }
        if ((self.b.min_z < 0.0f64) || (self.b.max_z > 1.0f64)) {
            var12 = ((((var10 as f32) + 0.0f32) / 256.0f32) as f64);
            var14 = ((((var10 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        if ((self.b.min_y < 0.0f64) || (self.b.max_y > 1.0f64)) {
            var16 = ((((var11 as f32) + 0.0f32) / 256.0f32) as f64);
            var18 = ((((var11 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        var20 = var14;
        let mut var22: f64 = var12;
        let mut var24: f64 = var16;
        let mut var26: f64 = var18;
        if (self.damage_uv[2] == 2) {
            var12 = (((var10 as f64) + (self.b.min_y * 16.0f64)) / 256.0f64);
            var16 = ((((var11 + 16) as f64) - (self.b.min_z * 16.0f64)) / 256.0f64);
            var14 = (((var10 as f64) + (self.b.max_y * 16.0f64)) / 256.0f64);
            var18 = ((((var11 + 16) as f64) - (self.b.max_z * 16.0f64)) / 256.0f64);
            var24 = var16;
            var26 = var18;
            var20 = var12;
            var22 = var14;
            var16 = var18;
            var18 = var24;
        } else if (self.damage_uv[2] == 1) {
            var12 = ((((var10 + 16) as f64) - (self.b.max_y * 16.0f64)) / 256.0f64);
            var16 = (((var11 as f64) + (self.b.max_z * 16.0f64)) / 256.0f64);
            var14 = ((((var10 + 16) as f64) - (self.b.min_y * 16.0f64)) / 256.0f64);
            var18 = (((var11 as f64) + (self.b.min_z * 16.0f64)) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var12 = var14;
            var14 = var22;
            var24 = var18;
            var26 = var16;
        } else if (self.damage_uv[2] == 3) {
            var12 = ((((var10 + 16) as f64) - (self.b.min_z * 16.0f64)) / 256.0f64);
            var14 = (((((var10 + 16) as f64) - (self.b.max_z * 16.0f64)) - 0.01f64) / 256.0f64);
            var16 = (((var11 as f64) + (self.b.max_y * 16.0f64)) / 256.0f64);
            var18 = ((((var11 as f64) + (self.b.min_y * 16.0f64)) - 0.01f64) / 256.0f64);
            var20 = var14;
            var22 = var12;
            var24 = var16;
            var26 = var18;
        }
        let mut var28: f64 = (var2 + self.b.max_x);
        let mut var30: f64 = (var4 + self.b.min_y);
        let mut var32: f64 = (var4 + self.b.max_y);
        let mut var34: f64 = (var6 + self.b.min_z);
        let mut var36: f64 = (var6 + self.b.max_z);
        if false {
            ();
            self.t.vertex_uv(var28, var30, var36, var22, var26);
            ();
            self.t.vertex_uv(var28, var30, var34, var14, var18);
            ();
            self.t.vertex_uv(var28, var32, var34, var20, var24);
            ();
            self.t.vertex_uv(var28, var32, var36, var12, var16);
        } else {
            self.t.vertex_uv(var28, var30, var36, var22, var26);
            self.t.vertex_uv(var28, var30, var34, var14, var18);
            self.t.vertex_uv(var28, var32, var34, var20, var24);
            self.t.vertex_uv(var28, var32, var36, var12, var16);
        }
    }
    fn damage_piston(
        &mut self,
        mut var1: i32,
        mut var2: i32,
        mut var3: i32,
        mut var4: i32,
        mut var5: bool,
    ) -> bool {
        let mut var6: i32 = self.w.meta(var2, var3, var4);
        let mut var7: bool = (var5 || ((var6 & 8) != 0));
        let mut var8: i32 = (var6 & 7);
        if var7 {
            match var8 {
                0 => {
                    self.damage_uv[0] = 3;
                    self.damage_uv[1] = 3;
                    self.damage_uv[2] = 3;
                    self.damage_uv[3] = 3;
                    self.damage_bounds([0.0f32, 0.25f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32]);
                }
                1 => {
                    self.damage_bounds([
                        0.0f32,
                        0.0f32,
                        0.0f32,
                        1.0f32,
                        (12.0f32 / 16.0f32),
                        1.0f32,
                    ]);
                }
                2 => {
                    self.damage_uv[2] = 1;
                    self.damage_uv[3] = 2;
                    self.damage_bounds([0.0f32, 0.0f32, 0.25f32, 1.0f32, 1.0f32, 1.0f32]);
                }
                3 => {
                    self.damage_uv[2] = 2;
                    self.damage_uv[3] = 1;
                    self.damage_uv[4] = 3;
                    self.damage_uv[5] = 3;
                    self.damage_bounds([
                        0.0f32,
                        0.0f32,
                        0.0f32,
                        1.0f32,
                        1.0f32,
                        (12.0f32 / 16.0f32),
                    ]);
                }
                4 => {
                    self.damage_uv[0] = 1;
                    self.damage_uv[1] = 2;
                    self.damage_uv[4] = 2;
                    self.damage_uv[5] = 1;
                    self.damage_bounds([0.25f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32]);
                }
                5 => {
                    self.damage_uv[0] = 2;
                    self.damage_uv[1] = 1;
                    self.damage_uv[4] = 1;
                    self.damage_uv[5] = 2;
                    self.damage_bounds([
                        0.0f32,
                        0.0f32,
                        0.0f32,
                        (12.0f32 / 16.0f32),
                        1.0f32,
                        1.0f32,
                    ]);
                }
                _ => {}
            }
            self.render_standard_block(var1 as u8, var2, var3, var4);
            self.damage_uv[0] = 0;
            self.damage_uv[1] = 0;
            self.damage_uv[2] = 0;
            self.damage_uv[3] = 0;
            self.damage_uv[4] = 0;
            self.damage_uv[5] = 0;
            self.damage_bounds([0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32]);
        } else {
            match var8 {
                0 => {
                    self.damage_uv[0] = 3;
                    self.damage_uv[1] = 3;
                    self.damage_uv[2] = 3;
                    self.damage_uv[3] = 3;
                }
                1 => {}
                2 => {
                    self.damage_uv[2] = 1;
                    self.damage_uv[3] = 2;
                }
                3 => {
                    self.damage_uv[2] = 2;
                    self.damage_uv[3] = 1;
                    self.damage_uv[4] = 3;
                    self.damage_uv[5] = 3;
                }
                4 => {
                    self.damage_uv[0] = 1;
                    self.damage_uv[1] = 2;
                    self.damage_uv[4] = 2;
                    self.damage_uv[5] = 1;
                }
                5 => {
                    self.damage_uv[0] = 2;
                    self.damage_uv[1] = 1;
                    self.damage_uv[4] = 1;
                    self.damage_uv[5] = 2;
                }
                _ => {}
            }
            self.render_standard_block(var1 as u8, var2, var3, var4);
            self.damage_uv[0] = 0;
            self.damage_uv[1] = 0;
            self.damage_uv[2] = 0;
            self.damage_uv[3] = 0;
            self.damage_uv[4] = 0;
            self.damage_uv[5] = 0;
        }
        return true;
    }
    fn damage_extension(
        &mut self,
        mut var1: i32,
        mut var2: i32,
        mut var3: i32,
        mut var4: i32,
        mut var5: bool,
    ) -> bool {
        let mut var6: i32 = self.w.meta(var2, var3, var4);
        let mut var7: i32 = (var6 & 7);
        let mut var11: f32 = 1.0f32;
        let mut var12: f32 = (if var5 { 1.0f32 } else { 0.5f32 });
        let mut var13: f64 = (if var5 { 16.0f64 } else { 8.0f64 });
        match var7 {
            0 => {
                self.damage_uv[0] = 3;
                self.damage_uv[1] = 3;
                self.damage_uv[2] = 3;
                self.damage_uv[3] = 3;
                self.damage_bounds([0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.25f32, 1.0f32]);
                self.render_standard_block(var1 as u8, var2, var3, var4);
                self.damage_stem_a(
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + 0.25f32) as f64),
                    ((((var3 as f32) + 0.25f32) + var12) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (var11 * 0.8f32),
                    var13,
                );
                self.damage_stem_a(
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + 0.25f32) as f64),
                    ((((var3 as f32) + 0.25f32) + var12) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (var11 * 0.8f32),
                    var13,
                );
                self.damage_stem_a(
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + 0.25f32) as f64),
                    ((((var3 as f32) + 0.25f32) + var12) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
                self.damage_stem_a(
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + 0.25f32) as f64),
                    ((((var3 as f32) + 0.25f32) + var12) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
            }
            1 => {
                self.damage_bounds([0.0f32, (12.0f32 / 16.0f32), 0.0f32, 1.0f32, 1.0f32, 1.0f32]);
                self.render_standard_block(var1 as u8, var2, var3, var4);
                self.damage_stem_a(
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((((var3 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var3 as f32) - 0.25f32) + 1.0f32) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (var11 * 0.8f32),
                    var13,
                );
                self.damage_stem_a(
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((((var3 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var3 as f32) - 0.25f32) + 1.0f32) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (var11 * 0.8f32),
                    var13,
                );
                self.damage_stem_a(
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((((var3 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var3 as f32) - 0.25f32) + 1.0f32) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
                self.damage_stem_a(
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((((var3 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var3 as f32) - 0.25f32) + 1.0f32) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
            }
            2 => {
                self.damage_uv[2] = 1;
                self.damage_uv[3] = 2;
                self.damage_bounds([0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 0.25f32]);
                self.render_standard_block(var1 as u8, var2, var3, var4);
                self.damage_stem_b(
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + 0.25f32) as f64),
                    ((((var4 as f32) + 0.25f32) + var12) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
                self.damage_stem_b(
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + 0.25f32) as f64),
                    ((((var4 as f32) + 0.25f32) + var12) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
                self.damage_stem_b(
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + 0.25f32) as f64),
                    ((((var4 as f32) + 0.25f32) + var12) as f64),
                    (var11 * 0.5f32),
                    var13,
                );
                self.damage_stem_b(
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + 0.25f32) as f64),
                    ((((var4 as f32) + 0.25f32) + var12) as f64),
                    var11,
                    var13,
                );
            }
            3 => {
                self.damage_uv[2] = 2;
                self.damage_uv[3] = 1;
                self.damage_uv[4] = 3;
                self.damage_uv[5] = 3;
                self.damage_bounds([0.0f32, 0.0f32, (12.0f32 / 16.0f32), 1.0f32, 1.0f32, 1.0f32]);
                self.render_standard_block(var1 as u8, var2, var3, var4);
                self.damage_stem_b(
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((((var4 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var4 as f32) - 0.25f32) + 1.0f32) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
                self.damage_stem_b(
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((((var4 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var4 as f32) - 0.25f32) + 1.0f32) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
                self.damage_stem_b(
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((((var4 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var4 as f32) - 0.25f32) + 1.0f32) as f64),
                    (var11 * 0.5f32),
                    var13,
                );
                self.damage_stem_b(
                    (((var2 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var2 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((((var4 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var4 as f32) - 0.25f32) + 1.0f32) as f64),
                    var11,
                    var13,
                );
            }
            4 => {
                self.damage_uv[0] = 1;
                self.damage_uv[1] = 2;
                self.damage_uv[4] = 2;
                self.damage_uv[5] = 1;
                self.damage_bounds([0.0f32, 0.0f32, 0.0f32, 0.25f32, 1.0f32, 1.0f32]);
                self.render_standard_block(var1 as u8, var2, var3, var4);
                self.damage_stem_c(
                    (((var2 as f32) + 0.25f32) as f64),
                    ((((var2 as f32) + 0.25f32) + var12) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (var11 * 0.5f32),
                    var13,
                );
                self.damage_stem_c(
                    (((var2 as f32) + 0.25f32) as f64),
                    ((((var2 as f32) + 0.25f32) + var12) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    var11,
                    var13,
                );
                self.damage_stem_c(
                    (((var2 as f32) + 0.25f32) as f64),
                    ((((var2 as f32) + 0.25f32) + var12) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
                self.damage_stem_c(
                    (((var2 as f32) + 0.25f32) as f64),
                    ((((var2 as f32) + 0.25f32) + var12) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
            }
            5 => {
                self.damage_uv[0] = 2;
                self.damage_uv[1] = 1;
                self.damage_uv[4] = 1;
                self.damage_uv[5] = 2;
                self.damage_bounds([(12.0f32 / 16.0f32), 0.0f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32]);
                self.render_standard_block(var1 as u8, var2, var3, var4);
                self.damage_stem_c(
                    (((((var2 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var2 as f32) - 0.25f32) + 1.0f32) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (var11 * 0.5f32),
                    var13,
                );
                self.damage_stem_c(
                    (((((var2 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var2 as f32) - 0.25f32) + 1.0f32) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    var11,
                    var13,
                );
                self.damage_stem_c(
                    (((((var2 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var2 as f32) - 0.25f32) + 1.0f32) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
                self.damage_stem_c(
                    (((((var2 as f32) - 0.25f32) + 1.0f32) - var12) as f64),
                    ((((var2 as f32) - 0.25f32) + 1.0f32) as f64),
                    (((var3 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var3 as f32) + (6.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (((var4 as f32) + (10.0f32 / 16.0f32)) as f64),
                    (var11 * 0.6f32),
                    var13,
                );
            }
            _ => {}
        }
        self.damage_uv[0] = 0;
        self.damage_uv[1] = 0;
        self.damage_uv[2] = 0;
        self.damage_uv[3] = 0;
        self.damage_uv[4] = 0;
        self.damage_uv[5] = 0;
        self.damage_bounds([0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32]);
        return true;
    }
    fn damage_stem_a(
        &mut self,
        mut var1: f64,
        mut var3: f64,
        mut var5: f64,
        mut var7: f64,
        mut var9: f64,
        mut var11: f64,
        mut var13: f32,
        mut var14: f64,
    ) {
        let mut var16: i32 = 108;
        if (self.override_texture.unwrap() >= 0) {
            var16 = self.override_texture.unwrap();
        }
        let mut var17: i32 = ((var16 & 15) << 4);
        let mut var18: i32 = (var16 & 240);

        let mut var20: f64 = ((((var17 + 0) as f32) / 256.0f32) as f64);
        let mut var22: f64 = ((((var18 + 0) as f32) / 256.0f32) as f64);
        let mut var24: f64 = ((((var17 as f64) + var14) - 0.01f64) / 256.0f64);
        let mut var26: f64 = (((((var18 as f32) + 4.0f32) as f64) - 0.01f64) / 256.0f64);
        ();
        self.t.vertex_uv(var1, var7, var9, var24, var22);
        self.t.vertex_uv(var1, var5, var9, var20, var22);
        self.t.vertex_uv(var3, var5, var11, var20, var26);
        self.t.vertex_uv(var3, var7, var11, var24, var26);
    }
    fn damage_stem_b(
        &mut self,
        mut var1: f64,
        mut var3: f64,
        mut var5: f64,
        mut var7: f64,
        mut var9: f64,
        mut var11: f64,
        mut var13: f32,
        mut var14: f64,
    ) {
        let mut var16: i32 = 108;
        if (self.override_texture.unwrap() >= 0) {
            var16 = self.override_texture.unwrap();
        }
        let mut var17: i32 = ((var16 & 15) << 4);
        let mut var18: i32 = (var16 & 240);

        let mut var20: f64 = ((((var17 + 0) as f32) / 256.0f32) as f64);
        let mut var22: f64 = ((((var18 + 0) as f32) / 256.0f32) as f64);
        let mut var24: f64 = ((((var17 as f64) + var14) - 0.01f64) / 256.0f64);
        let mut var26: f64 = (((((var18 as f32) + 4.0f32) as f64) - 0.01f64) / 256.0f64);
        ();
        self.t.vertex_uv(var1, var5, var11, var24, var22);
        self.t.vertex_uv(var1, var5, var9, var20, var22);
        self.t.vertex_uv(var3, var7, var9, var20, var26);
        self.t.vertex_uv(var3, var7, var11, var24, var26);
    }
    fn damage_stem_c(
        &mut self,
        mut var1: f64,
        mut var3: f64,
        mut var5: f64,
        mut var7: f64,
        mut var9: f64,
        mut var11: f64,
        mut var13: f32,
        mut var14: f64,
    ) {
        let mut var16: i32 = 108;
        if (self.override_texture.unwrap() >= 0) {
            var16 = self.override_texture.unwrap();
        }
        let mut var17: i32 = ((var16 & 15) << 4);
        let mut var18: i32 = (var16 & 240);

        let mut var20: f64 = ((((var17 + 0) as f32) / 256.0f32) as f64);
        let mut var22: f64 = ((((var18 + 0) as f32) / 256.0f32) as f64);
        let mut var24: f64 = ((((var17 as f64) + var14) - 0.01f64) / 256.0f64);
        let mut var26: f64 = (((((var18 as f32) + 4.0f32) as f64) - 0.01f64) / 256.0f64);
        ();
        self.t.vertex_uv(var3, var5, var9, var24, var22);
        self.t.vertex_uv(var1, var5, var9, var20, var22);
        self.t.vertex_uv(var1, var7, var11, var20, var26);
        self.t.vertex_uv(var3, var7, var11, var24, var26);
    }
    fn damage_bed(&mut self, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) -> bool {
        let mut var6: i32 = self.w.meta(var2, var3, var4);
        let mut var7: i32 = (var6 & 3);
        let mut var8: bool = ((var6 & 8) != 0);
        let mut var9: f32 = 0.5f32;
        let mut var10: f32 = 1.0f32;
        let mut var11: f32 = 0.8f32;
        let mut var12: f32 = 0.6f32;
        let mut var25: f32 = 1.0f32;
        ();
        let mut var26: i32 = self.damage_native_texture(var1 as u8, var2, var3, var4, 0);
        let mut var27: i32 = ((var26 & 15) << 4);
        let mut var28: i32 = (var26 & 240);
        let mut var29: f64 = (((var27 as f32) / 256.0f32) as f64);
        let mut var31: f64 = ((((var27 + 16) as f64) - 0.01f64) / 256.0f64);
        let mut var33: f64 = (((var28 as f32) / 256.0f32) as f64);
        let mut var35: f64 = ((((var28 + 16) as f64) - 0.01f64) / 256.0f64);
        let mut var37: f64 = ((var2 as f64) + self.b.min_x);
        let mut var39: f64 = ((var2 as f64) + self.b.max_x);
        let mut var41: f64 = (((var3 as f64) + self.b.min_y) + 0.1875f64);
        let mut var43: f64 = ((var4 as f64) + self.b.min_z);
        let mut var45: f64 = ((var4 as f64) + self.b.max_z);
        self.t.vertex_uv(var37, var41, var45, var29, var35);
        self.t.vertex_uv(var37, var41, var43, var29, var33);
        self.t.vertex_uv(var39, var41, var43, var31, var33);
        self.t.vertex_uv(var39, var41, var45, var31, var35);
        let mut var64: f32 = 1.0f32;
        ();
        var27 = self.damage_native_texture(var1 as u8, var2, var3, var4, 1);
        var28 = ((var27 & 15) << 4);
        let mut var67: i32 = (var27 & 240);
        let mut var30: f64 = (((var28 as f32) / 256.0f32) as f64);
        let mut var32: f64 = ((((var28 + 16) as f64) - 0.01f64) / 256.0f64);
        let mut var34: f64 = (((var67 as f32) / 256.0f32) as f64);
        let mut var36: f64 = ((((var67 + 16) as f64) - 0.01f64) / 256.0f64);
        let mut var38: f64 = var30;
        let mut var40: f64 = var32;
        let mut var42: f64 = var34;
        let mut var44: f64 = var34;
        let mut var46: f64 = var30;
        let mut var48: f64 = var32;
        let mut var50: f64 = var36;
        let mut var52: f64 = var36;
        if (var7 == 0) {
            var40 = var30;
            var42 = var36;
            var46 = var32;
            var52 = var34;
        } else if (var7 == 2) {
            var38 = var32;
            var44 = var36;
            var48 = var30;
            var50 = var34;
        } else if (var7 == 3) {
            var38 = var32;
            var44 = var36;
            var48 = var30;
            var50 = var34;
            var40 = var30;
            var42 = var36;
            var46 = var32;
            var52 = var34;
        }
        let mut var54: f64 = ((var2 as f64) + self.b.min_x);
        let mut var56: f64 = ((var2 as f64) + self.b.max_x);
        let mut var58: f64 = ((var3 as f64) + self.b.max_y);
        let mut var60: f64 = ((var4 as f64) + self.b.min_z);
        let mut var62: f64 = ((var4 as f64) + self.b.max_z);
        self.t.vertex_uv(var56, var58, var62, var46, var50);
        self.t.vertex_uv(var56, var58, var60, var38, var42);
        self.t.vertex_uv(var54, var58, var60, var40, var44);
        self.t.vertex_uv(var54, var58, var62, var48, var52);
        var26 = [3i32, 4, 2, 5][var7 as usize];
        if var8 {
            var26 = [3i32, 4, 2, 5][[2i32, 3, 0, 1][var7 as usize] as usize];
        }
        let mut var65: i32 = 4;
        match var7 {
            0 => {
                var65 = 5;
            }
            1 => {
                var65 = 3;
            }
            2 => {}
            3 => {
                var65 = 2;
            }
            _ => {}
        }
        let mut var66: f32 = 0.0;
        if ((var26 != 2)
            && (false || self.should_side_be_rendered(var1 as u8, var2, var3, (var4 - 1), 2)))
        {
            var66 = 1.0f32;
            if (self.b.min_z > 0.0f64) {
                var66 = var25;
            }
            ();
            self.damage_flip = (var65 == 2);
            self.damage_east_face(
                var1,
                (var2 as f64),
                (var3 as f64),
                (var4 as f64),
                self.damage_native_texture(var1 as u8, var2, var3, var4, 2),
            );
        }
        if ((var26 != 3)
            && (false || self.should_side_be_rendered(var1 as u8, var2, var3, (var4 + 1), 3)))
        {
            var66 = 1.0f32;
            if (self.b.max_z < 1.0f64) {
                var66 = var25;
            }
            ();
            self.damage_flip = (var65 == 3);
            self.damage_west_face(
                var1,
                (var2 as f64),
                (var3 as f64),
                (var4 as f64),
                self.damage_native_texture(var1 as u8, var2, var3, var4, 3),
            );
        }
        if ((var26 != 4)
            && (false || self.should_side_be_rendered(var1 as u8, (var2 - 1), var3, var4, 4)))
        {
            var66 = 1.0f32;
            if (self.b.min_x > 0.0f64) {
                var66 = var25;
            }
            ();
            self.damage_flip = (var65 == 4);
            self.damage_north_face(
                var1,
                (var2 as f64),
                (var3 as f64),
                (var4 as f64),
                self.damage_native_texture(var1 as u8, var2, var3, var4, 4),
            );
        }
        if ((var26 != 5)
            && (false || self.should_side_be_rendered(var1 as u8, (var2 + 1), var3, var4, 5)))
        {
            var66 = 1.0f32;
            if (self.b.max_x < 1.0f64) {
                var66 = var25;
            }
            ();
            self.damage_flip = (var65 == 5);
            self.damage_south_face(
                var1,
                (var2 as f64),
                (var3 as f64),
                (var4 as f64),
                self.damage_native_texture(var1 as u8, var2, var3, var4, 5),
            );
        }
        self.damage_flip = false;
        return true;
    }
    fn damage_repeater(
        &mut self,
        mut var1: i32,
        mut var2: i32,
        mut var3: i32,
        mut var4: i32,
    ) -> bool {
        let mut var5: i32 = self.w.meta(var2, var3, var4);
        let mut var6: i32 = (var5 & 3);
        let mut var7: i32 = ((var5 & 12) >> 2);
        self.render_standard_block(var1 as u8, var2, var3, var4);

        let mut var9: f32 = 1.0f32;
        if ((worldgen::block::light_value(var1 as u8) as i32) > 0) {
            var9 = ((var9 + 1.0f32) * 0.5f32);
        }
        ();
        let mut var10: f64 = (-0.1875f64);
        let mut var12: f64 = 0.0f64;
        let mut var14: f64 = 0.0f64;
        let mut var16: f64 = 0.0f64;
        let mut var18: f64 = 0.0f64;
        match var6 {
            0 => {
                var18 = (-0.3125f64);
                var14 = [-0.0625f64, 0.0625, 0.1875, 0.3125][var7 as usize];
            }
            1 => {
                var16 = 0.3125f64;
                var12 = (-[-0.0625f64, 0.0625, 0.1875, 0.3125][var7 as usize]);
            }
            2 => {
                var18 = 0.3125f64;
                var14 = (-[-0.0625f64, 0.0625, 0.1875, 0.3125][var7 as usize]);
            }
            3 => {
                var16 = (-0.3125f64);
                var12 = [-0.0625f64, 0.0625, 0.1875, 0.3125][var7 as usize];
            }
            _ => {}
        }
        self.damage_torch_angle(
            var1,
            ((var2 as f64) + var12),
            ((var3 as f64) + var10),
            ((var4 as f64) + var14),
            0.0f64,
            0.0f64,
        );
        self.damage_torch_angle(
            var1,
            ((var2 as f64) + var16),
            ((var3 as f64) + var10),
            ((var4 as f64) + var18),
            0.0f64,
            0.0f64,
        );
        let mut var20: i32 = (rprops(var1 as u8).textures[0][1 as usize] as i32);
        let mut var21: i32 = ((var20 & 15) << 4);
        let mut var22: i32 = (var20 & 240);
        let mut var23: f64 = (((var21 as f32) / 256.0f32) as f64);
        let mut var25: f64 = ((((var21 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var27: f64 = (((var22 as f32) / 256.0f32) as f64);
        let mut var29: f64 = ((((var22 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var31: f32 = (2.0f32 / 16.0f32);
        let mut var32: f32 = ((var2 + 1) as f32);
        let mut var33: f32 = ((var2 + 1) as f32);
        let mut var34: f32 = ((var2 + 0) as f32);
        let mut var35: f32 = ((var2 + 0) as f32);
        let mut var36: f32 = ((var4 + 0) as f32);
        let mut var37: f32 = ((var4 + 1) as f32);
        let mut var38: f32 = ((var4 + 1) as f32);
        let mut var39: f32 = ((var4 + 0) as f32);
        let mut var40: f32 = ((var3 as f32) + var31);
        if (var6 == 2) {
            var33 = ((var2 + 0) as f32);
            var32 = var33;
            var35 = ((var2 + 1) as f32);
            var34 = var35;
            var39 = ((var4 + 1) as f32);
            var36 = var39;
            var38 = ((var4 + 0) as f32);
            var37 = var38;
        } else if (var6 == 3) {
            var35 = ((var2 + 0) as f32);
            var32 = var35;
            var34 = ((var2 + 1) as f32);
            var33 = var34;
            var37 = ((var4 + 0) as f32);
            var36 = var37;
            var39 = ((var4 + 1) as f32);
            var38 = var39;
        } else if (var6 == 1) {
            var35 = ((var2 + 1) as f32);
            var32 = var35;
            var34 = ((var2 + 0) as f32);
            var33 = var34;
            var37 = ((var4 + 1) as f32);
            var36 = var37;
            var39 = ((var4 + 0) as f32);
            var38 = var39;
        }
        self.t
            .vertex_uv((var35 as f64), (var40 as f64), (var39 as f64), var23, var27);
        self.t
            .vertex_uv((var34 as f64), (var40 as f64), (var38 as f64), var23, var29);
        self.t
            .vertex_uv((var33 as f64), (var40 as f64), (var37 as f64), var25, var29);
        self.t
            .vertex_uv((var32 as f64), (var40 as f64), (var36 as f64), var25, var27);
        return true;
    }
    fn damage_door(&mut self, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) -> bool {
        let mut var6: i32 = var1;
        let mut var7: bool = false;
        let mut var8: f32 = 0.5f32;
        let mut var9: f32 = 1.0f32;
        let mut var10: f32 = 0.8f32;
        let mut var11: f32 = 0.6f32;
        let mut var12: f32 = 1.0f32;
        let mut var13: f32 = 1.0f32;
        if (self.b.min_y > 0.0f64) {
            var13 = var12;
        }
        if ((worldgen::block::light_value(var1 as u8) as i32) > 0) {
            var13 = 1.0f32;
        }
        ();
        self.damage_bottom_face(
            var1,
            (var2 as f64),
            (var3 as f64),
            (var4 as f64),
            self.damage_native_texture(var1 as u8, var2, var3, var4, 0),
        );
        var7 = true;
        var13 = 1.0f32;
        if (self.b.max_y < 1.0f64) {
            var13 = var12;
        }
        if ((worldgen::block::light_value(var1 as u8) as i32) > 0) {
            var13 = 1.0f32;
        }
        ();
        self.damage_top_face(
            var1,
            (var2 as f64),
            (var3 as f64),
            (var4 as f64),
            self.damage_native_texture(var1 as u8, var2, var3, var4, 1),
        );
        var7 = true;
        var13 = 1.0f32;
        if (self.b.min_z > 0.0f64) {
            var13 = var12;
        }
        if ((worldgen::block::light_value(var1 as u8) as i32) > 0) {
            var13 = 1.0f32;
        }
        ();
        let mut var14: i32 = self.damage_native_texture(var1 as u8, var2, var3, var4, 2);
        if (var14 < 0) {
            self.damage_flip = true;
            var14 = (-var14);
        }
        self.damage_east_face(var1, (var2 as f64), (var3 as f64), (var4 as f64), var14);
        var7 = true;
        self.damage_flip = false;
        var13 = 1.0f32;
        if (self.b.max_z < 1.0f64) {
            var13 = var12;
        }
        if ((worldgen::block::light_value(var1 as u8) as i32) > 0) {
            var13 = 1.0f32;
        }
        ();
        var14 = self.damage_native_texture(var1 as u8, var2, var3, var4, 3);
        if (var14 < 0) {
            self.damage_flip = true;
            var14 = (-var14);
        }
        self.damage_west_face(var1, (var2 as f64), (var3 as f64), (var4 as f64), var14);
        var7 = true;
        self.damage_flip = false;
        var13 = 1.0f32;
        if (self.b.min_x > 0.0f64) {
            var13 = var12;
        }
        if ((worldgen::block::light_value(var1 as u8) as i32) > 0) {
            var13 = 1.0f32;
        }
        ();
        var14 = self.damage_native_texture(var1 as u8, var2, var3, var4, 4);
        if (var14 < 0) {
            self.damage_flip = true;
            var14 = (-var14);
        }
        self.damage_north_face(var1, (var2 as f64), (var3 as f64), (var4 as f64), var14);
        var7 = true;
        self.damage_flip = false;
        var13 = 1.0f32;
        if (self.b.max_x < 1.0f64) {
            var13 = var12;
        }
        if ((worldgen::block::light_value(var1 as u8) as i32) > 0) {
            var13 = 1.0f32;
        }
        ();
        var14 = self.damage_native_texture(var1 as u8, var2, var3, var4, 5);
        if (var14 < 0) {
            self.damage_flip = true;
            var14 = (-var14);
        }
        self.damage_south_face(var1, (var2 as f64), (var3 as f64), (var4 as f64), var14);
        var7 = true;
        self.damage_flip = false;
        return var7;
    }
    fn damage_wire(&mut self, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) -> bool {
        let mut var6: i32 = self.w.meta(var2, var3, var4);
        let mut var7: i32 = (rprops(var1 as u8).textures[var6 as usize & 15][1 as usize] as i32);
        if (self.override_texture.unwrap() >= 0) {
            var7 = self.override_texture.unwrap();
        }
        let mut var8: f32 = 1.0f32;
        let mut var9: f32 = ((var6 as f32) / 15.0f32);
        let mut var10: f32 = ((var9 * 0.6f32) + 0.4f32);
        if (var6 == 0) {
            var10 = 0.3f32;
        }
        let mut var11: f32 = (((var9 * var9) * 0.7f32) - 0.5f32);
        let mut var12: f32 = (((var9 * var9) * 0.6f32) - 0.7f32);
        if (var11 < 0.0f32) {
            var11 = 0.0f32;
        }
        if (var12 < 0.0f32) {
            var12 = 0.0f32;
        }
        ();
        let mut var13: i32 = ((var7 & 15) << 4);
        let mut var14: i32 = (var7 & 240);
        let mut var15: f64 = (((var13 as f32) / 256.0f32) as f64);
        let mut var17: f64 = ((((var13 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var19: f64 = (((var14 as f32) / 256.0f32) as f64);
        let mut var21: f64 = ((((var14 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var26: bool = (self.damage_wire_connect((var2 - 1), var3, var4, 1)
            || ((!self.damage_normal_cube((var2 - 1), var3, var4))
                && self.damage_wire_connect((var2 - 1), (var3 - 1), var4, (-1))));
        let mut var27: bool = (self.damage_wire_connect((var2 + 1), var3, var4, 3)
            || ((!self.damage_normal_cube((var2 + 1), var3, var4))
                && self.damage_wire_connect((var2 + 1), (var3 - 1), var4, (-1))));
        let mut var28: bool = (self.damage_wire_connect(var2, var3, (var4 - 1), 2)
            || ((!self.damage_normal_cube(var2, var3, (var4 - 1)))
                && self.damage_wire_connect(var2, (var3 - 1), (var4 - 1), (-1))));
        let mut var29: bool = (self.damage_wire_connect(var2, var3, (var4 + 1), 0)
            || ((!self.damage_normal_cube(var2, var3, (var4 + 1)))
                && self.damage_wire_connect(var2, (var3 - 1), (var4 + 1), (-1))));
        if (!self.damage_normal_cube(var2, (var3 + 1), var4)) {
            if (self.damage_normal_cube((var2 - 1), var3, var4)
                && self.damage_wire_connect((var2 - 1), (var3 + 1), var4, (-1)))
            {
                var26 = true;
            }
            if (self.damage_normal_cube((var2 + 1), var3, var4)
                && self.damage_wire_connect((var2 + 1), (var3 + 1), var4, (-1)))
            {
                var27 = true;
            }
            if (self.damage_normal_cube(var2, var3, (var4 - 1))
                && self.damage_wire_connect(var2, (var3 + 1), (var4 - 1), (-1)))
            {
                var28 = true;
            }
            if (self.damage_normal_cube(var2, var3, (var4 + 1))
                && self.damage_wire_connect(var2, (var3 + 1), (var4 + 1), (-1)))
            {
                var29 = true;
            }
        }
        let mut var31: f32 = ((var2 + 0) as f32);
        let mut var32: f32 = ((var2 + 1) as f32);
        let mut var33: f32 = ((var4 + 0) as f32);
        let mut var34: f32 = ((var4 + 1) as f32);
        let mut var35: i32 = 0;
        if (((var26 || var27) && (!var28)) && (!var29)) {
            var35 = 1;
        }
        if (((var28 || var29) && (!var27)) && (!var26)) {
            var35 = 2;
        }
        if (var35 != 0) {
            var15 = ((((var13 + 16) as f32) / 256.0f32) as f64);
            var17 = (((((var13 + 16) as f32) + 15.99f32) / 256.0f32) as f64);
            var19 = (((var14 as f32) / 256.0f32) as f64);
            var21 = ((((var14 as f32) + 15.99f32) / 256.0f32) as f64);
        }
        if (var35 == 0) {
            if (((var27 || var28) || var29) || var26) {
                if (!var26) {
                    var31 += (5.0f32 / 16.0f32);
                }
                if (!var26) {
                    var15 += (1.25f64 / 64.0f64);
                }
                if (!var27) {
                    var32 -= (5.0f32 / 16.0f32);
                }
                if (!var27) {
                    var17 -= (1.25f64 / 64.0f64);
                }
                if (!var28) {
                    var33 += (5.0f32 / 16.0f32);
                }
                if (!var28) {
                    var19 += (1.25f64 / 64.0f64);
                }
                if (!var29) {
                    var34 -= (5.0f32 / 16.0f32);
                }
                if (!var29) {
                    var21 -= (1.25f64 / 64.0f64);
                }
            }
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var17,
                var21,
            );
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var17,
                var19,
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var15,
                var19,
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var15,
                var21,
            );
            ();
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var17,
                (var21 + (1.0f64 / 16.0f64)),
            );
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var17,
                (var19 + (1.0f64 / 16.0f64)),
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var15,
                (var19 + (1.0f64 / 16.0f64)),
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var15,
                (var21 + (1.0f64 / 16.0f64)),
            );
        } else if (var35 == 1) {
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var17,
                var21,
            );
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var17,
                var19,
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var15,
                var19,
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var15,
                var21,
            );
            ();
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var17,
                (var21 + (1.0f64 / 16.0f64)),
            );
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var17,
                (var19 + (1.0f64 / 16.0f64)),
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var15,
                (var19 + (1.0f64 / 16.0f64)),
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var15,
                (var21 + (1.0f64 / 16.0f64)),
            );
        } else if (var35 == 2) {
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var17,
                var21,
            );
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var15,
                var21,
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var15,
                var19,
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var17,
                var19,
            );
            ();
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var17,
                (var21 + (1.0f64 / 16.0f64)),
            );
            self.t.vertex_uv(
                (var32 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var15,
                (var21 + (1.0f64 / 16.0f64)),
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var33 as f64),
                var15,
                (var19 + (1.0f64 / 16.0f64)),
            );
            self.t.vertex_uv(
                (var31 as f64),
                (((var3 as f32) + 0.015625f32) as f64),
                (var34 as f64),
                var17,
                (var19 + (1.0f64 / 16.0f64)),
            );
        }
        if (!self.damage_normal_cube(var2, (var3 + 1), var4)) {
            var15 = ((((var13 + 16) as f32) / 256.0f32) as f64);
            var17 = (((((var13 + 16) as f32) + 15.99f32) / 256.0f32) as f64);
            var19 = (((var14 as f32) / 256.0f32) as f64);
            var21 = ((((var14 as f32) + 15.99f32) / 256.0f32) as f64);
            if (self.damage_normal_cube((var2 - 1), var3, var4)
                && ((self.w.block((var2 - 1), (var3 + 1), var4) as i32) == 55))
            {
                ();
                self.t.vertex_uv(
                    (((var2 as f32) + 0.015625f32) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((var4 + 1) as f64),
                    var17,
                    var19,
                );
                self.t.vertex_uv(
                    (((var2 as f32) + 0.015625f32) as f64),
                    ((var3 + 0) as f64),
                    ((var4 + 1) as f64),
                    var15,
                    var19,
                );
                self.t.vertex_uv(
                    (((var2 as f32) + 0.015625f32) as f64),
                    ((var3 + 0) as f64),
                    ((var4 + 0) as f64),
                    var15,
                    var21,
                );
                self.t.vertex_uv(
                    (((var2 as f32) + 0.015625f32) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((var4 + 0) as f64),
                    var17,
                    var21,
                );
                ();
                self.t.vertex_uv(
                    (((var2 as f32) + 0.015625f32) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((var4 + 1) as f64),
                    var17,
                    (var19 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    (((var2 as f32) + 0.015625f32) as f64),
                    ((var3 + 0) as f64),
                    ((var4 + 1) as f64),
                    var15,
                    (var19 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    (((var2 as f32) + 0.015625f32) as f64),
                    ((var3 + 0) as f64),
                    ((var4 + 0) as f64),
                    var15,
                    (var21 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    (((var2 as f32) + 0.015625f32) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((var4 + 0) as f64),
                    var17,
                    (var21 + (1.0f64 / 16.0f64)),
                );
            }
            if (self.damage_normal_cube((var2 + 1), var3, var4)
                && ((self.w.block((var2 + 1), (var3 + 1), var4) as i32) == 55))
            {
                ();
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - 0.015625f32) as f64),
                    ((var3 + 0) as f64),
                    ((var4 + 1) as f64),
                    var15,
                    var21,
                );
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - 0.015625f32) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((var4 + 1) as f64),
                    var17,
                    var21,
                );
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - 0.015625f32) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((var4 + 0) as f64),
                    var17,
                    var19,
                );
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - 0.015625f32) as f64),
                    ((var3 + 0) as f64),
                    ((var4 + 0) as f64),
                    var15,
                    var19,
                );
                ();
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - 0.015625f32) as f64),
                    ((var3 + 0) as f64),
                    ((var4 + 1) as f64),
                    var15,
                    (var21 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - 0.015625f32) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((var4 + 1) as f64),
                    var17,
                    (var21 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - 0.015625f32) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((var4 + 0) as f64),
                    var17,
                    (var19 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - 0.015625f32) as f64),
                    ((var3 + 0) as f64),
                    ((var4 + 0) as f64),
                    var15,
                    (var19 + (1.0f64 / 16.0f64)),
                );
            }
            if (self.damage_normal_cube(var2, var3, (var4 - 1))
                && ((self.w.block(var2, (var3 + 1), (var4 - 1)) as i32) == 55))
            {
                ();
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((var3 + 0) as f64),
                    (((var4 as f32) + 0.015625f32) as f64),
                    var15,
                    var21,
                );
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    (((var4 as f32) + 0.015625f32) as f64),
                    var17,
                    var21,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    (((var4 as f32) + 0.015625f32) as f64),
                    var17,
                    var19,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((var3 + 0) as f64),
                    (((var4 as f32) + 0.015625f32) as f64),
                    var15,
                    var19,
                );
                ();
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((var3 + 0) as f64),
                    (((var4 as f32) + 0.015625f32) as f64),
                    var15,
                    (var21 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    (((var4 as f32) + 0.015625f32) as f64),
                    var17,
                    (var21 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    (((var4 as f32) + 0.015625f32) as f64),
                    var17,
                    (var19 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((var3 + 0) as f64),
                    (((var4 as f32) + 0.015625f32) as f64),
                    var15,
                    (var19 + (1.0f64 / 16.0f64)),
                );
            }
            if (self.damage_normal_cube(var2, var3, (var4 + 1))
                && ((self.w.block(var2, (var3 + 1), (var4 + 1)) as i32) == 55))
            {
                ();
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((((var4 + 1) as f32) - 0.015625f32) as f64),
                    var17,
                    var19,
                );
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((var3 + 0) as f64),
                    ((((var4 + 1) as f32) - 0.015625f32) as f64),
                    var15,
                    var19,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((var3 + 0) as f64),
                    ((((var4 + 1) as f32) - 0.015625f32) as f64),
                    var15,
                    var21,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((((var4 + 1) as f32) - 0.015625f32) as f64),
                    var17,
                    var21,
                );
                ();
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((((var4 + 1) as f32) - 0.015625f32) as f64),
                    var17,
                    (var19 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((var3 + 0) as f64),
                    ((((var4 + 1) as f32) - 0.015625f32) as f64),
                    var15,
                    (var19 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((var3 + 0) as f64),
                    ((((var4 + 1) as f32) - 0.015625f32) as f64),
                    var15,
                    (var21 + (1.0f64 / 16.0f64)),
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 1) as f32) + (7.0f32 / 320.0f32)) as f64),
                    ((((var4 + 1) as f32) - 0.015625f32) as f64),
                    var17,
                    (var21 + (1.0f64 / 16.0f64)),
                );
            }
        }
        return true;
    }
    fn damage_fire(&mut self, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) -> bool {
        let mut var6: i32 = (rprops(var1 as u8).textures[0][0 as usize] as i32);
        if (self.override_texture.unwrap() >= 0) {
            var6 = self.override_texture.unwrap();
        }
        let mut var7: f32 = 1.0f32;
        ();
        let mut var8: i32 = ((var6 & 15) << 4);
        let mut var9: i32 = (var6 & 240);
        let mut var10: f64 = (((var8 as f32) / 256.0f32) as f64);
        let mut var12: f64 = ((((var8 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var14: f64 = (((var9 as f32) / 256.0f32) as f64);
        let mut var16: f64 = ((((var9 as f32) + 15.99f32) / 256.0f32) as f64);
        let mut var18: f32 = 1.4f32;
        let mut var21: f64 = 0.0;
        let mut var23: f64 = 0.0;
        let mut var25: f64 = 0.0;
        let mut var27: f64 = 0.0;
        let mut var29: f64 = 0.0;
        let mut var31: f64 = 0.0;
        let mut var33: f64 = 0.0;
        if ((!self.damage_normal_cube(var2, (var3 - 1), var4))
            && (!matches!(
                self.w.block(var2, (var3 - 1), var4),
                5 | 85 | 53 | 17 | 18 | 47 | 46 | 31 | 35
            )))
        {
            let mut var37: f32 = 0.2f32;
            let mut var20: f32 = (1.0f32 / 16.0f32);
            if ((((var2 + var3) + var4) & 1) == 1) {
                var10 = (((var8 as f32) / 256.0f32) as f64);
                var12 = ((((var8 as f32) + 15.99f32) / 256.0f32) as f64);
                var14 = ((((var9 + 16) as f32) / 256.0f32) as f64);
                var16 = (((((var9 as f32) + 15.99f32) + 16.0f32) / 256.0f32) as f64);
            }
            if (((((var2 / 2) + (var3 / 2)) + (var4 / 2)) & 1) == 1) {
                var21 = var12;
                var12 = var10;
                var10 = var21;
            }
            if matches!(
                self.w.block((var2 - 1), var3, var4),
                5 | 85 | 53 | 17 | 18 | 47 | 46 | 31 | 35
            ) {
                self.t.vertex_uv(
                    (((var2 as f32) + var37) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((var4 + 1) as f64),
                    var12,
                    var14,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 1) as f64),
                    var12,
                    var16,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 0) as f64),
                    var10,
                    var16,
                );
                self.t.vertex_uv(
                    (((var2 as f32) + var37) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((var4 + 0) as f64),
                    var10,
                    var14,
                );
                self.t.vertex_uv(
                    (((var2 as f32) + var37) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((var4 + 0) as f64),
                    var10,
                    var14,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 0) as f64),
                    var10,
                    var16,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 1) as f64),
                    var12,
                    var16,
                );
                self.t.vertex_uv(
                    (((var2 as f32) + var37) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((var4 + 1) as f64),
                    var12,
                    var14,
                );
            }
            if matches!(
                self.w.block((var2 + 1), var3, var4),
                5 | 85 | 53 | 17 | 18 | 47 | 46 | 31 | 35
            ) {
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - var37) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((var4 + 0) as f64),
                    var10,
                    var14,
                );
                self.t.vertex_uv(
                    (((var2 + 1) - 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 0) as f64),
                    var10,
                    var16,
                );
                self.t.vertex_uv(
                    (((var2 + 1) - 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 1) as f64),
                    var12,
                    var16,
                );
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - var37) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((var4 + 1) as f64),
                    var12,
                    var14,
                );
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - var37) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((var4 + 1) as f64),
                    var12,
                    var14,
                );
                self.t.vertex_uv(
                    (((var2 + 1) - 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 1) as f64),
                    var12,
                    var16,
                );
                self.t.vertex_uv(
                    (((var2 + 1) - 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 0) as f64),
                    var10,
                    var16,
                );
                self.t.vertex_uv(
                    ((((var2 + 1) as f32) - var37) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((var4 + 0) as f64),
                    var10,
                    var14,
                );
            }
            if matches!(
                self.w.block(var2, var3, (var4 - 1)),
                5 | 85 | 53 | 17 | 18 | 47 | 46 | 31 | 35
            ) {
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    (((var4 as f32) + var37) as f64),
                    var12,
                    var14,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 0) as f64),
                    var12,
                    var16,
                );
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 0) as f64),
                    var10,
                    var16,
                );
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    (((var4 as f32) + var37) as f64),
                    var10,
                    var14,
                );
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    (((var4 as f32) + var37) as f64),
                    var10,
                    var14,
                );
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 0) as f64),
                    var10,
                    var16,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    ((var4 + 0) as f64),
                    var12,
                    var16,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    (((var4 as f32) + var37) as f64),
                    var12,
                    var14,
                );
            }
            if matches!(
                self.w.block(var2, var3, (var4 + 1)),
                5 | 85 | 53 | 17 | 18 | 47 | 46 | 31 | 35
            ) {
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((((var4 + 1) as f32) - var37) as f64),
                    var10,
                    var14,
                );
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    (((var4 + 1) - 0) as f64),
                    var10,
                    var16,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    (((var4 + 1) - 0) as f64),
                    var12,
                    var16,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((((var4 + 1) as f32) - var37) as f64),
                    var12,
                    var14,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((((var4 + 1) as f32) - var37) as f64),
                    var12,
                    var14,
                );
                self.t.vertex_uv(
                    ((var2 + 0) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    (((var4 + 1) - 0) as f64),
                    var12,
                    var16,
                );
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 + 0) as f32) + var20) as f64),
                    (((var4 + 1) - 0) as f64),
                    var10,
                    var16,
                );
                self.t.vertex_uv(
                    ((var2 + 1) as f64),
                    ((((var3 as f32) + var18) + var20) as f64),
                    ((((var4 + 1) as f32) - var37) as f64),
                    var10,
                    var14,
                );
            }
            if matches!(
                self.w.block(var2, (var3 + 1), var4),
                5 | 85 | 53 | 17 | 18 | 47 | 46 | 31 | 35
            ) {
                var21 = (((var2 as f64) + 0.5f64) + 0.5f64);
                var23 = (((var2 as f64) + 0.5f64) - 0.5f64);
                var25 = (((var4 as f64) + 0.5f64) + 0.5f64);
                var27 = (((var4 as f64) + 0.5f64) - 0.5f64);
                var29 = (((var2 as f64) + 0.5f64) - 0.5f64);
                var31 = (((var2 as f64) + 0.5f64) + 0.5f64);
                var33 = (((var4 as f64) + 0.5f64) - 0.5f64);
                let mut var35: f64 = (((var4 as f64) + 0.5f64) + 0.5f64);
                var10 = (((var8 as f32) / 256.0f32) as f64);
                var12 = ((((var8 as f32) + 15.99f32) / 256.0f32) as f64);
                var14 = (((var9 as f32) / 256.0f32) as f64);
                var16 = ((((var9 as f32) + 15.99f32) / 256.0f32) as f64);
                var3 += 1;
                var18 = (-0.2f32);
                if ((((var2 + var3) + var4) & 1) == 0) {
                    self.t.vertex_uv(
                        var29,
                        (((var3 as f32) + var18) as f64),
                        ((var4 + 0) as f64),
                        var12,
                        var14,
                    );
                    self.t.vertex_uv(
                        var21,
                        ((var3 + 0) as f64),
                        ((var4 + 0) as f64),
                        var12,
                        var16,
                    );
                    self.t.vertex_uv(
                        var21,
                        ((var3 + 0) as f64),
                        ((var4 + 1) as f64),
                        var10,
                        var16,
                    );
                    self.t.vertex_uv(
                        var29,
                        (((var3 as f32) + var18) as f64),
                        ((var4 + 1) as f64),
                        var10,
                        var14,
                    );
                    var10 = (((var8 as f32) / 256.0f32) as f64);
                    var12 = ((((var8 as f32) + 15.99f32) / 256.0f32) as f64);
                    var14 = ((((var9 + 16) as f32) / 256.0f32) as f64);
                    var16 = (((((var9 as f32) + 15.99f32) + 16.0f32) / 256.0f32) as f64);
                    self.t.vertex_uv(
                        var31,
                        (((var3 as f32) + var18) as f64),
                        ((var4 + 1) as f64),
                        var12,
                        var14,
                    );
                    self.t.vertex_uv(
                        var23,
                        ((var3 + 0) as f64),
                        ((var4 + 1) as f64),
                        var12,
                        var16,
                    );
                    self.t.vertex_uv(
                        var23,
                        ((var3 + 0) as f64),
                        ((var4 + 0) as f64),
                        var10,
                        var16,
                    );
                    self.t.vertex_uv(
                        var31,
                        (((var3 as f32) + var18) as f64),
                        ((var4 + 0) as f64),
                        var10,
                        var14,
                    );
                } else {
                    self.t.vertex_uv(
                        ((var2 + 0) as f64),
                        (((var3 as f32) + var18) as f64),
                        var35,
                        var12,
                        var14,
                    );
                    self.t.vertex_uv(
                        ((var2 + 0) as f64),
                        ((var3 + 0) as f64),
                        var27,
                        var12,
                        var16,
                    );
                    self.t.vertex_uv(
                        ((var2 + 1) as f64),
                        ((var3 + 0) as f64),
                        var27,
                        var10,
                        var16,
                    );
                    self.t.vertex_uv(
                        ((var2 + 1) as f64),
                        (((var3 as f32) + var18) as f64),
                        var35,
                        var10,
                        var14,
                    );
                    var10 = (((var8 as f32) / 256.0f32) as f64);
                    var12 = ((((var8 as f32) + 15.99f32) / 256.0f32) as f64);
                    var14 = ((((var9 + 16) as f32) / 256.0f32) as f64);
                    var16 = (((((var9 as f32) + 15.99f32) + 16.0f32) / 256.0f32) as f64);
                    self.t.vertex_uv(
                        ((var2 + 1) as f64),
                        (((var3 as f32) + var18) as f64),
                        var33,
                        var12,
                        var14,
                    );
                    self.t.vertex_uv(
                        ((var2 + 1) as f64),
                        ((var3 + 0) as f64),
                        var25,
                        var12,
                        var16,
                    );
                    self.t.vertex_uv(
                        ((var2 + 0) as f64),
                        ((var3 + 0) as f64),
                        var25,
                        var10,
                        var16,
                    );
                    self.t.vertex_uv(
                        ((var2 + 0) as f64),
                        (((var3 as f32) + var18) as f64),
                        var33,
                        var10,
                        var14,
                    );
                }
            }
        } else {
            let mut var19: f64 = (((var2 as f64) + 0.5f64) + 0.2f64);
            var21 = (((var2 as f64) + 0.5f64) - 0.2f64);
            var23 = (((var4 as f64) + 0.5f64) + 0.2f64);
            var25 = (((var4 as f64) + 0.5f64) - 0.2f64);
            var27 = (((var2 as f64) + 0.5f64) - 0.3f64);
            var29 = (((var2 as f64) + 0.5f64) + 0.3f64);
            var31 = (((var4 as f64) + 0.5f64) - 0.3f64);
            var33 = (((var4 as f64) + 0.5f64) + 0.3f64);
            self.t.vertex_uv(
                var27,
                (((var3 as f32) + var18) as f64),
                ((var4 + 1) as f64),
                var12,
                var14,
            );
            self.t.vertex_uv(
                var19,
                ((var3 + 0) as f64),
                ((var4 + 1) as f64),
                var12,
                var16,
            );
            self.t.vertex_uv(
                var19,
                ((var3 + 0) as f64),
                ((var4 + 0) as f64),
                var10,
                var16,
            );
            self.t.vertex_uv(
                var27,
                (((var3 as f32) + var18) as f64),
                ((var4 + 0) as f64),
                var10,
                var14,
            );
            self.t.vertex_uv(
                var29,
                (((var3 as f32) + var18) as f64),
                ((var4 + 0) as f64),
                var12,
                var14,
            );
            self.t.vertex_uv(
                var21,
                ((var3 + 0) as f64),
                ((var4 + 0) as f64),
                var12,
                var16,
            );
            self.t.vertex_uv(
                var21,
                ((var3 + 0) as f64),
                ((var4 + 1) as f64),
                var10,
                var16,
            );
            self.t.vertex_uv(
                var29,
                (((var3 as f32) + var18) as f64),
                ((var4 + 1) as f64),
                var10,
                var14,
            );
            var10 = (((var8 as f32) / 256.0f32) as f64);
            var12 = ((((var8 as f32) + 15.99f32) / 256.0f32) as f64);
            var14 = ((((var9 + 16) as f32) / 256.0f32) as f64);
            var16 = (((((var9 as f32) + 15.99f32) + 16.0f32) / 256.0f32) as f64);
            self.t.vertex_uv(
                ((var2 + 1) as f64),
                (((var3 as f32) + var18) as f64),
                var33,
                var12,
                var14,
            );
            self.t.vertex_uv(
                ((var2 + 1) as f64),
                ((var3 + 0) as f64),
                var25,
                var12,
                var16,
            );
            self.t.vertex_uv(
                ((var2 + 0) as f64),
                ((var3 + 0) as f64),
                var25,
                var10,
                var16,
            );
            self.t.vertex_uv(
                ((var2 + 0) as f64),
                (((var3 as f32) + var18) as f64),
                var33,
                var10,
                var14,
            );
            self.t.vertex_uv(
                ((var2 + 0) as f64),
                (((var3 as f32) + var18) as f64),
                var31,
                var12,
                var14,
            );
            self.t.vertex_uv(
                ((var2 + 0) as f64),
                ((var3 + 0) as f64),
                var23,
                var12,
                var16,
            );
            self.t.vertex_uv(
                ((var2 + 1) as f64),
                ((var3 + 0) as f64),
                var23,
                var10,
                var16,
            );
            self.t.vertex_uv(
                ((var2 + 1) as f64),
                (((var3 as f32) + var18) as f64),
                var31,
                var10,
                var14,
            );
            var19 = (((var2 as f64) + 0.5f64) - 0.5f64);
            var21 = (((var2 as f64) + 0.5f64) + 0.5f64);
            var23 = (((var4 as f64) + 0.5f64) - 0.5f64);
            var25 = (((var4 as f64) + 0.5f64) + 0.5f64);
            var27 = (((var2 as f64) + 0.5f64) - 0.4f64);
            var29 = (((var2 as f64) + 0.5f64) + 0.4f64);
            var31 = (((var4 as f64) + 0.5f64) - 0.4f64);
            var33 = (((var4 as f64) + 0.5f64) + 0.4f64);
            self.t.vertex_uv(
                var27,
                (((var3 as f32) + var18) as f64),
                ((var4 + 0) as f64),
                var10,
                var14,
            );
            self.t.vertex_uv(
                var19,
                ((var3 + 0) as f64),
                ((var4 + 0) as f64),
                var10,
                var16,
            );
            self.t.vertex_uv(
                var19,
                ((var3 + 0) as f64),
                ((var4 + 1) as f64),
                var12,
                var16,
            );
            self.t.vertex_uv(
                var27,
                (((var3 as f32) + var18) as f64),
                ((var4 + 1) as f64),
                var12,
                var14,
            );
            self.t.vertex_uv(
                var29,
                (((var3 as f32) + var18) as f64),
                ((var4 + 1) as f64),
                var10,
                var14,
            );
            self.t.vertex_uv(
                var21,
                ((var3 + 0) as f64),
                ((var4 + 1) as f64),
                var10,
                var16,
            );
            self.t.vertex_uv(
                var21,
                ((var3 + 0) as f64),
                ((var4 + 0) as f64),
                var12,
                var16,
            );
            self.t.vertex_uv(
                var29,
                (((var3 as f32) + var18) as f64),
                ((var4 + 0) as f64),
                var12,
                var14,
            );
            var10 = (((var8 as f32) / 256.0f32) as f64);
            var12 = ((((var8 as f32) + 15.99f32) / 256.0f32) as f64);
            var14 = (((var9 as f32) / 256.0f32) as f64);
            var16 = ((((var9 as f32) + 15.99f32) / 256.0f32) as f64);
            self.t.vertex_uv(
                ((var2 + 0) as f64),
                (((var3 as f32) + var18) as f64),
                var33,
                var10,
                var14,
            );
            self.t.vertex_uv(
                ((var2 + 0) as f64),
                ((var3 + 0) as f64),
                var25,
                var10,
                var16,
            );
            self.t.vertex_uv(
                ((var2 + 1) as f64),
                ((var3 + 0) as f64),
                var25,
                var12,
                var16,
            );
            self.t.vertex_uv(
                ((var2 + 1) as f64),
                (((var3 as f32) + var18) as f64),
                var33,
                var12,
                var14,
            );
            self.t.vertex_uv(
                ((var2 + 1) as f64),
                (((var3 as f32) + var18) as f64),
                var31,
                var10,
                var14,
            );
            self.t.vertex_uv(
                ((var2 + 1) as f64),
                ((var3 + 0) as f64),
                var23,
                var10,
                var16,
            );
            self.t.vertex_uv(
                ((var2 + 0) as f64),
                ((var3 + 0) as f64),
                var23,
                var12,
                var16,
            );
            self.t.vertex_uv(
                ((var2 + 0) as f64),
                (((var3 as f32) + var18) as f64),
                var31,
                var12,
                var14,
            );
        }
        return true;
    }
}
