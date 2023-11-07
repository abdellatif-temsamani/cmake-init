use std::{collections::HashMap, process::exit};
use struct_field_names_as_array::FieldNamesAsArray;

use crate::args::languages::Languages;

/// # Args struct
///
/// This struct is used to parse the arguments passed to the program.
///
/// ## Fields
///
/// * `name` - The name of the project
/// * `cmake_min_version` - The minimum version of CMake to use
/// * `lang` - The language chosen for the project
/// * `templates_dir` - The directory containing the template
#[derive(Debug, PartialEq, FieldNamesAsArray)]
pub struct Args {
    pub name: String,
    pub cmake_min_version: String,
    pub lang: Languages,
    pub templates_dir: String,
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
        let known_args = Args::FIELD_NAMES_AS_ARRAY;

        for (key, _) in &argv {
            if !known_args.contains(&key.as_str()) {
                eprintln!("Unknown argument: {}", key);
                exit(1);
            }
        }

        Args {
            name: Args::get_arg(&argv, "name", true, None),
            cmake_min_version: Args::get_arg(&argv, "cmake-version", false, Some("3.0")),
            lang: Languages::from_string(Args::get_arg(&argv, "lang", false, Some("c"))),
            templates_dir: Args::get_template(&argv),
        }
    }

    /// Get the template directory path.
    /// If the user has not specified a template directory, use the default.
    /// The default template directory is platform dependent.
    ///
    /// # Arguments
    ///
    /// * `argv` - The argument map.
    fn get_template(argv: &HashMap<String, Vec<String>>) -> String {
        #[cfg(target_os = "linux")]
        let templates_dir = Args::get_arg(
            &argv,
            "templates-dir",
            false,
            Some(
                format!(
                    "{}/.local/share/cmake-init/templates",
                    std::env::var("HOME").unwrap()
                )
                .as_str(),
            ),
        );

        #[cfg(target_os = "windows")]
        let templates_dir = Args::get_arg(
            &argv,
            "templates-dir",
            false,
            Some(
                format!(
                    "{}\\cmake-init\\templates",
                    std::env::var("APPDATA").unwrap()
                )
                .as_str(),
            ),
        );

        #[cfg(target_os = "macos")]
        let templates_dir = Args::get_arg(
            &argv,
            "templates-dir",
            false,
            Some(
                format!(
                    "{}/Library/Application Support/cmake-init/templates",
                    std::env::var("HOME").unwrap()
                )
                .as_str(),
            ),
        );

        templates_dir
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
            Args::print_help();
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
        println!("    --name <name>                The name of the project.");
        println!("    --cmake-version <version>    The minimum version of CMake to use.");
        println!("    --lang <version>             The language chosen for the project(cpp, c).");
        println!("    --templates-dir <dir>        The directory containing the templates.");
        println!("    --help | -h                  Print this help message.");
        println!("   --version | -v                Print the version of cmake-init.");
    }

    /// Print the version of cmake-init.
    /// This is called when the user passes the `--version` flag.
    pub fn print_version() {
        println!("cmake-init {}", env!("CARGO_PKG_VERSION"));
    }
}
