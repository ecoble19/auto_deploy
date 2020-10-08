
use std::process::{Command, Output};
use std::{error};
use std::fmt;
use std::path::{Path, PathBuf};
use std::string::String;

pub mod dotnet;

#[derive(PartialEq)]
pub enum Configuration {
    Debug,
    Release
}

#[derive(Debug)]
struct InvalidOperationError {
    details: String
}

impl InvalidOperationError {
    fn new(msg: &str) -> Self {
        InvalidOperationError{details: msg.to_string()}
    }
}

impl fmt::Display for InvalidOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl error::Error for InvalidOperationError {
    fn description(&self) -> &str {
        &self.details
    }
}

// Change the alias to `Box<error::Error>`.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error + Send>>;

pub trait AutoDeploy {
    fn build(&self) -> String;
    fn test(&self) -> String;
    fn deploy(&self) -> String;

}

pub struct Project<'a> {
    pub name: String,
    dir: PathBuf,
    built: bool,
    commands: &'a dyn AutoDeploy
}

fn run_command(p: &Path, c: &String) -> Result<Output> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir(p)
            .arg("/C")
            .arg(c)
            .output()
    } else {
        Command::new("sh")
            .current_dir(p)
            .arg("-c")
            .arg(c)
            .output()
    };
    match output {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(e))
    }
}

impl <'a> Project<'a> {

    pub fn new(name: String, dir: &str, commands: &'a dyn AutoDeploy) -> Self {
        Self {
            name,
            dir: PathBuf::from(dir),
            built: false,
            commands
        }
    }

    // pub fn project_dir(&mut self, p: PathBuf) {
    //     self.dir.push(p);
    // }
    //
    // /// Adds build command
    // pub fn build_command(&mut self, s: String) -> &mut Self {
    //     self.commands.build = s;
    //     self
    // }
    //
    // /// Adds test command
    // pub fn test_command(&mut self, s: String) -> &mut Self {
    //     self.commands.test = s;
    //     self
    // }
    //
    // pub fn deploy_command(&mut self, s: String) -> &mut Self {
    //     self.commands.deploy = s;
    //     self
    // }

    ///Builds the project before the tests can be run
    pub fn build(&mut self) -> Result<()> {
        let r = run_command(&self.dir, &self.commands.build());
        match r {
            Ok(o) => {
                self.built = true;
                if !o.status.success() {
                    return Err(Box::new(InvalidOperationError::new(&String::from_utf8(o.stdout).unwrap())));
                }
                Ok(())
            },
            Err(e) => {
                self.built = false;
                Err(e)
            }
        }
    }
    ///This function should execute unit tests or any other method of testing on a project
    /// and return a result whether all tests passed or not
    pub fn test(&self) -> Result<()> {
        match run_command(&self.dir, &self.commands.test()) {
            Ok(o) => {
                if !o.status.success() {
                    return Err(Box::new(InvalidOperationError::new(&String::from_utf8(o.stdout).unwrap())));
                }
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    ///Uses the implemented deployment method to deploy the project reporting an error
    /// if applicable
    pub fn deploy(&self) -> Result<()> {
        if !self.built {
            return Err(Box::new(InvalidOperationError::new("Project has not been built")));
        }
        match run_command(&self.dir, &self.commands.deploy()) {
            Ok(o) => {
                if !o.status.success() {
                    return Err(Box::new(InvalidOperationError::new(&String::from_utf8(o.stdout).unwrap())));
                }
                Ok(())
            },
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Project, ProjectCommand, run_command};
    use std::str;
    use std::ffi::{String};
    use std::error::Error;
    use std::process::{Command, ExitStatus, Output};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_works() {
        let command = String::from("echo Hello, world");
        let p = Project {
            name: "Some project".to_string(),
            dir: PathBuf::from("./"),
            built: false,
            commands: ProjectCommand {
                build: command.clone(),
                test: command.clone(),
                deploy: command.clone()
            }
        };
        match p.test() {
            Ok(r) => assert!(true),
            Err(e) => assert!(false, e)
        }
    }

    #[test]
    fn deploy_fails_with_no_build() {
        let command = String::from("echo Hello, world");
        let mut p = Project {
            name: "Some project".to_string(),
            built: false,
            dir: Path::new("./").to_owned(),
            commands: ProjectCommand {
                build: command.clone(),
                test: command.clone(),
                deploy: command.clone()
            }
        };
        match p.deploy() {
            Ok(r) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), "Project has not been built")
        }
    }

    #[test]
    fn deploy_succeeds_with_build() {
        let command = String::from("echo Hello, world");
        let mut p = Project {
            name: "Some project".to_string(),
            built: true,
            dir: Path::new("./").to_owned(),
            commands: ProjectCommand {
                build: command.clone(),
                test: command.clone(),
                deploy: command.clone()
            }
        };
        match p.deploy() {
            Ok(r) => assert!(true),
            Err(e) => assert!(false, "Project should run build command")
        }
    }

    #[test]
    fn build_sets_built() {
        let command = String::from("echo Hello, world");
        let mut p = Project {
            name: "Some project".to_string(),
            built: false,
            dir: Path::new("./").to_owned(),
            commands: ProjectCommand {
                build: command.clone(),
                test: command.clone(),
                deploy: command.clone()
            }
        };
        p.build();
        assert!(p.built, "Built boolean should be set");
    }

    #[test]
    fn bad_command_fails() {
        match run_command(Path::new("./"), &String::from("eco Hello"))
            .unwrap().status.success() {
            true => assert!(false, "`eco` should not be recognized"),
            false => assert!(true)
        }
    }

    #[test]
    fn good_command_succeeds() {
        match run_command(Path::new("./"), &String::from("echo Hello"))
            .unwrap().status.success() {
            true => assert!(true),
            false => assert!(false, "`echo` should be recognized"),
        }
    }

    // #[test]
    // fn test_command() {
    //     let s: Output = Command::new("sh")
    //         .arg("-c")
    //         .current_dir("/home/erick/work/lpp")
    //         .arg("dotnet build")
    //         .output().unwrap();
    //     assert_eq!(s.status.success(), true);
    //     assert_eq!(String::from_utf8(s.stdout).unwrap(), "");
    // }
}

