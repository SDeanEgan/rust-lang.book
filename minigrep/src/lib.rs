use std::fs; // filesystem manipulation
use std::error::Error;
use std::env; // for environment variables

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub verbose: bool,
    pub count: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
		let query;
		let file_path;
		let mut ignore_case;
		let mut verbose;
		let mut count;
		let n = args.len();
		
		// env::var returns a Result, is_ok returns bool 
        ignore_case = env::var("IGNORE_CASE").is_ok();
        verbose = env::var("VERBOSE").is_ok();
        count = env::var("COUNT").is_ok();
		
        if n < 3 {
            // error values will be string literals with 'static lifetime
            return Err("not enough arguments provided");
        } else if n == 3 {
			query = args[1].clone();
			file_path = args[2].clone();
		} else {
			query = args[n-2].clone();
			file_path = args[n-1].clone();
			for arg in &args[1..(n-2)] {
				match arg.as_str() {
					"-i" | "--ignore-case" => ignore_case = true,
					"-v" | "--verbose" => verbose = true,
					"-c" | "--count" => count = true,
					_ => return Err("unrecognized argument"),
				}
			}
		}
        
        Ok(Config { query, file_path, ignore_case, verbose, count })
    }
}

#[cfg(test)]
mod tests{
	use super::*;
	
	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}
	
	
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn is_count() {
		// invent results
        let results: Vec<&str> = vec!["I'm a result!"];
		
		// assert result
		assert_eq!("1", count(&results));
    }
}

pub fn help() {
	println!("\
Usage: minigrep [OPTION]... PATTERN [FILEPATH]
Search for PATTERN in a FILE.
Example: minigrep -i 'hello world' hello.txt

Pattern selection and interpretation:
  -i, --ignore-case         ignore case distinctions in patterns and data

Miscellaneous:
  -h, --help                display this help text and exit

Output control:
  -v, --verbose             include additional output
  -c, --count               print only a count of selected lines per FILE
");
}

pub fn verbose(config: &Config) {
	println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	/* we include lifetime 'a to direct the borrow checker to understand
	 * that the lifetime of data from our document's contents will need 
	 * to live on with our returned vector
	 */
	 let mut results = Vec::new();
	 for line in contents.lines() { // lines returns an iterator
		 if line.contains(query) {
			 results.push(line);
		 }
	 }
	 
	 results	
}

pub fn search_case_insensitive<'a>(
	query: &str, contents: &'a str) -> Vec<&'a str> {
	
	let query: String = query.to_lowercase();
	let mut results = Vec::new();
	
	for line in contents.lines() {
		if line.to_lowercase().contains(&query) { // note: &query
			results.push(line);
		}
	}
	
	results
}

pub fn count(results: & Vec<&str>) -> String {
	// return vector containing string of length
	format!("{}", results.len())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? to either unwrap or propogate an error by returning early
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
		search_case_insensitive(&config.query, &contents)
	} else {
		search(&config.query, &contents)
	};
	
	if let true = config.count {
		println!("{}", count(&results));
	} else {
		for line in results {
			println!("{line}");
		}
	}
    
    Ok(())
}
