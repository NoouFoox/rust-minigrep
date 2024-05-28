use std::{error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            // 参数错误
            return Err("Parameter error");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Self { query, file_path })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_txt = fs::read_to_string(config.file_path)?;
    println!("With text:\n{file_txt}");
    for line in search(&config.query, &file_txt) {
        println!("{line}")
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.trim())
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn on_result() {
        let query = "safe";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
