
use crate::command::{Configuration, Cmd};
use std::ffi::OsString;
use string_builder::Builder;

pub struct BuildCmd {
    pub configuration: Configuration,
    pub framework: Option<String>,
    pub force: bool,

}

impl BuildCmd {
    pub fn new(configuration:Configuration, framework: Option<String>, force: bool) -> Self {
        BuildCmd {
            configuration,
            framework,
            force
        }
    }
    pub fn default() -> Self {
        BuildCmd::new( Configuration::Release, None, false)
    }
}

impl Cmd for BuildCmd {
    fn to_os_string(&self) -> OsString {
        let mut b = Builder::default();
        b.append("dotnet build");
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
        OsString::from(b.string().unwrap())
    }
}