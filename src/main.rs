use std::{env, process};

use minils::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if config.show_help {
        minils::print_help();
    } else if config.show_version {
        minils::print_version()
    } else {
        let contents = if config.recursive_enabled {
            minils::get_directory_contents_recursively(config.path.clone(), &config).unwrap_or_else(
                |err| {
                    eprintln!("{err:?}");
                    process::exit(1);
                },
            )
        } else {
            minils::get_directory_contents(&config).unwrap_or_else(|err| {
                eprintln!("{err:?}");
                process::exit(1);
            })
        };

        minils::print_contents(&config, &contents);
    }
}
