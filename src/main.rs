use std::env;

fn main() {
    println!("Операционная система: {} {}", env::consts::OS, env::consts::ARCH);
}
