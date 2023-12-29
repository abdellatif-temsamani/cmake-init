use std::process::exit;

/// Enum for the languages that can be used.
/** NOTE: Currently only C and C++ are supported. */
#[derive(Debug, Clone)]
pub enum Language {
    C,
    Cpp,
}

impl Language {
    /// Create a new Languages enum from the given string.
    /// If the string is not a valid language, print an error and exit.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the language to create.
    pub fn from_string(name: String) -> Language {
        match &name as &str {
            "c" => Language::C,
            "cpp" => Language::Cpp,
            _ => {
                eprintln!("{} is not a valid language.", name);
                exit(1);
            }
        }
    }

    /// Get the file extension for the language.
    ///
    /// # Returns
    ///
    /// * A string representing the file extension for the language.
    fn file_extension(&self) -> String {
        match self {
            Language::C => String::from("c"),
            Language::Cpp => String::from("cpp"),
        }
    }

    /// Get the path to the main file path for the language.
    ///
    /// # Returns
    ///
    /// * A array of the main file path and the main file name.
    pub fn to_main(&self) -> String {
        format!("main.{}", self.file_extension())
    }
}

impl ToString for Language {
    /// Convert the language to a string.
    ///
    /// # Returns
    ///
    /// * A string representing the language.
    fn to_string(&self) -> String {
        match self {
            Language::C => String::from("c"),
            Language::Cpp => String::from("cpp"),
        }
    }
}
