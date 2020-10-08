//! This module contains the logic for building, testing, and publishing .NET projects

mod build;
mod test;
mod deploy;
pub use build::BuildCmd;
pub use test::TestCmd;
pub use deploy::DeployCmd;
use crate::AutoDeploy;

pub struct DotNet {
    build_cmd: BuildCmd,
    test_cmd: TestCmd,
    deploy_cmd: DeployCmd
}

impl AutoDeploy for DotNet {
    fn build(&self) -> String {
        self.build_cmd.to_cmd()
    }

    fn test(&self) -> String {
        self.test_cmd.to_cmd()
    }

    fn deploy(&self) -> String {
        self.deploy_cmd.to_cmd()
    }
}


#[cfg(test)]
mod dotnet {
    use std::path::PathBuf;
    use crate::dotnet::{BuildCmd, TestCmd, DeployCmd};
    use crate::command::Cmd;
    use crate::Cmd;

    #[test]
    fn build_command_works() {
        let b = BuildCmd::default();
        let cmd = b.to_cmd();
        assert_eq!(cmd, "dotnet build -c Release");
    }

    #[test]
    fn test_command_works() {
        let mut b = TestCmd::default();
        b.log_file_name = PathBuf::from("/test/directory/output.xml");
        let cmd = b.to_cmd();
        assert_eq!(cmd, "dotnet test -c Debug --no-build --logger:\"trx;logFileName=/test/directory/output.xml\"");
    }

    #[test]
    fn deploy_command_works() {
        let mut b = DeployCmd::default();
        b.location = PathBuf::from("/test/directory");
        let cmd = b.to_cmd();
        assert_eq!(cmd, "dotnet publish -c Release --no-build -o /test/directory");
    }
}
