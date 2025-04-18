pub mod config;
pub mod display;
pub mod file;

pub use config::Config;
pub use display::{print_contents, print_help, print_version};
pub use file::File;

use std::{cmp::Reverse, fs::DirEntry, os::windows::fs::MetadataExt, path::PathBuf};

fn add_file_entry(
    contents: &mut Vec<File>,
    config: &Config,
    file: DirEntry,
) -> Result<(), &'static str> {
    let filename = file.file_name().to_string_lossy().into_owned();
    let file_metadata = file.metadata().map_err(|_| "Unable to read file")?;

    let (file_type, file_permissions) = if file_metadata.is_symlink() {
        ("Symlink", "Symlink")
    } else {
        let permissions = if file_metadata.permissions().readonly() {
            "r"
        } else {
            "rw"
        };

        if file_metadata.is_file() {
            ("File", permissions)
        } else {
            ("Dir", permissions)
        }
    };

    if config.show_hidden_items || !filename.starts_with(".") {
        contents.push(File {
            filename,
            file_type,
            size: file_metadata.len(),
            permissions: file_permissions,
            creation_time: file_metadata.creation_time(),
        });
    }

    Ok(())
}

fn sort_contents(contents: &mut Vec<File>, config: &Config) {
    if config.sort_by_size {
        contents.sort_by_key(|file| Reverse(file.size));
    } else if config.sort_by_time {
        contents.sort_by_key(|file| Reverse(file.creation_time));
    } else {
        contents.sort();
    }

    if config.show_in_reverse {
        contents.reverse();
    }
}

pub fn get_directory_contents(config: &Config) -> Result<Vec<File>, &'static str> {
    let mut contents: Vec<File> = Vec::new();

    let items = config
        .path
        .read_dir()
        .map_err(|_| "Unable to read current working directory")?;

    for item in items {
        if let Ok(file) = item {
            add_file_entry(&mut contents, config, file)?;
        }
    }

    sort_contents(&mut contents, config);

    Ok(contents)
}

pub fn get_directory_contents_recursively(
    path: PathBuf,
    config: &Config,
) -> Result<Vec<File>, &'static str> {
    let mut contents: Vec<File> = Vec::new();

    let items = path.read_dir().map_err(|_| "Unable to read directory")?;

    for item in items {
        if let Ok(file) = item {
            let file_metadata = file.metadata().map_err(|_| "Unable to read file")?;

            if file_metadata.is_dir() {
                let files = get_directory_contents_recursively(file.path(), config)?;

                for file in files {
                    contents.push(file);
                }
            } else {
                add_file_entry(&mut contents, config, file)?;
            }
        }
    }

    sort_contents(&mut contents, config);

    Ok(contents)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_config_building() {
        let command = vec!["minils".to_string(), "-l".to_string(), "-a".to_string()];

        let args = command.into_iter();

        let config = Config::build(args).unwrap();

        assert!(config.show_hidden_items);
        assert!(config.show_in_list);
        assert_eq!(config.path, PathBuf::from("."));
    }

    #[test]
    pub fn test_parse_size_kb() {
        let file = File {
            creation_time: 0,
            file_type: "File",
            filename: String::from("test-file.txt"),
            permissions: "rw",
            size: 2048,
        };

        assert_eq!(file.parse_size(), "2 KB".to_string());
    }

    #[test]
    pub fn test_parse_size_gb() {
        let file = File {
            creation_time: 0,
            file_type: "File",
            filename: String::from("test-file.txt"),
            permissions: "rw",
            size: 2_147_483_648,
        };

        assert_eq!(file.parse_size(), "2 GB".to_string());
    }
}
