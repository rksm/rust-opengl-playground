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

const FPS_INTERVAL: u32 = 1000;

pub struct FPSCounter {
    timer: sdl2::TimerSubsystem,
    fps_lasttime: u32,
    fps_frames: u32,
    fps_current: u32,
}

impl FPSCounter {
    pub fn new(mut timer: sdl2::TimerSubsystem) -> Self {
        let fps_lasttime = timer.ticks();
        FPSCounter {
            timer,
            fps_lasttime,
            fps_frames: 0,
            fps_current: 0,
        }
    }

    pub fn count(&mut self) {
        self.fps_frames += 1;
        let ticks = self.timer.ticks();

        if self.fps_lasttime + FPS_INTERVAL < ticks {
            self.fps_lasttime = ticks;
            self.fps_current = self.fps_frames;
            self.fps_frames = 0;
            println!("fps: {}", self.fps_current);
        }
    }
}
