
use crate::{Configuration};
use string_builder::Builder;

/// Commonly used commands for building a .NET project
pub struct BuildCmd {

    /// Sets the build configuration for the project
    /// `crate::Configuration::Debug`, `Configuration::Release`
    ///
    /// dotnet build `-c [--configuration]`
    pub configuration: Configuration,

    /// Sets the build target if set else it will pull it from the sln/csproj file
    ///
    /// dotnet build `-f [--framework]`
    pub framework: Option<String>,

    /// Sets the output directory. defaults to the projects bin folder
    ///
    /// dotnet build -o `--output`
    pub output_dir: Option<String>,

    /// Forces all dependencies to be resolved even if the last restore was successful
    ///
    /// dotnet build `--force`
    pub force: bool,
}

impl BuildCmd {
    /// Creates struct with necessary information to build a project
    pub fn new(configuration:Configuration, framework: Option<String>, output_dir: Option<String>, force: bool) -> Self {
        Self {
            configuration,
            framework,
            output_dir,
            force
        }
    }
    /// Creates default configuration for minimalistic build options
    ///
    /// `dotnet build -c Release`
    pub fn default() -> Self {
        BuildCmd::new(Configuration::Release, None, None, false)
    }
}

impl BuildCmd {
    /// Returns the command line arguments that would be used to build an application
    pub fn to_cmd(&self) -> String {
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
        if self.output_dir.is_some() {
            b.append(" -o ");
            b.append(self.output_dir.as_ref().unwrap().clone());
        }
        if self.force {
            b.append(" --force");
        }
        b.string().unwrap()
    }
}