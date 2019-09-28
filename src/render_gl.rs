use crate::resources::{self, Resources};
use gl;
use std;
use std::ffi::{CStr, CString};

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
// helper
fn create_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#[derive(Debug)]
pub enum Error {
    LinkError {
        message: String,
    },
    CompileError {
        message: String,
    },
    CannotDetermineShaderTypeForResource {
        name: String,
    },
    ResourceLoad {
        name: String,
        inner: resources::Error,
    },
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];
        let shaders = POSSIBLE_EXT
            .iter()
            .map(|file_ext| Shader::from_res(gl, res, &format!("{}{}", name, file_ext)))
            .collect::<Result<Vec<Shader>, Error>>()?;
        Self::from_shaders(gl, &shaders)
    }

    fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, Error> {
        let id = unsafe { gl.CreateProgram() };
        for s in shaders {
            unsafe {
                gl.AttachShader(id, s.id());
            }
        }
        unsafe {
            gl.LinkProgram(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error: CString = create_cstring_with_len(len as usize);
            unsafe {
                gl.GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }
            return Err(Error::LinkError {
                message: error.to_string_lossy().into_owned(),
            });
        }

        for s in shaders {
            unsafe {
                gl.DetachShader(id, s.id());
            }
        }

        Ok(Program { gl: gl.clone(), id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] =
            [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];
        let shader_kind = POSSIBLE_EXT
            .iter()
            .find(|&&(file_ext, _)| name.ends_with(file_ext))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::CannotDetermineShaderTypeForResource { name: name.into() })?;
        let source = res.load_cstring(name).map_err(|e| Error::ResourceLoad {
            name: name.into(),
            inner: e,
        })?;
        Shader::from_source(gl, &source, shader_kind)
    }

    fn from_source(gl: &gl::Gl, source: &CStr, kind: gl::types::GLuint) -> Result<Shader, Error> {
        let id = shader_from_source(gl, source, kind)?;
        Ok(Shader { gl: gl.clone(), id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn shader_from_source(
    gl: &gl::Gl,
    source: &CStr,
    kind: gl::types::GLuint,
) -> Result<gl::types::GLuint, Error> {
    let id = unsafe { gl.CreateShader(kind) };
    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        let error: CString = create_cstring_with_len(len as usize);
        unsafe {
            gl.GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }
        return Err(Error::CompileError {
            message: error.to_string_lossy().into_owned(),
        });
    }

    Ok(id)
}
