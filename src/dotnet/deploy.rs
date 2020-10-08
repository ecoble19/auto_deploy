
use crate::{Configuration};
use string_builder::Builder;
use std::path::PathBuf;

pub struct DeployCmd {
    pub location: PathBuf,
    pub configuration: Configuration,
    pub framework: Option<String>,
    pub folder_profile: Option<String>,
    pub force: bool,

}

impl DeployCmd {
    pub fn new(location: PathBuf, configuration:Configuration, framework: Option<String>, folder_profile: Option<String>, force: bool) -> Self {
        DeployCmd {
            location,
            configuration,
            framework,
            folder_profile,
            force
        }
    }
    pub fn default() -> Self {
        DeployCmd::new(PathBuf::from("./"), Configuration::Release, None,None, false)
    }
}

impl DeployCmd {
    pub fn to_cmd(&self) -> String {
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
        if self.folder_profile.is_some() {
            b.append(" -p:PublishProfile=");
            b.append(self.folder_profile.as_ref().unwrap().to_owned());
        }
        b.append(" --no-build");
        b.append(" -o");
        b.append(" ");
        b.append(self.location.to_str().unwrap());
        b.string().unwrap()
    }
}