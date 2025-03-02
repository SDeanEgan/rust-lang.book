use std::env; // includes environment functions like args or args_os
use std::process; // contains exit

use minigrep::Config;

/* Try: `IGNORE_CASE=1 cargo run -- to poem.txt` in terminal to control
 * for whether minigrep is checking for case.
 * To leave enabled for the session: `export IGNORE_CASE=1`.
 */

fn main() {
    // create args vector, using collect method to return iterator
    // collect needs type annotation
    let args: Vec<String> = env::args().collect(); 
    let n = args.len();
    //dbg!(args); // debug macro
    
    if let "-h" | "--help" = args[n-1].as_str() { 
        minigrep::help();
        process::exit(0);
    }
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        // unwrap_or_else is a Result method, this is its closure
        eprintln!("Problem parsing arguments: {err}");
        // exiting the command line tool with a nonzero error code 
        process::exit(1);
    });
    
    if config.verbose {
        minigrep::verbose(&config);
    }
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}"); // prints to standard error
        process::exit(1);
    }
}
