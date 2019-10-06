use failure::Fail;
use std::ffi::CString;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{ffi, fs, io};

use crossbeam_channel::{unbounded, Receiver};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

pub fn absolute_path<P>(path: P) -> Result<PathBuf, Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to get executable path")]
    FaildToGetExecPath,
    #[fail(display = "IO error")]
    Io(#[cause] io::Error),
    #[fail(display = "Failed to read CString from file that contains 0")]
    FileContainsNil,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#[derive(Debug)]
pub struct Source {
    pub content: ffi::CString,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Resources {
    root_path: PathBuf,
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();
    for part in location.split("/") {
        path = path.join(part);
    }
    path
}

impl Resources {
    pub fn from_relative_exe_path(path: &Path) -> Result<Resources, Error> {
        let exec_file_name = ::std::env::current_exe().map_err(|_| Error::FaildToGetExecPath)?;
        let exe_path = exec_file_name.parent().ok_or(Error::FaildToGetExecPath)?;
        Ok(Resources {
            root_path: exe_path.join(path),
        })
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<Source, Error> {
        let path = resource_name_to_path(&self.root_path, resource_name);
        let mut file = fs::File::open(path.to_path_buf())?;
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }
        Ok(Source {
            path,
            content: unsafe { ffi::CString::from_vec_unchecked(buffer) },
        })
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

pub trait Reloadable {
    fn get_paths(&self) -> &[PathBuf];
    fn reload(&mut self, gl: &gl::Gl, res: &Resources) -> Result<(), failure::Error>;
}

pub struct ResourceWatcher {
    watcher: RecommendedWatcher,
    pub rx: Receiver<notify::RawEvent>,
}

impl ResourceWatcher {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        let watcher: RecommendedWatcher = Watcher::new_immediate(tx).unwrap();
        Self { watcher, rx }
    }

    // pub fn add_reloadable<R: Reloadable, V: IntoIterator<Item = PathBuf> + Sized>(
    pub fn add_reloadable(&mut self, res: &dyn Reloadable) {
        println!("hmmmm {:?}", res.get_paths());
        for p in res.get_paths() {
            let p = absolute_path(p).unwrap();
            println!("watching {:#?}", p);
            self.watcher.watch(&p, RecursiveMode::Recursive).unwrap();
        }
    }
}
