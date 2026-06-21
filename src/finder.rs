use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn find_executable(name: &str, paths: &[&Path]) -> Option<PathBuf> {
    if name.is_empty() {
        return None;
    }

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
    use crate::tests::utilities::{create_executable, create_file};

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn single_path_with_executable() {
        let name = "my_file";
        let dir = tempdir().unwrap();
        let file = create_executable(name, "", dir.path());

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
        create_file(name, "", dir.path());

        assert!(find_executable(name, &[&dir.path()]).is_none());
    }

    #[test]
    fn multiple_paths_with_one_executable() {
        let name = "my_file";

        let dirs: [tempfile::TempDir; 3] = std::array::from_fn(|_| tempdir().unwrap());

        let file = create_executable(name, "", dirs[1].path());

        let paths = dirs.iter().map(|dir| dir.path()).collect::<Vec<&Path>>();

        assert_eq!(find_executable(name, &paths), Some(file));
    }

    #[test]
    fn multiple_paths_with_multiple_executables() {
        let name = "my_file";

        let dirs: [tempfile::TempDir; 3] = std::array::from_fn(|_| tempdir().unwrap());

        let file1 = create_executable(name, "", dirs[1].path());
        create_executable(name, "", dirs[2].path());

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

        create_file(name, "", dirs[1].path());
        let file2 = create_executable(name, "", dirs[2].path());

        let paths = dirs.iter().map(|dir| dir.path()).collect::<Vec<&Path>>();

        assert_eq!(find_executable(name, &paths), Some(file2));
    }

    #[test]
    fn empty_name_does_not_match_when_path_entry_is_a_file() {
        let name = "bash";
        let dir = tempdir().unwrap();
        let file = create_executable(name, "", dir.path());

        // "bash" es un path-entry inválido (debería ser un dir), pero si existe
        // y es ejecutable, name="" no debe matchearlo
        assert!(find_executable("", &[&file]).is_none());
    }
}
