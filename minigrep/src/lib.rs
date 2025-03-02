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
    pub fn new(
        mut args: impl Iterator<Item = String> + DoubleEndedIterator,
        n: usize,
    ) -> Result<Config, &'static str> {
		/* new now uses iterators to construct the config from args.
		 * included ` + DoubleEndedIterator` to allow for `next_back`.
		 */
		let query: String;
		let file_path: String;
		let mut ignore_case: bool;
		let mut verbose: bool;
		let mut count: bool;
		
		if n < 3 {
            // error values will be string literals with 'static lifetime
            return Err("not enough arguments provided");
        }
		
		// env::var returns a Result, is_ok returns bool 
        ignore_case = env::var("IGNORE_CASE").is_ok();
        verbose = env::var("VERBOSE").is_ok();
        count = env::var("COUNT").is_ok();
		
		file_path = match args.next_back() { // consume from the end
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        
        query = match args.next_back() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        
        args.next();
		
		while let Some(arg) = args.next() {
			match arg.as_str() {
				"-i" | "--ignore-case" => ignore_case = true,
				"-v" | "--verbose" => verbose = true,
				"-c" | "--count" => count = true,
				_ => return Err("unrecognized argument"),
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
	 contents.lines() // lines makes an iterator of contents
			 .filter(|line| line.contains(query))
			 .collect()
}

pub fn search_case_insensitive<'a>(
	query: &str, contents: &'a str) -> Vec<&'a str> {
	
	let query: String = query.to_lowercase();	
	contents.lines()
			.filter(|line| line.to_lowercase().contains(&query))
			.collect()
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
