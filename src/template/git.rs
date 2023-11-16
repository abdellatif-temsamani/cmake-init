#[derive(Debug)]
pub struct Git {
    binary: String,
}

impl Git {
    /// Create a new Git struct from the given arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - The arguments to use.
    pub fn new() -> Git {
        // let binary = the output of `which git`
        Git {
            binary: String::from("git"),
        }
    }
}
