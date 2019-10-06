use crate::render_gl;
use crate::render_gl::buffer;
use crate::render_gl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
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

pub struct Rectangle {
    program: render_gl::Program,
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
}

impl Rectangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Self, failure::Error> {
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
                pos: (0.5, 0.5, 0.0).into(),
                clr: (1.0, 0.5, 1.0, 1.0).into(),
            },
            Vertex {
                pos: (-0.5, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
            },
        ];
        let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];

        let vao = VertexArray::new(gl);
        let buffer = ArrayBuffer::new(gl);
        let element_buffer = ElementArrayBuffer::new(gl);

        vao.bind();

        buffer.bind();
        buffer.static_draw(&vertices);
        Vertex::vertex_attrib_pointers(&gl);

        element_buffer.bind();
        element_buffer.static_draw(&indices);

        vao.unbind();

        buffer.unbind();
        element_buffer.unbind();

        Ok(Rectangle {
            program: Program::from_res(&gl, &res, "shaders/triangle")?,
            vao,
            _vbo: buffer,
            _ebo: element_buffer,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();
        unsafe {
            gl.DrawElements(
                gl::TRIANGLE_STRIP,
                6,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
        }
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

use std::path::PathBuf;

impl Reloadable for Rectangle {
    fn reload(&mut self, gl: &gl::Gl, res: &Resources) -> Result<(), failure::Error> {
        println!("reloading rectangle");
        Program::from_res(&gl, &res, "shaders/triangle")
            .map(|program| self.program = program)
            .unwrap_or_else(|err| {
                println!("Failed to reload rectangle. {:?}", err);
            });

        Ok(())
    }

    fn get_paths(&self) -> &[PathBuf] {
        &self.program.paths
    }
}
