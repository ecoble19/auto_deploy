use crate::command::{Configuration, Cmd};
use std::ffi::OsString;
use string_builder::Builder;
use std::path::PathBuf;

pub struct Logger {

}

pub struct TestCmd {
    pub configuration: Configuration,
    pub log_file_name: PathBuf,
}

impl TestCmd {
    pub fn new(configuration: Configuration, log_file_name: PathBuf) -> Self {
        TestCmd {
            configuration,
            log_file_name
        }
    }
    pub fn default() -> Self {
        TestCmd::new(Configuration::Debug, PathBuf::from("./"))
    }
}

impl Cmd for TestCmd {
    fn to_os_string(&self) -> OsString {
        let mut b = Builder::default();
        b.append("dotnet test");
        b.append(" -c");
        if self.configuration == Configuration::Debug {
            b.append(" Debug");
        } else {
            b.append(" Release");
        }
        b.append(" --no-build");
        b.append(" --logger:\"trx;logFileName=");
        b.append(self.log_file_name.to_str().unwrap());
        b.append("\"");
        OsString::from(b.string().unwrap())
    }
}