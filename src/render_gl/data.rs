use gl;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> Self {
        f32_f32_f32 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        location: usize,
        stride: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev_float {
    pub inner: vec_2_10_10_10::Vector,
}

impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev_float {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        let (x, y, z, w) = other;
        u2_u10_u10_u10_rev_float {
            inner: vec_2_10_10_10::Vector::new(x, y, z, w),
        }
    }
}

impl u2_u10_u10_u10_rev_float {
    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        location: usize,
        stride: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct gl_i8 {
    pub d0: i8,
}

impl From<i8> for gl_i8 {
    fn from(d0: i8) -> Self {
        gl_i8 { d0 }
    }
}

impl gl_i8 {
    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        location: usize,
        stride: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,
            gl::BYTE,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct gl_i8_float {
    pub d0: i8,
}

impl From<i8> for gl_i8_float {
    fn from(d0: i8) -> Self {
        gl_i8_float { d0 }
    }
}

impl gl_i8_float {
    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        location: usize,
        stride: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,
            gl::BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}
