mod args;

use args::Args;

use argmap;

fn main() {
    let (_, argv) = argmap::parse(std::env::args());

    let args = Args::from(argv);
    println!("{:?}", args);
}
