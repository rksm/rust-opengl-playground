use crate::render_gl;
use crate::render_gl::buffer;
use crate::render_gl::buffer::{ArrayBuffer, VertexArray};
use crate::render_gl::data;
use crate::render_gl::Program;
use crate::resources::{Reloadable, Resources};
use failure;
use gl;
use render_gl_derive::VertexAttribPointers;

#[derive(VertexAttribPointers, Debug, Clone, Copy)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub struct Triangle {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Self, failure::Error> {
        let program = Program::from_res(&gl, &res, "shaders/triangle")?;
        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
            },
            Vertex {
                pos: (0.0, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
            },
        ];

        let vao = VertexArray::new(gl);
        let buffer = ArrayBuffer::new(gl);
        vao.bind();
        buffer.bind();
        buffer.static_draw(&vertices);
        buffer.unbind();

        buffer.bind();
        Vertex::vertex_attrib_pointers(&gl);
        buffer.unbind();
        vao.unbind();

        Ok(Triangle {
            program,
            vao,
            _vbo: buffer,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();
        unsafe {
            gl.DrawArrays(
                //mode
                gl::TRIANGLES,
                // starting index in the enabled arrays
                0,
                // number of incies to be rendered
                3,
            );
        }
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

use std::path::{Path, PathBuf};

impl Reloadable for Triangle {
    fn reload(&mut self, gl: &gl::Gl, res: &Resources) -> Result<(), failure::Error> {
        println!("reloading triangle");
        Program::from_res(&gl, &res, "shaders/triangle")
            .map(|program| self.program = program)
            .unwrap_or_else(|err| {
                println!("Failed to reload triangle. {:?}", err);
            });

        Ok(())
    }

    fn get_paths(&self) -> Vec<PathBuf> {
        self.program.paths.clone()
    }
}
