//
// File read
//
#[derive(Debug)]
pub struct Config {
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments, please specify the path to a binary file.");
        }

        let file_path = args[1].clone();

        Ok(Self { file_path })
    }

    pub fn get_fp(&self) -> &str {
        &self.file_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_test() {
        let args = vec!["0".to_string()];
        let config = Config::build(&args);
        assert_eq!(config.unwrap_err(), "not enough arguments, please specify the path to a binary file.");

        let args = vec!["0".to_string(), "helloworld.bin".to_string()];
        let config = Config::build(&args);
        assert_eq!(config.unwrap().file_path, "helloworld.bin");
    }

    #[test]
    fn get_fp_test() {
        let args = vec!["0".to_string(), "helloworld.bin".to_string()];
        let config = Config::build(&args);
        assert_eq!(config.unwrap().get_fp(), "helloworld.bin");
    }
}