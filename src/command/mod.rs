use std::ffi::OsString;

#[derive(PartialEq)]
pub enum Configuration {
    Debug,
    Release
}

pub trait Cmd {
    fn to_os_string(&self) -> OsString;
}