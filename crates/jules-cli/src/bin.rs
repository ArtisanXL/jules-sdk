use jules_cli::commands::config::{ConfigArgs, ConfigCommand};

fn main() {
    let args = ConfigArgs {
        command: ConfigCommand::Set {
            api_key: Some("my_super_secret_api_key".to_string()),
            base_url: None,
        }
    };
    println!("{:?}", args);
}
