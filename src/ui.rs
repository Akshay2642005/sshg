use crate::config;
use dialoguer::Select;

pub fn start() {
    let hosts = config::parse_config();
    let selection = Select::new()
        .with_prompt("Select a host to connect")
        .items(&hosts)
        .interact()
        .unwrap();

    let host = &hosts[selection];
    crate::ssh::connect(host);
}