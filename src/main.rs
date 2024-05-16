use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut os = HashMap::new();
    os.insert("linux", "Линукс");

    let operating_system = os.get(env::consts::OS);

    println!(
        "Операционная система: {} {}",
        operating_system.expect("неизвестно операционная система"),
        env::consts::ARCH
    );

    Ok(())
}
