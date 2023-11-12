mod arg;
mod template;

use crate::arg::Args;
use crate::template::Template;

fn main() {
    let (_, argv) = argmap::parse(std::env::args());

    let args = Args::from(argv);

    let template = Template::new(args);
    template.create();
}
