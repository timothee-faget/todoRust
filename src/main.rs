fn main() {
    if let Err(e) = todorust::run() {
        eprint!("{e}")
    };
}
