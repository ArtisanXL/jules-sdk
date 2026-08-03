#[derive(Clone)]
pub struct BuiltClient {
    base_url: String,
    auth_token: String,
}

impl std::fmt::Debug for BuiltClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuiltClient")
            .field("base_url", &self.base_url)
            .field("auth_token", &"***REDACTED***")
            .finish()
    }
}

fn main() {
    let client = BuiltClient {
        base_url: "url".into(),
        auth_token: "secret".into(),
    };
    println!("{:?}", client);
}
