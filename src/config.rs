use std::path::PathBuf;

pub struct Config {
    pub path: PathBuf,
    pub show_in_list: bool,
    pub show_hidden_items: bool,
    pub show_in_reverse: bool,
    pub show_in_human_readable_form: bool,
    pub sort_by_size: bool,
    pub sort_by_time: bool,
    pub recursive_enabled: bool,
    pub show_version: bool,
    pub show_help: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let mut path = PathBuf::from(".");

        let mut show_hidden_items = false;
        let mut show_in_list = false;
        let mut show_in_reverse = false;
        let mut show_in_human_readable_form = false;
        let mut sort_by_size = false;
        let mut sort_by_time = false;
        let mut recursive_enabled = false;
        let mut show_version = false;
        let mut show_help = false;

        for arg in args {
            match arg.as_str() {
                "--version" => {
                    show_version = true;
                    break;
                }
                "-v" => {
                    show_version = true;
                    break;
                }
                "--help" => {
                    show_help = true;
                    break;
                }
                "-a" => show_hidden_items = true,
                "-l" => show_in_list = true,
                "-r" => show_in_reverse = true,
                "-h" => show_in_human_readable_form = true,
                "-S" => sort_by_size = true,
                "-t" => sort_by_time = true,
                "-R" => recursive_enabled = true,
                _ => path = PathBuf::from(arg),
            }
        }

        Ok(Config {
            path,
            show_in_list,
            show_hidden_items,
            show_in_reverse,
            show_in_human_readable_form,
            sort_by_size,
            sort_by_time,
            recursive_enabled,
            show_version,
            show_help,
        })
    }
}
