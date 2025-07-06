use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

pub fn parse_config() -> Vec<String> {
    let path = dirs::home_dir().unwrap().join(".ssh/config");
    let file = std::fs::File::open(path).expect("Cannot open config");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            if line.trim().starts_with("Host ") {
                Some(line.trim().replace("Host ", ""))
            } else {
                None
            }
        })
        .collect()
}

pub fn add_new_host() {
    use inquire::Text;

    let host = Text::new("Host:").prompt().unwrap();
    let hostname = Text::new("HostName:").prompt().unwrap();
    let user = Text::new("User:").prompt().unwrap();
    let identity = Text::new("IdentityFile [default: ~/.ssh/id_rsa]:")
        .prompt()
        .unwrap_or("~/.ssh/id_rsa".into());

    let config_entry = format!(
        "\nHost {}\n    HostName {}\n    User {}\n    IdentityFile {}\n",
        host, hostname, user, identity
    );

    let path = dirs::home_dir().unwrap().join(".ssh/config");
    let mut file = OpenOptions::new().append(true).open(path).unwrap();
    file.write_all(config_entry.as_bytes()).unwrap();

    println!("✅ Host '{}' added!", host);
}
