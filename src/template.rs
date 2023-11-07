use crate::args::Args;
use std::path::PathBuf;

pub struct Template {
    pub args: Args,
    pub pwd: PathBuf,
}

impl Template {
    pub fn new(args: Args) -> Self {
        Self {
            args,
            pwd: std::env::current_dir().unwrap(),
        }
    }

    pub fn create(&self) {
        // create src directory if it doesn't exist
        let template = self.get_template();

        let src_dir = self.pwd.join("src");
        if !src_dir.exists() {
            std::fs::create_dir(src_dir).unwrap();
        }

        // create main file
        let main_file = self.pwd.join("src").join(self.args.lang.to_string());
        if !main_file.exists() {
            std::fs::write(main_file, template).unwrap();
        }
    }

    fn get_template(&self) -> String {
        // read template file
        let template_file = self
            .pwd
            .join(&self.args.templates_dir)
            .join(self.args.lang.to_string());

        std::fs::read_to_string(template_file).unwrap()
    }
}
