pub mod render_gl;
pub mod resources;

use failure;
use render_gl::data;
use render_gl::Program;
use render_gl_derive::VertexAttribPointers;
use resources::Resources;
use std::path::Path;

pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();
    for (i, cause) in e
        .iter_chain()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
    {
        if i > 0 {
            writeln!(&mut result, "  Which caused:").unwrap();
        }
        write!(&mut result, "{}", cause).unwrap();
        if let Some(backtrace) = cause.backtrace() {
            let backtrace_sir = format!("{}", backtrace);
            if backtrace_sir.len() > 0 {
                writeln!(&mut result, " This happened at {}", backtrace).unwrap();
            } else {
                writeln!(&mut result).unwrap();
            }
        } else {
            writeln!(&mut result).unwrap();
        }
    }

    result
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#[derive(VertexAttribPointers, Debug, Clone, Copy)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
}

fn create_triangle(gl: &gl::Gl) -> gl::types::GLuint {
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
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

        Vertex::vertex_attrib_pointers(&gl);
        gl.BindVertexArray(0);
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    vao
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

const WINDOW_TITLE: &str = "OpenGL ";

struct State {
    _sdl: sdl2::Sdl,
    gl: gl::Gl,
    _gl_context: sdl2::video::GLContext,
    program: Program,
    vao: gl::types::GLuint,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
}

fn setup() -> Result<State, failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets"))?;
    let sdl = sdl2::init().map_err(failure::err_msg)?;
    let video = sdl.video().map_err(failure::err_msg)?;

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video
        .window(WINDOW_TITLE, 800, 600)
        .opengl()
        .resizable()
        .build()
        .map_err(failure::err_msg)?;
    let event_pump = sdl.event_pump().map_err(failure::err_msg)?;

    let gl_context = window.kgl_create_context().map_err(failure::err_msg)?;
    let gl = gl::Gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let program = Program::from_res(&gl, &res, "shaders/triangle")?;
    let vao = create_triangle(&gl);

    unsafe {
        gl.Viewport(0, 0, 800, 600);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    Ok(State {
        _sdl: sdl,
        _gl_context: gl_context,
        gl,
        program,
        vao,
        window,
        event_pump,
    })
}

fn run(state: State) -> Result<(), failure::Error> {
    match state {
        State {
            gl,
            program,
            vao,
            window,
            mut event_pump,
            ..
        } => 'main: loop {
            unsafe {
                gl.Clear(gl::COLOR_BUFFER_BIT);
            }
            program.set_used();
            unsafe {
                gl.BindVertexArray(vao);
                gl.DrawArrays(gl::TRIANGLES, 0, 3);
            }
            window.gl_swap_window();

            for event in event_pump.poll_iter() {
                use sdl2::event::Event::{Quit, Window};
                use sdl2::event::WindowEvent::Resized;
                match event {
                    Quit { .. } => break 'main,
                    Window { win_event, .. } => match win_event {
                        Resized(w, h) => unsafe {
                            gl.Viewport(0, 0, w, h);
                        },
                        _ => {}
                    },
                    _ => {}
                }
            }
        },
    };
    Ok(())
}

fn main() {
    if let Err(e) = setup().and_then(run) {
        println!("{}", failure_to_string(e));
        std::process::exit(1);
    }
}
