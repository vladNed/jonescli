/// Configuration struct for managing input arguments to console
pub struct Config {
    pub class_name: String
}
impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Class name query was not given")
        }

        let class_name = args[1].clone();

        Ok(Config {class_name})
    }
}