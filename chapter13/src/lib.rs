// 리팩토링하기
use std::error::Error;
use std::fs;

pub struct CommandConfig {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl CommandConfig {
    pub fn build( // args를 이터레이터 구현체로 변경
        mut args: impl Iterator<Item=String>
    ) -> Result<Self, &'static str> {
        args.next(); // 첫번째 인자는무시

        // ok_or을 이용하여 Some이 아니면 에러 반환
        let query = args.next().ok_or("there is no query")?;

        let file_path = args.next().ok_or("there is no file_path")?;
            // Some(file_path) => query,
            // None => return Err("there is no file_path")

        let ignore_case = Self::parse_ignore_case(args);

        Ok(CommandConfig {
            query,
            file_path,
            ignore_case,
        })
    }

    /**
     * IGNORE_CASE 포함 여부를 반환하는 메서드
     */
    fn parse_ignore_case(args: impl Iterator<Item=String>) -> bool {
      let key = String::from("IGNORE_CASE");

      std::env::var(&key).is_ok() || args.collect::<Vec<String>>().contains(&key)
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
    contents
    .lines()
    .filter(|it| it.contains(query))
    .collect()
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
    .lines()
    .filter(|it| it.to_lowercase().contains(&query))
    .collect()
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
