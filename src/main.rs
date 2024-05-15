use std::env;

fn main() {
    println!("Операционная система: {}", env::consts::OS);
    println!("Центральный процессор архитектура: {}", env::consts::ARCH);
}
