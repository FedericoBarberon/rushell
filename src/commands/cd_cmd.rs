use std::{env, path::PathBuf};

use crate::{
    commands::InvalidArgsError,
    execution::{Executable, ExecutionResult},
};

#[derive(Debug, PartialEq)]
pub struct CdCmd {
    path: PathBuf,
}

impl CdCmd {
    pub fn new(mut path: PathBuf) -> Self {
        if path.starts_with("~")
            && let Some(home_dir) = env::home_dir()
        {
            path = path
                .to_string_lossy()
                .replacen("~", &home_dir.to_string_lossy(), 1)
                .into();
        }

        Self { path }
    }
}

impl Executable for CdCmd {
    fn execute(
        &self,
        _input: &mut impl std::io::prelude::Read,
        _output: &mut impl std::io::prelude::Write,
        error: &mut impl std::io::prelude::Write,
    ) -> ExecutionResult {
        if !self.path.is_dir() {
            let _ = writeln!(
                error,
                "cd: {}: No such file or directory",
                self.path.to_string_lossy()
            );
        } else {
            let _ = env::set_current_dir(self.path.clone());
        }

        ExecutionResult::Continue
    }
}

impl TryFrom<&[String]> for CdCmd {
    type Error = InvalidArgsError;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        match value {
            [] => Ok(Self::new(PathBuf::new())),
            [path] => Ok(Self::new(PathBuf::from(path))),
            _ => Err(InvalidArgsError::TooManyArgs),
        }
    }
}

impl TryFrom<Vec<String>> for CdCmd {
    type Error = InvalidArgsError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use crate::{
        execution::ExecutionResult,
        tests::utilities::{TestBuffers, build_args},
    };

    use super::*;

    struct TmpDirGuard {
        path: PathBuf,
    }

    impl TmpDirGuard {
        fn create(path: PathBuf) -> std::io::Result<Self> {
            fs::create_dir_all(&path)?;

            Ok(Self { path })
        }
    }

    impl Drop for TmpDirGuard {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    struct CwdGuard {
        original: PathBuf,
    }

    impl CwdGuard {
        fn capture() -> Self {
            let cwd = env::current_dir().unwrap();

            Self { original: cwd }
        }
    }

    impl Drop for CwdGuard {
        fn drop(&mut self) {
            let _ = env::set_current_dir(&self.original);
        }
    }

    #[test]
    fn from_valid_args() {
        let args = build_args(&["path"]);
        let cmd = CdCmd::try_from(args).unwrap();

        assert_eq!(cmd, CdCmd::new(PathBuf::from("path")));
    }

    #[test]
    fn fails_from_multiple_args() {
        let args = build_args(&["foo", "bar"]);
        let err = CdCmd::try_from(args).unwrap_err();

        assert_eq!(err, InvalidArgsError::TooManyArgs);
    }

    #[test]
    fn execute_with_absolute_path() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);

        let _guard = CwdGuard::capture();

        let path = PathBuf::from("/");
        let cmd = CdCmd::new(path.clone());

        assert_eq!(
            cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        let new_cwd = env::current_dir().unwrap();

        assert_eq!(new_cwd, path);

        assert!(output.is_empty());
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }

    #[test]
    fn execute_with_relative_current_path() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);

        let guard = CwdGuard::capture();

        let path = PathBuf::from("./");
        let cmd = CdCmd::new(path.clone());

        assert_eq!(
            cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        let new_cwd = env::current_dir().unwrap();

        assert_eq!(new_cwd, guard.original);

        assert!(output.is_empty());
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }

    #[test]
    fn execute_with_relative_parent_path() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);

        let _guard = CwdGuard::capture();

        env::set_current_dir("/home").unwrap();

        let path = PathBuf::from("../");
        let cmd = CdCmd::new(path.clone());

        assert_eq!(
            cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        let new_cwd = env::current_dir().unwrap();

        assert_eq!(new_cwd, PathBuf::from("/"));

        assert!(output.is_empty());
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }

    #[test]
    fn execute_with_relative_current_path_shorthand() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);

        let _guard = CwdGuard::capture();

        env::set_current_dir("/").unwrap();

        let path = PathBuf::from("home");
        let cmd = CdCmd::new(path.clone());

        assert_eq!(
            cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        let new_cwd = env::current_dir().unwrap();

        assert_eq!(new_cwd, PathBuf::from("/home"));

        assert!(output.is_empty());
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }

    #[test]
    fn execute_with_relative_path() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);

        let _guard = CwdGuard::capture();

        env::set_current_dir("/").unwrap();

        let path = PathBuf::from("./home/..");
        let cmd = CdCmd::new(path.clone());

        assert_eq!(
            cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        let new_cwd = env::current_dir().unwrap();

        assert_eq!(new_cwd, PathBuf::from("/"));

        assert!(output.is_empty());
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }

    #[test]
    fn execute_with_home_alias() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);

        let _guard = CwdGuard::capture();

        let path = PathBuf::from("~");
        let cmd = CdCmd::new(path.clone());

        assert_eq!(
            cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        let new_cwd = env::current_dir().unwrap();
        let home_dir = env::home_dir().unwrap();

        assert_eq!(new_cwd, home_dir);

        assert!(output.is_empty());
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }

    #[test]
    fn execute_with_home_alias_with_extra_path() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);

        let _cwd_guard = CwdGuard::capture();

        let home_dir = env::home_dir().unwrap();
        let mut tmp_dir = home_dir.clone();
        tmp_dir.push("rushell_test_tmp_dir");

        let _tmp_guard = TmpDirGuard::create(tmp_dir.clone()).unwrap();

        let path = PathBuf::from("~/rushell_test_tmp_dir");
        let cmd = CdCmd::new(path.clone());

        assert_eq!(
            cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        let new_cwd = env::current_dir().unwrap();

        assert_eq!(new_cwd, tmp_dir);

        assert!(output.is_empty());
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }

    #[test]
    fn execute_with_invalid_path_prints_error_and_does_not_change_cwd() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);

        let guard = CwdGuard::capture();

        let path = PathBuf::from("some/probably/invalid/path");
        let cmd = CdCmd::new(path.clone());

        assert_eq!(
            cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        let new_cwd = env::current_dir().unwrap();

        assert_eq!(new_cwd, guard.original);

        assert!(output.is_empty());
        assert!(input.get_ref().is_empty());
        assert_eq!(
            String::from_utf8_lossy(&error),
            format!(
                "cd: {}: No such file or directory\n",
                path.to_string_lossy()
            )
        );
    }
}
