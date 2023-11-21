use super::git::Git;
use crate::arg::Args;
use std::fs;
use std::path::PathBuf;

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
    args: Args,
    pwd: PathBuf,
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
        let git = Git::new(self.args.git_path.clone());
        self.get_template();

        // create CMakeLists.txt
        self.create_cmakelists();

        git.init();
    }

    /// Get the template file contents.
    /// If the template file does not exist, print an error and exit.
    fn get_template(&self) {
        let dir = format!("{}/{}", self.args.templates_dir, self.args.lang.to_string());

        // git ignore file
        let file = fs::read(format!("{}/.gitignore", dir)).unwrap();
        let new_file = self.pwd.join(".gitignore");
        fs::write(new_file, file).unwrap();

        let src = self.pwd.join("src");
        // create src directory
        if !src.exists() {
            fs::create_dir(src.clone()).unwrap();
        }

        let files = fs::read_dir(format!("{}/src", dir)).unwrap();

        for file in files {
            let file = file.unwrap();
            let file_name = file.file_name();
            let file_name = file_name.to_str().unwrap();

            println!("Creating file: {}", file_name);
            let file = fs::read(format!("{}/src/{}", dir, file_name)).unwrap();
            let new_file = src.join(file_name);
            fs::write(new_file, file).unwrap();
        }
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
                self.args.cmake_version
            ),
            format!(
                "add_executable({} src/{})",
                project_name,
                self.args.lang.to_main()
            ),
        ]
    }
}
