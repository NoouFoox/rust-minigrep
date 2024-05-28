use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            // 参数错误
            return Err("Parameter error");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE ").is_ok();
        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_txt = fs::read_to_string(config.file_path)?;
    let reuslt = if config.ignore_case {
        case_search(&config.query, &file_txt)
    } else {
        search(&config.query, &file_txt)
    };
    for line in reuslt {
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

pub fn case_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
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
    #[test]
    fn case_search_test() {
        let query = "saFe";
        let contents = "\
      Rust:
      safe, fast, productive.
      Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            case_search(query, contents)
        );
    }
}
