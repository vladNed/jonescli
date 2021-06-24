use std::path::PathBuf;
use std::env::current_dir;

/// Configuration struct for managing input arguments to console
pub struct Config {
    pub class_name: String,
    pub dir_path: PathBuf
}
impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Class name query was not given")
        }
        let class_name = args[1].clone();
        let dir_path = match current_dir() {
            Ok(dir) => dir,
            Err(_) => {
                return Err("Could not read current directory path")
            }
        };

        Ok(Config {class_name, dir_path})
    }
}