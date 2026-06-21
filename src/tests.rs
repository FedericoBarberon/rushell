#[cfg(test)]
pub mod utilities {
    use std::{
        env,
        ffi::OsString,
        fs,
        io::Cursor,
        os::unix::fs::PermissionsExt,
        path::{Path, PathBuf},
    };

    pub struct TestBuffers {
        pub input: Cursor<Vec<u8>>,
        pub output: Vec<u8>,
        pub error: Vec<u8>,
    }

    impl TestBuffers {
        pub fn new(input_buf: Option<Vec<u8>>) -> Self {
            let input_buf = input_buf.unwrap_or_default();

            Self {
                input: Cursor::new(input_buf),
                output: Vec::new(),
                error: Vec::new(),
            }
        }
    }

    pub fn build_args(args: &[&str]) -> Vec<String> {
        args.into_iter().map(|&s| s.to_owned()).collect()
    }

    pub fn create_file(name: &str, content: &str, path: &Path) -> PathBuf {
        let file = path.join(name);
        let _ = fs::write(file.as_path(), content);
        file
    }

    pub fn create_executable(name: &str, content: &str, path: &Path) -> PathBuf {
        let file = create_file(name, content, path);
        fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).unwrap();
        file
    }

    pub struct EnvPathGuard {
        original: Option<OsString>,
    }

    // SAFETY: it is the caller's responsibility to guarantee non-concurrent
    // modification of the `PATH` env var (e.g. via the #[serial] macro).
    impl EnvPathGuard {
        pub fn prepend(dir: &Path) -> Self {
            let original = std::env::var_os("PATH");
            let new_path = match &original {
                Some(p) => {
                    let mut paths = vec![dir.to_path_buf()];
                    paths.extend(std::env::split_paths(p));
                    std::env::join_paths(paths).unwrap()
                }
                None => dir.as_os_str().to_owned(),
            };
            unsafe { env::set_var("PATH", new_path) };
            Self { original }
        }
    }

    impl Drop for EnvPathGuard {
        fn drop(&mut self) {
            unsafe {
                match &self.original {
                    Some(p) => std::env::set_var("PATH", p),
                    None => std::env::remove_var("PATH"),
                }
            }
        }
    }
}
