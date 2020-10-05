
mod build;
mod test;
mod deploy;
pub use build::BuildCmd;
pub use test::TestCmd;
pub use deploy::DeployCmd;
//pub mod test;

#[cfg(test)]
mod dotnet {
    use std::path::PathBuf;
    use crate::dotnet::{BuildCmd, TestCmd, DeployCmd};
    use crate::command::Cmd;

    #[test]
    fn build_command_works() {
        let b = BuildCmd::default();
        let cmd = b.to_os_string();
        assert_eq!(cmd, "dotnet build -c Release");
    }

    #[test]
    fn test_command_works() {
        let mut b = TestCmd::default();
        b.log_file_name = PathBuf::from("/test/directory/output.xml");
        let cmd = b.to_os_string();
        assert_eq!(cmd, "dotnet test -c Debug --no-build --logger:\"trx;logFileName=/test/directory/output.xml\"");
    }

    #[test]
    fn deploy_command_works() {
        let mut b = DeployCmd::default();
        b.location = PathBuf::from("/test/directory");
        let cmd = b.to_os_string();
        assert_eq!(cmd, "dotnet publish -c Release --no-build -o /test/directory");
    }
}
