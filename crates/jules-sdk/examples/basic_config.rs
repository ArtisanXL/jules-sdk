use jules_sdk::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder().api_key("test_key").timeout(30).build()?;

    println!(
        "Successfully created config with API key: {} and timeout: {:?}",
        config.api_key(),
        config.timeout()
    );

    Ok(())
}
