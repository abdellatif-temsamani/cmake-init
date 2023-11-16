use crate::arg::Args;
use std::{path::PathBuf, process::exit};

/// # Template struct
///
/// This struct is used to create the template files
/// and directories for the project.
///
/// ## Fields
///
/// * `args` - The arguments passed to the program
/// * `pwd` - The current working directory
pub struct Template {
    pub args: Args,
    pub pwd: PathBuf,
}

impl Template {
    /// Create a new Template struct from the given arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - The arguments passed to the program
    pub fn new(args: Args) -> Self {
        Self {
            args,
            pwd: std::env::current_dir().unwrap(),
        }
    }

    /// Create the template files and directories.
    /// If the src directory does not exist, create it.
    /// If the main file does not exist, create it.
    /// If the CMakeLists.txt file does not exist, create it.
    pub fn create(&self) {
        let main = self.args.lang.to_main();

        let template = self.get_template(main.clone());

        let src_dir = self.pwd.join("src");
        if !src_dir.exists() {
            std::fs::create_dir(src_dir).unwrap();
        }

        // create main file
        let main_file = self.pwd.join("src").join(main[1].clone());
        if !main_file.exists() {
            std::fs::write(main_file, template).unwrap();
        }

        // create CMakeLists.txt
        self.create_cmakelists();
    }

    /// Get the template file contents.
    /// If the template file does not exist, print an error and exit.
    fn get_template(&self, main: [String; 2]) -> String {
        // read template file
        let template_file = self
            .pwd
            .join(&self.args.templates_dir)
            .join(&main[0])
            .join(&main[1]);

        std::fs::read_to_string(template_file).unwrap_or_else(|_| {
            eprintln!("cannot read template file");
            exit(1);
        })
    }

    /// Create the CMakeLists.txt file.
    fn create_cmakelists(&self) {
        let cmakelists = self.pwd.join("CMakeLists.txt");

        std::fs::write(cmakelists, self.get_cmake_template().join("\n")).unwrap();
    }

    /// Get the CMakeLists.txt template.
    /// This is a three element array of strings.
    fn get_cmake_template(&self) -> [String; 3] {
        let project_name = self.args.name.to_lowercase().replace('-', "_");
        [
            format!("project({})", project_name),
            format!(
                "cmake_minimum_required(VERSION {})",
                self.args.cmake_min_version
            ),
            format!(
                "add_executable({} src/{})",
                project_name,
                self.args.lang.to_main()[1]
            ),
        ]
    }
}
