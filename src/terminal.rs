use colored::Colorize;

pub fn print_done(msg: &str) {
    println!(" {} {}", "✓".green(), msg)
}
