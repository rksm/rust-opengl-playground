extern crate gl;
extern crate sdl2;

pub mod render_gl;

use render_gl::{Program, Shader};
use std::ffi::CString;

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

fn create_shader_program() -> Result<Program, String> {
    let v_source = CString::new(include_str!("triangle.vert"))
        .or(Err("Vertex shader contains non-utf8 content."))?;
    let f_source = CString::new(include_str!("triangle.frag"))
        .or(Err("Fragment shader contains non-utf8 content."))?;
    let v_shader = Shader::vertex_from_source(&v_source)?;
    let f_shader = Shader::fragment_from_source(&f_source)?;
    let p = Program::from_shaders(&[v_shader, f_shader])?;
    p.set_used();
    Ok(p)
}

fn create_triangle() -> gl::types::GLuint {
    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    vao
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video
        .window("test", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let program = create_shader_program().unwrap();
    let vao = create_triangle();

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

    'main: loop {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.gl_swap_window();

        for event in event_pump.poll_iter() {
            use sdl2::event::Event::{Quit, Window};
            use sdl2::event::WindowEvent::Resized;
            match event {
                Quit { .. } => break 'main,
                Window { win_event, .. } => match win_event {
                    Resized(w, h) => unsafe {
                        gl::Viewport(0, 0, w, h);
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
