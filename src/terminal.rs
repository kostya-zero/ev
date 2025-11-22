pub fn print_error(msg: &str) {
    println!(" \x1b[91mError\x1b[0m: {msg}");
}

pub fn print_done(msg: &str) {
    println!(" \x1b[92m{}\x1b[0m {}", "✓", msg)
}
