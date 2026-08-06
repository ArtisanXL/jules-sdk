use std::fmt;

#[derive(Clone)]
pub struct HttpRequest {
    pub headers: Vec<(String, String)>,
}

impl fmt::Debug for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct RedactedHeaders<'a>(&'a [(String, String)]);

        impl<'a> fmt::Debug for RedactedHeaders<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut list = f.debug_list();
                for (k, v) in self.0 {
                    let k_lower = k.to_lowercase();
                    if k_lower == "authorization" || k_lower.contains("key") || k_lower.contains("token") || k_lower.contains("secret") {
                        list.entry(&(k, "***REDACTED***"));
                    } else {
                        list.entry(&(k, v));
                    }
                }
                list.finish()
            }
        }

        f.debug_struct("HttpRequest")
            .field("headers", &RedactedHeaders(&self.headers))
            .finish()
    }
}

fn main() {
    let req = HttpRequest {
        headers: vec![
            ("Authorization".to_string(), "Bearer secret".to_string()),
            ("Content-Type".to_string(), "application/json".to_string()),
            ("x-api-key".to_string(), "my-key".to_string()),
        ]
    };
    println!("{:?}", req);
}
