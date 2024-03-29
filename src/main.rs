mod debug;
mod rectangle;
pub mod render_gl;
pub mod resources;
mod triangle;

use debug::{failure_to_string, FPSCounter};
use failure;
use nalgebra as na;
use rectangle::Rectangle;
use render_gl::ColorBuffer;
use render_gl::Viewport;
use resources::{Reloadable, ResourceWatcher, Resources};
use std::path::Path;
use triangle::Triangle;

const WINDOW_TITLE: &str = "OpenGL ";

struct State {
    _sdl: sdl2::Sdl,
    gl: gl::Gl,
    _gl_context: sdl2::video::GLContext,
    triangle: Triangle,
    rectangle: Rectangle,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    viewport: Viewport,
    color_buffer: ColorBuffer,
    fps_counter: FPSCounter,
    watcher: ResourceWatcher,
    resources: Resources,
}

fn setup() -> Result<State, failure::Error> {
    let sdl = sdl2::init().map_err(failure::err_msg)?;

    let timer = sdl.timer().map_err(failure::err_msg)?;
    let fps_counter = FPSCounter::new(timer);

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

    let gl_context = window.gl_create_context().map_err(failure::err_msg)?;
    let gl = gl::Gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let resources = Resources::from_relative_exe_path(Path::new("assets"))?;
    let triangle = Triangle::new(&resources, &gl)?;
    let rectangle = Rectangle::new(&resources, &gl)?;
    let viewport = Viewport::for_window(800, 600);

    viewport.set_used(&gl);

    let color_buffer = ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
    color_buffer.set_used(&gl);

    let mut watcher = ResourceWatcher::new();
    watcher.add_reloadable(&triangle);
    watcher.add_reloadable(&rectangle);

    Ok(State {
        _sdl: sdl,
        _gl_context: gl_context,
        gl,
        triangle,
        rectangle,
        window,
        event_pump,
        viewport,
        color_buffer,
        fps_counter,
        watcher,
        resources,
    })
}

fn run(state: State) -> Result<(), failure::Error> {
    match state {
        State {
            gl,
            mut triangle,
            mut rectangle,
            window,
            mut event_pump,
            mut viewport,
            color_buffer,
            mut fps_counter,
            resources,
            watcher,
            ..
        } => 'main: loop {
                       match watcher.rx.try_recv() {
                           Ok(evt) => {
                               println!("{:#?}", evt);
                               triangle.reload(&gl, &resources)?;
                               rectangle.reload(&gl, &resources)?;
                           }
                           Err(_) => {}
                       }

            fps_counter.count();
            color_buffer.clear(&gl);
            // triangle.render(&gl);
            rectangle.render(&gl);
            window.gl_swap_window();

            for event in event_pump.poll_iter() {
                use sdl2::event::Event::{Quit, Window};
                use sdl2::event::WindowEvent::Resized;
                match event {
                    Quit { .. } => break 'main,
                    Window { win_event, .. } => match win_event {
                        Resized(w, h) => {
                            viewport.update_size(w, h);
                            viewport.set_used(&gl);
                        }
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
        println!("Error occurred: {}", failure_to_string(e));
        std::process::exit(1);
    }
}
