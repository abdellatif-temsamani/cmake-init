use std::process::exit;

/// Enum for the languages that can be used.
/** NOTE: Currently only C and C++ are supported. */
#[derive(Debug, Clone)]
pub enum Languages {
    C,
    Cpp,
}

impl Languages {
    /// Create a new Languages enum from the given string.
    /// If the string is not a valid language, print an error and exit.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the language to create.
    pub fn from_string(name: String) -> Languages {
        match &name as &str {
            "c" => Languages::C,
            "cpp" => Languages::Cpp,
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
            Languages::C => String::from("c"),
            Languages::Cpp => String::from("cpp"),
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

impl ToString for Languages {
    /// Convert the language to a string.
    ///
    /// # Returns
    ///
    /// * A string representing the language.
    fn to_string(&self) -> String {
        match self {
            Languages::C => String::from("c"),
            Languages::Cpp => String::from("cpp"),
        }
    }
}
