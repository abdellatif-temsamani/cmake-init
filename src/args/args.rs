use std::{collections::HashMap, process::exit};

use super::Languages;

/// A struct to hold the arguments passed to cmake-init.
/// The arguments are parsed from the command line and stored in this struct.
/// The struct is then passed to the `init` function to create the project.
#[derive(Debug)]
pub struct Args {
    pub name: String,
    pub cmake_min_version: String,
    pub lang: Languages,
}

impl Args {
    /// Create a new Args struct from the given argument map.
    /// If any of the required arguments are missing, print an error and exit.
    ///
    /// # Arguments
    ///
    /// * `argv` - The argument map.
    pub fn from(argv: HashMap<String, Vec<String>>) -> Args {
        Args::validate_args(&argv);

        #[allow(unused_doc_comments)]
        /**
         * HACK: This is a hack to check unknown arguments.
         */
        for (key, _) in &argv {
            if key != "name" && key != "cmake-version" && key != "lang" {
                eprintln!("Unknown argument: {}", key);
                exit(1);
            }
        }

        Args {
            name: Args::get_arg(&argv, "name", true, None),
            cmake_min_version: Args::get_arg(&argv, "cmake-version", false, Some("3.0")),
            lang: Languages::from_string(Args::get_arg(&argv, "lang", false, Some("c"))),
        }
    }

    /// Get the value of the argument at the given index.
    /// If the argument is not found, print an error and exit.
    ///
    /// # Arguments
    ///
    /// * `argv` - The argument map.
    /// * `index` - The index of the argument to get.
    pub fn get_arg(
        argv: &HashMap<String, Vec<String>>,
        index: &str,
        required: bool,
        default: Option<&str>,
    ) -> String {
        let default = &default.unwrap_or("").to_string();

        match argv.get(index).unwrap_or(&vec![]).get(0) {
            Some(expr) => expr.to_owned(),
            None => {
                if required {
                    eprintln!("{} is required.", index);
                    exit(1);
                } else {
                    default.to_string()
                }
            }
        }
    }

    /// Check the health of the arguments.
    /// If there are no arguments, print an error and exit.
    ///
    /// # Arguments
    ///
    /// * `argv` - The argument map.
    fn validate_args(argv: &HashMap<String, Vec<String>>) {
        if argv.len() == 0 {
            eprintln!("No arguments provided.");
            exit(1);
        } else if argv.get("help").is_some() || argv.get("h").is_some() {
            Args::print_help();
            exit(0);
        } else if argv.get("version").is_some() || argv.get("v").is_some() {
            Args::print_version();
            exit(0);
        }
    }

    /// Print the help message.
    /// This is called when the user passes the `--help` flag.
    fn print_help() {
        println!("Usage: cmake-init --name=<name>");
        println!();
        println!("Options:");
        println!("    --name=<name>                The name of the project.");
        println!("    --cmake-version=<version>    The minimum version of CMake to use.");
        println!("    --lang=<version>             The language chosen for the project(cpp, c).");
        println!("    --help | -h                  Print this help message.");
        println!("   --version | -v                Print the version of cmake-init.");
    }

    /// Print the version of cmake-init.
    /// This is called when the user passes the `--version` flag.
    pub fn print_version() {
        println!("cmake-init {}", env!("CARGO_PKG_VERSION"));
    }
}

/// tests
#[cfg(test)]
mod arg_tests {
    use super::Languages;
    use std::collections::HashMap;
    #[test]
    fn test_args() {
        let mut args = HashMap::new();
        args.insert("name".to_string(), vec!["test".to_string()]);
        args.insert("cmake-version".to_string(), vec!["3.0".to_string()]);
        args.insert("lang".to_string(), vec!["cpp".to_string()]);
        let args = super::Args::from(args);
        assert_eq!(args.name, "test");
        assert_eq!(args.cmake_min_version, "3.0");
        assert_eq!(args.lang, Languages::CPP);
    }

    #[test]
    fn test_args_default() {
        let mut args = HashMap::new();
        args.insert("name".to_string(), vec!["test".to_string()]);
        let args = super::Args::from(args);
        assert_eq!(args.name, "test");
        assert_eq!(args.cmake_min_version, "3.0");
        assert_eq!(args.lang, Languages::CPP);
    }

    #[test]
    #[should_panic]
    fn test_args_panic() {
        let args = HashMap::new();
        let _args = super::Args::from(args);
    }

    #[test]
    fn test_args_help() {
        let mut args = HashMap::new();
        args.insert("help".to_string(), vec!["".to_string()]);
        let args = super::Args::from(args);
        assert_eq!(args.name, "");
        assert_eq!(args.cmake_min_version, "3.0");
        assert_eq!(args.lang, Languages::CPP);
    }
}
