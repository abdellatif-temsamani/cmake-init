use std::process::exit;

#[derive(Debug)]
pub struct Git {
    path: String,
}

impl Git {
    /// Create a new Git struct from the given arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - The arguments to use.
    pub fn new(path: String) -> Self {
        // let binary = the output of `which git`
        Git { path }
    }

    fn execute(&self, args: &[&str]) -> Result<String, String> {
        // let output = the output of `self.binary args`
        // if output.status.success() {
        //     Ok(output.stdout)
        // } else {
        //     Err(output.stderr)
        // }
        std::process::Command::new(&self.path)
            .args(args)
            .output()
            .map_err(|e| e.to_string())
            .and_then(|output| {
                if output.status.success() {
                    Ok(String::from_utf8(output.stdout).unwrap())
                } else {
                    Err(String::from_utf8(output.stderr).unwrap())
                }
            })
    }

    pub fn init(&self) {
        // create git repo
        match self.execute(&["init"]) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
        }
        match self.execute(&["add", "."]) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
        }
        match self.execute(&["commit", "-m", "\"Initial commit\""]) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
        }
    }
}
