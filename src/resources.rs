use std::ffi::CString;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{ffi, fs, io};
use failure::Fail;

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

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }
        Ok(unsafe { CString::from_vec_unchecked(buffer) })
    }
}
