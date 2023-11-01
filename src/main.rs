mod args;

use argmap;

use crate::args::Args;

fn main() {
    let (_, argv) = argmap::parse(std::env::args());

    let args = Args::from(argv);
    println!("{:?}", args);
}
