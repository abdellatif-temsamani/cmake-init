use std::{collections::HashMap, process::exit};

#[derive(Debug)]
pub struct Args {
    pub name: String,
    pub cmake_min_version: String,
    pub lang: String,
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

        Args {
            name: Args::get_arg(&argv, "name", true, None),
            cmake_min_version: Args::get_arg(&argv, "cmake-version", false, Some("3.0")),
            lang: Args::get_arg(&argv, "lang", false, Some("")),
        }
    }

    /// Get the value of the argument at the given index.
    /// If the argument is not found, print an error and exit.
    ///
    /// # Arguments
    ///
    /// * `argv` - The argument map.
    /// * `index` - The index of the argument to get.
    fn get_arg(
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
        } else if argv.get("help").is_some() {
            Args::print_help();
        } else if argv.get("version").is_some() {
            Args::print_version();
        }
        exit(0);
    }

    /// Print the help message.
    /// This is called when the user passes the `--help` flag.
    fn print_help() {
        println!("Usage: cmake-init --name=<name>");
        println!();
        println!("Options:");
        println!("    --name=<name>                The name of the project.");
        println!("    --cmake-version=<version>    The minimum version of CMake to use.");
        println!("    --lang=<version>             the language chosen for the project(cpp, c).");
        println!("    --help                       Print this help message.");
        println!("   --version                    Print the version of cmake-init.");
    }

    /// Print the version of cmake-init.
    /// This is called when the user passes the `--version` flag.
    pub fn print_version() {
        println!("cmake-init {}", env!("CARGO_PKG_VERSION"));
    }
}
