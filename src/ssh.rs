use std::process::Command;

pub fn connect(host: &str) {
    println!("🔗 Connecting to {}...", host);
    let _ = Command::new("ssh")
        .arg(host)
        .status()
        .expect("Failed to execute SSH");
}

