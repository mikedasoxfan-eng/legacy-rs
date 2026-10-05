//! GuiMainMenu uses the installed OpenGL driver's fixed-function matrix
//! operations, whose last bits are not specified by OpenGL. The cube uses the original
//! single-sample GL rasterization; framebuffer copies, blur and GUI stay in wgpu.
#[cfg(target_os = "windows")]
mod native {
    use std::{cell::RefCell, ffi::c_void};
    type Handle = *mut c_void;
    #[repr(C)]
    struct PixelFormat {
        size: u16,
        version: u16,
        flags: u32,
        pixel_type: u8,
        color_bits: u8,
        red_bits: u8,
        red_shift: u8,
        green_bits: u8,
        green_shift: u8,
        blue_bits: u8,
        blue_shift: u8,
        alpha_bits: u8,
        alpha_shift: u8,
        accum_bits: u8,
        accum_red: u8,
        accum_green: u8,
        accum_blue: u8,
        accum_alpha: u8,
        depth_bits: u8,
        stencil_bits: u8,
        aux: u8,
        layer: u8,
        reserved: u8,
        layer_mask: u32,
        visible_mask: u32,
        damage_mask: u32,
    }
    #[link(name = "user32")]
    unsafe extern "system" {
        fn CreateWindowExW(
            ex: u32,
            class: *const u16,
            title: *const u16,
            style: u32,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            parent: Handle,
            menu: Handle,
            instance: Handle,
            param: Handle,
        ) -> Handle;
        fn DestroyWindow(window: Handle) -> i32;
        fn GetDC(window: Handle) -> Handle;
        fn ReleaseDC(window: Handle, dc: Handle) -> i32;
    }
    #[link(name = "gdi32")]
    unsafe extern "system" {
        fn ChoosePixelFormat(dc: Handle, format: *const PixelFormat) -> i32;
        fn SetPixelFormat(dc: Handle, index: i32, format: *const PixelFormat) -> i32;
    }
    #[link(name = "opengl32")]
    unsafe extern "system" {
        fn wglCreateContext(dc: Handle) -> Handle;
        fn wglDeleteContext(context: Handle) -> i32;
        fn wglMakeCurrent(dc: Handle, context: Handle) -> i32;
        fn wglGetCurrentContext() -> Handle;
        fn wglGetCurrentDC() -> Handle;
        fn glMatrixMode(mode: u32);
        fn glLoadIdentity();
        fn glLoadMatrixf(matrix: *const f32);
        fn glMultMatrixf(matrix: *const f32);
        fn glPushMatrix();
        fn glPopMatrix();
        fn glTranslatef(x: f32, y: f32, z: f32);
        fn glRotatef(angle: f32, x: f32, y: f32, z: f32);
        fn glGetFloatv(name: u32, out: *mut f32);
        fn wglGetProcAddress(name: *const u8) -> *const c_void;
        fn glGenTextures(count: i32, names: *mut u32);
        fn glDeleteTextures(count: i32, names: *const u32);
        fn glBindTexture(target: u32, name: u32);
        fn glTexParameteri(target: u32, name: u32, value: i32);
        fn glTexImage2D(
            target: u32,
            level: i32,
            internal: i32,
            w: i32,
            h: i32,
            border: i32,
            format: u32,
            kind: u32,
            data: *const c_void,
        );
        fn glViewport(x: i32, y: i32, w: i32, h: i32);
        fn glEnable(state: u32);
        fn glDisable(state: u32);
        fn glBlendFunc(source: u32, destination: u32);
        fn glDepthMask(enabled: u8);
        fn glDepthFunc(mode: u32);
        fn glClear(mask: u32);
        fn glColorMask(r: u8, g: u8, b: u8, a: u8);
        fn glVertexPointer(size: i32, kind: u32, stride: i32, data: *const c_void);
        fn glTexCoordPointer(size: i32, kind: u32, stride: i32, data: *const c_void);
        fn glColorPointer(size: i32, kind: u32, stride: i32, data: *const c_void);
        fn glEnableClientState(state: u32);
        fn glDrawArrays(mode: u32, first: i32, count: i32);
        fn glReadPixels(x: i32, y: i32, w: i32, h: i32, format: u32, kind: u32, data: *mut c_void);

    }
    struct Context {
        window: Handle,
        dc: Handle,
        gl: Handle,
    }
    impl Context {
        fn new() -> Option<Self> {
            unsafe {
                let class: Vec<_> = "STATIC\0".encode_utf16().collect();
                let window = CreateWindowExW(
                    0,
                    class.as_ptr(),
                    class.as_ptr(),
                    0x80000000,
                    0,
                    0,
                    1,
                    1,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
                if window.is_null() {
                    return None;
                }
                let dc = GetDC(window);
                let mut format: PixelFormat = std::mem::zeroed();
                format.size = std::mem::size_of::<PixelFormat>() as u16;
                format.version = 1;
                format.flags = 0x4 | 0x20;
                format.color_bits = 32;
                format.depth_bits = 24;
                let index = ChoosePixelFormat(dc, &format);
                if index == 0 || SetPixelFormat(dc, index, &format) == 0 {
                    ReleaseDC(window, dc);
                    DestroyWindow(window);
                    return None;
                }
                let gl = wglCreateContext(dc);
                if gl.is_null() {
                    ReleaseDC(window, dc);
                    DestroyWindow(window);
                    return None;
                }
                Some(Self { window, dc, gl })
            }
        }
        fn cube(
            &self,
            time: f32,
            projection: glam::Mat4,
            initial: &[u8],
            faces: &[crate::render::gpu::Image; 6],
            mask: u8,
            depth: bool,
        ) -> Option<Vec<u8>> {
            unsafe {
                let previous_dc = wglGetCurrentDC();
                let previous = wglGetCurrentContext();
                if wglMakeCurrent(self.dc, self.gl) == 0 {
                    return None;
                }
                type Gen = unsafe extern "system" fn(i32, *mut u32);
                type Bind = unsafe extern "system" fn(u32, u32);
                type Attach = unsafe extern "system" fn(u32, u32, u32, u32, i32);
                type Storage = unsafe extern "system" fn(u32, u32, i32, i32);
                type AttachDepth = unsafe extern "system" fn(u32, u32, u32, u32);
                type Delete = unsafe extern "system" fn(i32, *const u32);
                let names: [&[u8]; 8] = [
                    b"glGenFramebuffersEXT\0",
                    b"glBindFramebufferEXT\0",
                    b"glFramebufferTexture2DEXT\0",
                    b"glGenRenderbuffersEXT\0",
                    b"glBindRenderbufferEXT\0",
                    b"glRenderbufferStorageEXT\0",
                    b"glFramebufferRenderbufferEXT\0",
                    b"glDeleteFramebuffersEXT\0",
                ];
                let pointers = names.map(|n| wglGetProcAddress(n.as_ptr()));
                if pointers
                    .iter()
                    .any(|p| p.is_null() || (*p as usize) <= 3 || (*p as usize) == usize::MAX)
                {
                    wglMakeCurrent(previous_dc, previous);
                    return None;
                }
                let generate_framebuffer: Gen = std::mem::transmute(pointers[0]);
                let bind: Bind = std::mem::transmute(pointers[1]);
                let attach: Attach = std::mem::transmute(pointers[2]);
                let gen_depth: Gen = std::mem::transmute(pointers[3]);
                let bind_depth: Bind = std::mem::transmute(pointers[4]);
                let storage: Storage = std::mem::transmute(pointers[5]);
                let attach_depth: AttachDepth = std::mem::transmute(pointers[6]);
                let delete: Delete = std::mem::transmute(pointers[7]);
                let delete_depth_pointer =
                    wglGetProcAddress(b"glDeleteRenderbuffersEXT\0".as_ptr());
                if delete_depth_pointer.is_null()
                    || (delete_depth_pointer as usize) <= 3
                    || (delete_depth_pointer as usize) == usize::MAX
                {
                    wglMakeCurrent(previous_dc, previous);
                    return None;
                }
                let delete_depth: Delete = std::mem::transmute(delete_depth_pointer);
                let (mut fbo, mut depth_buffer) = (0, 0);
                generate_framebuffer(1, &mut fbo);
                bind(0x8d40, fbo);
                let mut textures = [0; 7];
                glGenTextures(7, textures.as_mut_ptr());
                glBindTexture(0x0de1, textures[0]);
                glTexImage2D(
                    0x0de1,
                    0,
                    0x8058,
                    256,
                    256,
                    0,
                    0x1908,
                    0x1401,
                    initial.as_ptr().cast(),
                );
                attach(0x8d40, 0x8ce0, 0x0de1, textures[0], 0);
                gen_depth(1, &mut depth_buffer);
                bind_depth(0x8d41, depth_buffer);
                storage(0x8d41, 0x81a6, 256, 256);
                attach_depth(0x8d40, 0x8d00, 0x8d41, depth_buffer);
                type Check = unsafe extern "system" fn(u32) -> u32;
                let check_pointer = wglGetProcAddress(b"glCheckFramebufferStatusEXT\0".as_ptr());
                if check_pointer.is_null()
                    || (check_pointer as usize) <= 3
                    || (check_pointer as usize) == usize::MAX
                {
                    bind(0x8d40, 0);
                    glDeleteTextures(7, textures.as_ptr());
                    delete_depth(1, &depth_buffer);
                    delete(1, &fbo);
                    wglMakeCurrent(previous_dc, previous);
                    return None;
                }
                let check: Check = std::mem::transmute(check_pointer);
                if check(0x8d40) != 0x8cd5 {
                    bind(0x8d40, 0);
                    glDeleteTextures(7, textures.as_ptr());
                    delete_depth(1, &depth_buffer);
                    delete(1, &fbo);
                    wglMakeCurrent(previous_dc, previous);
                    return None;
                }

                for (i, face) in faces.iter().enumerate() {
                    glBindTexture(0x0de1, textures[i + 1]);
                    glTexParameteri(0x0de1, 0x2801, 0x2600);
                    glTexParameteri(0x0de1, 0x2800, 0x2600);
                    glTexParameteri(0x0de1, 0x2802, 0x2901);
                    glTexParameteri(0x0de1, 0x2803, 0x2901);
                    glTexImage2D(
                        0x0de1,
                        0,
                        0x1908,
                        face.width as i32,
                        face.height as i32,
                        0,
                        0x1908,
                        0x1401,
                        face.rgba.as_ptr().cast(),
                    );
                }
                glViewport(0, 0, 256, 256);
                glDepthMask(1);
                glClear(0x100);
                glDepthMask(0);
                glDepthFunc(0x0203);
                if depth {
                    glEnable(0x0b71);
                } else {
                    glDisable(0x0b71);
                }
                glDisable(0x0b44);
                glDisable(0x0bc0);
                glDisable(0x0b50);
                glDisable(0x0b60);
                glEnable(0x0de1);
                glEnable(0x0be2);
                glBlendFunc(0x0302, 0x0303);
                glColorMask(mask & 1, (mask >> 1) & 1, (mask >> 2) & 1, (mask >> 3) & 1);
                glMatrixMode(0x1701);
                glLoadMatrixf(projection.to_cols_array().as_ptr());
                glMatrixMode(0x1700);
                glLoadIdentity();
                glRotatef(180.0, 1.0, 0.0, 0.0);
                let vertices: [f32; 12] = [
                    -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
                ];
                let uv: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
                glVertexPointer(3, 0x1406, 0, vertices.as_ptr().cast());
                glTexCoordPointer(2, 0x1406, 0, uv.as_ptr().cast());
                glEnableClientState(0x8074);
                glEnableClientState(0x8078);
                glEnableClientState(0x8076);
                for sample in 0..64 {
                    glPushMatrix();
                    glTranslatef(
                        ((sample % 8) as f32 / 8.0 - 0.5) / 64.0,
                        ((sample / 8) as f32 / 8.0 - 0.5) / 64.0,
                        0.0,
                    );
                    glRotatef(
                        worldgen::math::sin(time / 400.0) * 25.0 + 20.0,
                        1.0,
                        0.0,
                        0.0,
                    );
                    glRotatef(-time * 0.1, 0.0, 1.0, 0.0);
                    let mut color = [255u8; 16];
                    for i in 0..4 {
                        color[i * 4 + 3] = (255 / (sample + 1)) as u8;
                    }
                    glColorPointer(4, 0x1401, 0, color.as_ptr().cast());
                    for face in 0..6 {
                        glPushMatrix();
                        match face {
                            1 => glRotatef(90.0, 0.0, 1.0, 0.0),
                            2 => glRotatef(180.0, 0.0, 1.0, 0.0),
                            3 => glRotatef(-90.0, 0.0, 1.0, 0.0),
                            4 => glRotatef(90.0, 1.0, 0.0, 0.0),
                            5 => glRotatef(-90.0, 1.0, 0.0, 0.0),
                            _ => {}
                        }
                        glBindTexture(0x0de1, textures[face + 1]);
                        glDrawArrays(0x0007, 0, 4);
                        glPopMatrix();
                    }
                    glPopMatrix();
                    glColorMask(1, 1, 1, 0);
                }
                let mut pixels = vec![0; 256 * 256 * 4];
                glReadPixels(0, 0, 256, 256, 0x1908, 0x1401, pixels.as_mut_ptr().cast());
                bind(0x8d40, 0);
                glDeleteTextures(7, textures.as_ptr());
                delete_depth(1, &depth_buffer);
                delete(1, &fbo);
                wglMakeCurrent(previous_dc, previous);
                Some(pixels)
            }
        }
        fn matrices(
            &self,
            time: f32,
            projection: glam::Mat4,
        ) -> Option<(Vec<glam::Mat4>, Vec<glam::Mat4>)> {
            unsafe {
                let previous_dc = wglGetCurrentDC();
                let previous = wglGetCurrentContext();
                if wglMakeCurrent(self.dc, self.gl) == 0 {
                    return None;
                }
                glMatrixMode(0x1700);
                glLoadIdentity();
                glRotatef(180.0, 1.0, 0.0, 0.0);
                let mut result = Vec::with_capacity(384);
                let mut combined = Vec::with_capacity(384);
                for sample in 0..64 {
                    glPushMatrix();
                    glTranslatef(
                        ((sample % 8) as f32 / 8.0 - 0.5) / 64.0,
                        ((sample / 8) as f32 / 8.0 - 0.5) / 64.0,
                        0.0,
                    );
                    glRotatef(
                        worldgen::math::sin(time / 400.0) * 25.0 + 20.0,
                        1.0,
                        0.0,
                        0.0,
                    );
                    glRotatef(-time * 0.1, 0.0, 1.0, 0.0);
                    for face in 0..6 {
                        glPushMatrix();
                        match face {
                            1 => glRotatef(90.0, 0.0, 1.0, 0.0),
                            2 => glRotatef(180.0, 0.0, 1.0, 0.0),
                            3 => glRotatef(-90.0, 0.0, 1.0, 0.0),
                            4 => glRotatef(90.0, 1.0, 0.0, 0.0),
                            5 => glRotatef(-90.0, 1.0, 0.0, 0.0),
                            _ => {}
                        }
                        let mut m = [0.0; 16];
                        glGetFloatv(0x0ba6, m.as_mut_ptr());
                        result.push(glam::Mat4::from_cols_array(&m));
                        glPushMatrix();
                        glLoadMatrixf(projection.to_cols_array().as_ptr());
                        glMultMatrixf(m.as_ptr());
                        glGetFloatv(0x0ba6, m.as_mut_ptr());
                        combined.push(glam::Mat4::from_cols_array(&m));
                        glPopMatrix();
                        glPopMatrix();
                    }
                    glPopMatrix();
                }
                wglMakeCurrent(previous_dc, previous);
                Some((result, combined))
            }
        }
    }
    impl Drop for Context {
        fn drop(&mut self) {
            unsafe {
                wglDeleteContext(self.gl);
                ReleaseDC(self.window, self.dc);
                DestroyWindow(self.window);
            }
        }
    }
    thread_local! {static CONTEXT:RefCell<Option<Context>>=RefCell::new(Context::new());}
    pub(super) fn cube(
        time: f32,
        projection: glam::Mat4,
        initial: &[u8],
        faces: &[crate::render::gpu::Image; 6],
        mask: u8,
        depth: bool,
    ) -> Option<Vec<u8>> {
        CONTEXT.with(|context| {
            context
                .borrow()
                .as_ref()?
                .cube(time, projection, initial, faces, mask, depth)
        })
    }
    pub(super) fn matrices(
        time: f32,
        projection: glam::Mat4,
    ) -> Option<(Vec<glam::Mat4>, Vec<glam::Mat4>)> {
        CONTEXT.with(|context| context.borrow().as_ref()?.matrices(time, projection))
    }
}
pub(super) fn matrices(
    time: f32,
    projection: glam::Mat4,
) -> Option<(Vec<glam::Mat4>, Vec<glam::Mat4>)> {
    #[cfg(target_os = "windows")]
    {
        native::matrices(time, projection)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (time, projection);
        None
    }
}

pub(super) fn cube(
    time: f32,
    projection: glam::Mat4,
    initial: &[u8],
    faces: &[crate::render::gpu::Image; 6],
    mask: u8,
    depth: bool,
) -> Option<Vec<u8>> {
    #[cfg(target_os = "windows")]
    {
        native::cube(time, projection, initial, faces, mask, depth)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (time, projection, initial, faces, mask, depth);
        None
    }
}
