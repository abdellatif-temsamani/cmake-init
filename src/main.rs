mod args;

use argmap;

use crate::args::Args;

fn main() {
    let (_, argv) = argmap::parse(std::env::args());

    let args = Args::from(argv);

    let pwd = std::env::current_dir().unwrap();

    // create src directory if it doesn't exist
    let src_dir = pwd.join("src");
    if !src_dir.exists() {
        std::fs::create_dir(src_dir).unwrap();
    }

    // read template file
    let template_file = pwd.join(args.templates_dir).join(args.lang.to_string());
    let template = std::fs::read_to_string(template_file).unwrap();

    // create main file
    let main_file = pwd.join("src").join(args.lang.to_string());
    if !main_file.exists() {
        std::fs::write(main_file, template).unwrap();
    }
}
