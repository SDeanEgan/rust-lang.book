use std::env; // includes environment functions like args or args_os
use std::process; // contains exit

use minigrep::Config;

/* Try: `IGNORE_CASE=1 cargo run -- to poem.txt` in terminal to control
 * for whether minigrep is checking for case.
 * To leave enabled for the session: `export IGNORE_CASE=1`.
 */

fn main() {
    // create args vector, using collect method to return iterator
    // collect needs type annotationlet args = env::args(); 
    let n = env::args().count(); // moves out env::args(), returns usize
    
    // call eng::args() anew. check if this is a help request
    // call to next_back() consumes an element
    // unwrap is guaranteed an element in this case
    let check_help = env::args().next_back().unwrap();
    
    if let "-h" | "--help" = check_help.as_str() { 
        minigrep::help();
        process::exit(0);
    }
    
    let config = Config::new(env::args(), n).unwrap_or_else(|err| {
        // unwrap_or_else is a Result method, this is its closure
        eprintln!("Problem parsing arguments: {err}");
        // exiting the command line tool with a nonzero error code 
        process::exit(1);
    });
    
    if config.verbose {
        minigrep::verbose(&config);
    }
    
    if let Err(e) = minigrep::run(config) {
        // print to std error and exit
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
