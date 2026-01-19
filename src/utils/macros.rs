
#[macro_export]
macro_rules! err {
    ($($arg:expr), *) => {{
        use colored::Colorize;  
        let formated = format!($($arg), *);

        eprintln!("[{}] {}", "*".red().bold(), formated);
    }};
}


#[macro_export]
macro_rules! info {
    ($($arg:expr), *) => {{
        use colored::Colorize;  
        let formated = format!($($arg), *);

        eprintln!("[{}] {}", "*".green().bold(), formated);
    }};
}