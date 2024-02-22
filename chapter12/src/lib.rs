use std::error::Error;
use std::fs;

pub struct CommandConfig {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl CommandConfig {
    pub fn build(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("need more arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let additional_args = &args[2..];

        let ignore_case = Self::parse_ignore_case(additional_args);

        Ok(CommandConfig {
            query,
            file_path,
            ignore_case,
        })
    }

    /**
     * IGNORE_CASE 포함 여부를 반환하는 메서드
     */
    fn parse_ignore_case(args: &[String]) -> bool {
      let key = String::from("IGNORE_CASE");

      std::env::var(&key).is_ok() || args.contains(&key)
    }
}

pub fn run(config: CommandConfig) -> Result<(), Box<dyn Error>> {
    // let contents = match fs::read_to_string(&config.file_path) {
    //     Ok(content) => content,
    //     Err(e) => return Err(Box::new(e))
    // };

    let contents = fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    print_searched_lines(results);

    Ok(())
}

fn print_searched_lines(results: Vec<&str>) {
    println!("+-->\n");
    for (idx, result) in results.iter().enumerate() {
        println!("[{idx}]: {result}");
    }
    println!("\n+-->");
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for content in contents.lines() {
        if content.contains(query) {
            result.push(content);
        }
    }

    result
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for content in contents.lines() {
        if content.to_lowercase().contains(&query) {
            result.push(content);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work_with_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn it_work_with_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let result = search_insensitive(query, contents);
        assert_eq!(vec!["Rust:", "Trust me."], result);
    }
}
