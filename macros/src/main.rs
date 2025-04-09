macro_rules! debug_log {
    // matches a single log level and message
    ($level:expr, $message:expr) => {
        println!(
            "[{}] ({}) {}: {}",
            $level,
            module_path!(),
            line!(),
            $message
        )
    };
    
    // matches a log level, a format string, and variable arguments
    ($level:expr, $format:expr, $($arg:expr),*) => {
        println!(
            "[{}] ({}) {}: {}",
            $level,
            module_path!(),
            line!(),
            format!($format, $($arg),*)
        )
    };
}

fn main() {
    debug_log!("INFO", "Application start");
    
    let x = 42;
    let name = "Guy";
    debug_log!("DEBUG", "Value: {}, Name: {}", x, name);
    debug_log!("ERROR", 
               "Failed #{} - {}", 
               8675309, 
               "Invalid input");
}
