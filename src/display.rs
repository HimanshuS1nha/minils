use crate::config::Config;
use crate::file::File;

pub fn print_version() {
    println!("Minils v1.0.0");
}

pub fn print_help() {
    println!("Usage: minils [options] [path]");
    println!("");
    println!("Options:");
    println!("-a -> Show hidden files");
    println!("-l -> Display in detailed list format");
    println!("-r -> Reverse sort order");
    println!("-S -> Sort by file size");
    println!("-t -> Sort by creation time");
    println!("-R -> Recursively list subdirectories");
    println!("-h -> Human-readable file sizes (only works with `-l`)");
    println!("-v -> Print version");
    println!("--version -> Print version");
    println!("--help -> Show usage guide");
}

pub fn print_contents(config: &Config, contents: &Vec<File>) {
    if config.show_in_list {
        let largest_filename_length = contents
            .iter()
            .map(|file| file.filename.len())
            .max()
            .unwrap()
            .max(4);

        let largest_size_length = contents
            .iter()
            .map(|file| file.size.to_string().len())
            .max()
            .unwrap()
            .max(4);

        println!(
            "{:<name_width$}  {:>7}  {:>size_width$}  {:>10}",
            "Name",
            "Type",
            "Size",
            "Permissions",
            name_width = largest_filename_length,
            size_width = largest_size_length + 6,
        );

        for file in contents {
            println!(
                "{:<name_width$}  {:>7}  {:>size_width$}  {:>10}",
                file.filename,
                file.file_type,
                if config.show_in_human_readable_form {
                    file.parse_size()
                } else {
                    file.size.to_string()
                },
                file.permissions,
                name_width = largest_filename_length,
                size_width = largest_size_length + 6,
            );
        }
    } else {
        for file in contents {
            if file.file_type == "Symlink" {
                print!("\x1b[42m{}\x1b[0m \t", file.filename);
            } else if file.file_type == "Dir" {
                print!("\x1b[32m{}\x1b[0m \t", file.filename);
            } else {
                print!("{} \t", file.filename);
            }
        }
        println!();
    }
}
