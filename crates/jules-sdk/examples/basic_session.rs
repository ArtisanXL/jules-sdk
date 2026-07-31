use jules_sdk::Session;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::builder().name("My First Session").build()?;

    println!("Successfully created session: {:?}", session.name());

    Ok(())
}
