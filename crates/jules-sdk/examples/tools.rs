use jules_sdk::jules_core::errors::ToolError;
use jules_sdk::jules_core::tool::{Tool, ToolParameter, ToolParameters, ToolRegistry};
use std::collections::HashMap;
use std::future::Future;

struct ReverseTextTool;

impl Tool for ReverseTextTool {
    fn name(&self) -> &'static str {
        "reverse_text"
    }

    fn description(&self) -> &'static str {
        "Reverses the given string."
    }

    fn parameters(&self) -> ToolParameters {
        let mut properties = HashMap::new();
        properties.insert(
            "text".to_string(),
            ToolParameter {
                param_type: "string".to_string(),
                description: Some("The text to reverse".to_string()),
            },
        );
        ToolParameters {
            properties,
            required: vec!["text".to_string()],
        }
    }

    fn call(&self, args: &str) -> impl Future<Output = Result<String, ToolError>> + Send {
        // In a real application, you'd parse args (which is JSON) and extract "text".
        // Here we just reverse the raw string for demonstration.
        let reversed: String = args.chars().rev().collect();
        async move { Ok(reversed) }
    }
}

fn main() {
    let mut registry = ToolRegistry::new();
    registry.register(ReverseTextTool);

    if let Some(tool) = registry.get("reverse_text") {
        println!("Found tool: {} - {}", tool.name(), tool.description());

        let f = async {
            match tool.call_dyn("hello").await {
                Ok(result) => println!("Result: {result}"),
                Err(e) => println!("Error: {e:?}"),
            }
        };

        // Poor man's block_on
        let waker = std::task::Waker::noop();
        let mut cx = std::task::Context::from_waker(waker);
        let mut future = std::boxed::Box::pin(f);
        let mut iters = 0;
        while std::future::Future::poll(future.as_mut(), &mut cx).is_pending() {
            iters += 1;
            assert!(iters <= 100, "future didn't resolve");
        }
    }
}
