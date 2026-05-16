//! Compilation and linking GLSL, working with uniform.

use crate::engine::graphics::math::matrix4_column_major;
use cgmath::{Matrix4, Vector3};
use std::ffi::CString;

/// Maximum number of directional lights per frame (arrays in the shader).
pub const MAX_DIRECTIONAL_LIGHTS: usize = 4;
/// Maximum number of point lights per frame.
pub const MAX_POINT_LIGHTS: usize = 8;

const VERT_SRC: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    layout (location = 2) in vec3 aNormal;
    layout (location = 3) in vec4 iModel0;
    layout (location = 4) in vec4 iModel1;
    layout (location = 5) in vec4 iModel2;
    layout (location = 6) in vec4 iModel3;
    layout (location = 7) in vec3 iNorm0;
    layout (location = 8) in vec3 iNorm1;
    layout (location = 9) in vec3 iNorm2;
    out vec3 vColor;
    out vec3 vWorldPos;
    out vec3 vNormal;
    uniform mat4 uVP;
    void main()
    {
        mat4 iModel = mat4(iModel0, iModel1, iModel2, iModel3);
        mat3 iNormalMat = mat3(iNorm0, iNorm1, iNorm2);
        vec4 world = iModel * vec4(aPos, 1.0);
        vWorldPos = world.xyz;
        vNormal = normalize(iNormalMat * aNormal);
        gl_Position = uVP * world;
        vColor = aColor;
    }
"#;

const FRAG_SRC: &str = r#"
    #version 330 core
    in vec3 vColor;
    in vec3 vWorldPos;
    in vec3 vNormal;
    out vec4 FragColor;
    uniform vec3 uCameraPos;
    uniform vec3 uMatRgb;
    uniform float uMatAlpha;
    uniform int uUseVertexColor;
    uniform float uSurfAmbient;
    uniform float uSurfDiffuse;
    uniform vec3 uSurfSpecRgb;
    uniform float uSurfShininess;
    uniform int uDirCount;
    uniform vec3 uDirTowardLight[4];
    uniform vec3 uDirRadiance[4];
    uniform int uPointCount;
    uniform vec3 uPointPos[8];
    uniform vec3 uPointRadiance[8];
    uniform vec3 uPointAtten[8];
    void main()
    {
        if (uUseVertexColor != 0) {
            FragColor = vec4(vColor, 1.0);
            return;
        }
        vec3 N = normalize(vNormal);
        vec3 V = normalize(uCameraPos - vWorldPos);
        vec3 albedo = uMatRgb;
        vec3 rgb = albedo * uSurfAmbient;
        for (int i = 0; i < uDirCount; i++) {
            vec3 L = normalize(uDirTowardLight[i]);
            float nl = max(dot(N, L), 0.0);
            vec3 H = normalize(L + V);
            float nh = max(dot(N, H), 0.0);
            vec3 rad = uDirRadiance[i];
            rgb += rad * (uSurfDiffuse * albedo * nl + uSurfSpecRgb * pow(nh, uSurfShininess));
        }
        for (int j = 0; j < uPointCount; j++) {
            vec3 toL = uPointPos[j] - vWorldPos;
            float dist = length(toL);
            vec3 L = toL / max(dist, 1e-5);
            float att = 1.0 / (uPointAtten[j].x + uPointAtten[j].y * dist + uPointAtten[j].z * dist * dist);
            float nl = max(dot(N, L), 0.0);
            vec3 H = normalize(L + V);
            float nh = max(dot(N, H), 0.0);
            vec3 rad = uPointRadiance[j] * att;
            rgb += rad * (uSurfDiffuse * albedo * nl + uSurfSpecRgb * pow(nh, uSurfShininess));
        }
        FragColor = vec4(rgb, uMatAlpha);
    }
"#;

fn uniform_location(program: u32, name: &str) -> i32 {
    let c = CString::new(name).expect("uniform name without NUL");
    unsafe { gl::GetUniformLocation(program, c.as_ptr()) }
}

fn uniform_vec3_array(program: u32, base: &str, len: usize) -> Vec<i32> {
    (0..len)
        .map(|i| uniform_location(program, &format!("{base}[{i}]")))
        .collect()
}

/// Linked OpenGL program and known uniform locations.
pub struct ShaderProgram {
    id: u32,
    vp_location: i32,
    mat_rgb_location: i32,
    mat_alpha_location: i32,
    use_vertex_color_location: i32,
    camera_pos_location: i32,
    surf_ambient_location: i32,
    surf_diffuse_location: i32,
    surf_spec_rgb_location: i32,
    surf_shininess_location: i32,
    dir_count_location: i32,
    dir_toward_light_location: [i32; MAX_DIRECTIONAL_LIGHTS],
    dir_radiance_location: [i32; MAX_DIRECTIONAL_LIGHTS],
    point_count_location: i32,
    point_pos_location: [i32; MAX_POINT_LIGHTS],
    point_radiance_location: [i32; MAX_POINT_LIGHTS],
    point_atten_location: [i32; MAX_POINT_LIGHTS],
}

impl ShaderProgram {
    /// Creates a program with a pair of vertex+color+normal and Blinn–Phong lighting shaders.
    pub fn new_colored_mesh() -> Self {
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            let src = CString::new(VERT_SRC).expect("vertex shader source without NUL");
            gl::ShaderSource(vs, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(vs);

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            let src = CString::new(FRAG_SRC).expect("fragment shader source without NUL");
            gl::ShaderSource(fs, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(fs);

            let id = gl::CreateProgram();
            gl::AttachShader(id, vs);
            gl::AttachShader(id, fs);
            gl::LinkProgram(id);
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            let toward = uniform_vec3_array(id, "uDirTowardLight", MAX_DIRECTIONAL_LIGHTS);
            let rad = uniform_vec3_array(id, "uDirRadiance", MAX_DIRECTIONAL_LIGHTS);
            let mut dir_toward = [0i32; MAX_DIRECTIONAL_LIGHTS];
            let mut dir_rad = [0i32; MAX_DIRECTIONAL_LIGHTS];
            dir_toward[..MAX_DIRECTIONAL_LIGHTS].copy_from_slice(&toward[..MAX_DIRECTIONAL_LIGHTS]);
            dir_rad[..MAX_DIRECTIONAL_LIGHTS].copy_from_slice(&rad[..MAX_DIRECTIONAL_LIGHTS]);

            let pp = uniform_vec3_array(id, "uPointPos", MAX_POINT_LIGHTS);
            let pr = uniform_vec3_array(id, "uPointRadiance", MAX_POINT_LIGHTS);
            let pa = uniform_vec3_array(id, "uPointAtten", MAX_POINT_LIGHTS);
            let mut point_pos = [0i32; MAX_POINT_LIGHTS];
            let mut point_rad = [0i32; MAX_POINT_LIGHTS];
            let mut point_atten = [0i32; MAX_POINT_LIGHTS];
            point_pos[..MAX_POINT_LIGHTS].copy_from_slice(&pp[..MAX_POINT_LIGHTS]);
            point_rad[..MAX_POINT_LIGHTS].copy_from_slice(&pr[..MAX_POINT_LIGHTS]);
            point_atten[..MAX_POINT_LIGHTS].copy_from_slice(&pa[..MAX_POINT_LIGHTS]);

            Self {
                id,
                vp_location: uniform_location(id, "uVP"),
                mat_rgb_location: uniform_location(id, "uMatRgb"),
                mat_alpha_location: uniform_location(id, "uMatAlpha"),
                use_vertex_color_location: uniform_location(id, "uUseVertexColor"),
                camera_pos_location: uniform_location(id, "uCameraPos"),
                surf_ambient_location: uniform_location(id, "uSurfAmbient"),
                surf_diffuse_location: uniform_location(id, "uSurfDiffuse"),
                surf_spec_rgb_location: uniform_location(id, "uSurfSpecRgb"),
                surf_shininess_location: uniform_location(id, "uSurfShininess"),
                dir_count_location: uniform_location(id, "uDirCount"),
                dir_toward_light_location: dir_toward,
                dir_radiance_location: dir_rad,
                point_count_location: uniform_location(id, "uPointCount"),
                point_pos_location: point_pos,
                point_radiance_location: point_rad,
                point_atten_location: point_atten,
            }
        }
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// View×projection matrix (`proj * view`); per-instance model is in vertex attributes.
    pub fn set_vp(&self, vp: &Matrix4<f32>) {
        let cols = matrix4_column_major(vp);
        unsafe {
            gl::UniformMatrix4fv(self.vp_location, 1, gl::FALSE, cols.as_ptr());
        }
    }

    pub fn set_camera_pos(&self, eye: Vector3<f32>) {
        unsafe {
            gl::Uniform3f(self.camera_pos_location, eye.x, eye.y, eye.z);
        }
    }

    /// `true` — fragment color from the vertex attribute (grid lines); `false` — lighted material.
    pub fn set_vertex_color_mode(&self, use_vertex_color: bool) {
        unsafe {
            gl::Uniform1i(
                self.use_vertex_color_location,
                if use_vertex_color { 1 } else { 0 },
            );
        }
    }

    pub fn set_material_rgba(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::Uniform3f(self.mat_rgb_location, r, g, b);
            gl::Uniform1f(self.mat_alpha_location, a);
        }
    }

    pub fn set_surface_lighting(
        &self,
        ambient: f32,
        diffuse: f32,
        spec_r: f32,
        spec_g: f32,
        spec_b: f32,
        shininess: f32,
    ) {
        unsafe {
            gl::Uniform1f(self.surf_ambient_location, ambient);
            gl::Uniform1f(self.surf_diffuse_location, diffuse);
            gl::Uniform3f(self.surf_spec_rgb_location, spec_r, spec_g, spec_b);
            gl::Uniform1f(self.surf_shininess_location, shininess);
        }
    }

    /// Fills the uniform sources; slices are already truncated by the shader limits.
    pub fn set_frame_lights(
        &self,
        dir_toward: &[Vector3<f32>],
        dir_radiance: &[Vector3<f32>],
        point_pos: &[Vector3<f32>],
        point_radiance: &[Vector3<f32>],
        point_atten: &[Vector3<f32>],
    ) {
        debug_assert_eq!(dir_toward.len(), dir_radiance.len());
        debug_assert_eq!(point_pos.len(), point_radiance.len());
        debug_assert_eq!(point_pos.len(), point_atten.len());

        let dir_n = dir_toward.len().min(MAX_DIRECTIONAL_LIGHTS) as i32;
        let point_n = point_pos.len().min(MAX_POINT_LIGHTS) as i32;

        unsafe {
            gl::Uniform1i(self.dir_count_location, dir_n);
            for i in 0..MAX_DIRECTIONAL_LIGHTS {
                let loc_t = self.dir_toward_light_location[i];
                let loc_r = self.dir_radiance_location[i];
                if (i as i32) < dir_n {
                    let t = dir_toward[i];
                    let r = dir_radiance[i];
                    gl::Uniform3f(loc_t, t.x, t.y, t.z);
                    gl::Uniform3f(loc_r, r.x, r.y, r.z);
                } else {
                    gl::Uniform3f(loc_t, 0.0, 1.0, 0.0);
                    gl::Uniform3f(loc_r, 0.0, 0.0, 0.0);
                }
            }

            gl::Uniform1i(self.point_count_location, point_n);
            for i in 0..MAX_POINT_LIGHTS {
                let loc_p = self.point_pos_location[i];
                let loc_r = self.point_radiance_location[i];
                let loc_a = self.point_atten_location[i];
                if (i as i32) < point_n {
                    let p = point_pos[i];
                    let r = point_radiance[i];
                    let a = point_atten[i];
                    gl::Uniform3f(loc_p, p.x, p.y, p.z);
                    gl::Uniform3f(loc_r, r.x, r.y, r.z);
                    gl::Uniform3f(loc_a, a.x, a.y, a.z);
                } else {
                    gl::Uniform3f(loc_p, 0.0, 0.0, 0.0);
                    gl::Uniform3f(loc_r, 0.0, 0.0, 0.0);
                    gl::Uniform3f(loc_a, 1.0, 0.0, 0.0);
                }
            }
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteProgram(self.id);
            }
        }
    }
}
