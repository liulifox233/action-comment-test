use std::process::exit;

fn main() {
    println!("test ok");
    eprintln!("test error");
    exit(1);
}
