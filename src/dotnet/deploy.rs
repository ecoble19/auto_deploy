
use crate::command::{Configuration, Cmd};
use std::ffi::OsString;
use string_builder::Builder;
use std::path::PathBuf;

pub struct DeployCmd {
    pub location: PathBuf,
    pub configuration: Configuration,
    pub framework: Option<String>,
    pub force: bool,

}

impl DeployCmd {
    pub fn new(location: PathBuf, configuration:Configuration, framework: Option<String>, force: bool) -> Self {
        DeployCmd {
            location,
            configuration,
            framework,
            force
        }
    }
    pub fn default() -> Self {
        DeployCmd::new(PathBuf::from("./"), Configuration::Release, None, false)
    }
}

impl Cmd for DeployCmd {
    fn to_os_string(&self) -> OsString {
        let mut b = Builder::default();
        b.append("dotnet publish");
        b.append(" -c");
        if self.configuration == Configuration::Debug {
            b.append(" Debug");
        }
        else {
            b.append(" Release");
        }
        if self.framework.is_some() {
            b.append(" -f ");
            b.append(self.framework.as_ref().unwrap().clone());
        }
        if self.force {
            b.append(" --force");
        }
        b.append(" --no-build");
        b.append(" -o");
        b.append(" ");
        b.append(self.location.to_str().unwrap());
        OsString::from(b.string().unwrap())
    }
}