use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn find_executable(name: &str, paths: &[&Path]) -> Option<PathBuf> {
    for path in paths {
        let file_path = path.join(name);

        if let Ok(b) = is_executable(&file_path)
            && b
        {
            return Some(file_path);
        }
    }

    None
}

pub fn find_executable_in_path(name: &str) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;
    let paths = env::split_paths(&path_var).collect::<Vec<_>>();
    let paths = paths.iter().map(|p| p.as_path()).collect::<Vec<_>>();

    find_executable(name, &paths)
}

fn is_executable(path: &Path) -> std::io::Result<bool> {
    let metadata = fs::metadata(path)?;

    Ok(metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
}

#[cfg(test)]
mod tests {
    use std::{fs, os::unix::fs::PermissionsExt};

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn single_path_with_executable() {
        let name = "my_file";

        let dir = tempdir().unwrap();
        let file = dir.path().join(name);
        let _ = fs::write(file.as_path(), "");
        fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).unwrap();

        assert_eq!(find_executable(name, &[&dir.path()]), Some(file));
    }

    #[test]
    fn single_path_without_file() {
        let name = "my_file";

        let dir = tempdir().unwrap();

        assert!(find_executable(name, &[&dir.path()]).is_none());
    }

    #[test]
    fn single_path_with_file_no_executable() {
        let name = "my_file";

        let dir = tempdir().unwrap();
        let file = dir.path().join(name);
        let _ = fs::write(file.as_path(), "");

        assert!(find_executable(name, &[&dir.path()]).is_none());
    }

    #[test]
    fn multiple_paths_with_one_executable() {
        let name = "my_file";

        let dirs: [tempfile::TempDir; 3] = std::array::from_fn(|_| tempdir().unwrap());

        let file = dirs[1].path().join(name);
        let _ = fs::write(file.as_path(), "");
        fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).unwrap();

        let paths = dirs.iter().map(|dir| dir.path()).collect::<Vec<&Path>>();

        assert_eq!(find_executable(name, &paths), Some(file));
    }

    #[test]
    fn multiple_paths_with_multiple_executables() {
        let name = "my_file";

        let dirs: [tempfile::TempDir; 3] = std::array::from_fn(|_| tempdir().unwrap());

        let file1 = dirs[1].path().join(name);
        let _ = fs::write(file1.as_path(), "");
        fs::set_permissions(&file1, fs::Permissions::from_mode(0o755)).unwrap();

        let file2 = dirs[2].path().join(name);
        let _ = fs::write(file2.as_path(), "");
        fs::set_permissions(&file2, fs::Permissions::from_mode(0o755)).unwrap();

        let paths = dirs.iter().map(|dir| dir.path()).collect::<Vec<&Path>>();

        assert_eq!(find_executable(name, &paths), Some(file1));
    }

    #[test]
    fn multiple_paths_without_file() {
        let name = "my_file";

        let dirs: [tempfile::TempDir; 3] = std::array::from_fn(|_| tempdir().unwrap());
        let paths = dirs.iter().map(|dir| dir.path()).collect::<Vec<&Path>>();

        assert!(find_executable(name, &paths).is_none());
    }

    #[test]
    fn multiple_paths_with_one_executable_and_one_non_executable() {
        let name = "my_file";

        let dirs: [tempfile::TempDir; 3] = std::array::from_fn(|_| tempdir().unwrap());

        let file1 = dirs[1].path().join(name);
        let _ = fs::write(file1.as_path(), "");

        let file2 = dirs[2].path().join(name);
        let _ = fs::write(file2.as_path(), "");
        fs::set_permissions(&file2, fs::Permissions::from_mode(0o755)).unwrap();

        let paths = dirs.iter().map(|dir| dir.path()).collect::<Vec<&Path>>();

        assert_eq!(find_executable(name, &paths), Some(file2));
    }
}
