#![cfg(feature = "tools")]

use jules_core::errors::ToolError;
use jules_core::message::{Message, Role};
use jules_core::tool::{Tool, ToolCall, ToolCallInfo, ToolParameter, ToolParameters, ToolRegistry};
use std::collections::HashMap;
use std::future::Future;

struct WeatherTool;

impl Tool for WeatherTool {
    fn name(&self) -> &str {
        "get_weather"
    }

    fn description(&self) -> &str {
        "Gets the weather for a location."
    }

    fn parameters(&self) -> ToolParameters {
        let mut properties = HashMap::new();
        properties.insert(
            "location".to_string(),
            ToolParameter {
                param_type: "string".to_string(),
                description: Some("The city name".to_string()),
            },
        );
        ToolParameters {
            properties,
            required: vec!["location".to_string()],
        }
    }

    fn call(&self, args: &str) -> impl Future<Output = Result<String, ToolError>> + Send {
        let args = args.to_string();
        async move {
            if args.contains("London") {
                Ok("Rainy".to_string())
            } else {
                Ok("Sunny".to_string())
            }
        }
    }
}

#[tokio::test]
async fn test_tool_workflow() {
    let mut registry = ToolRegistry::new();
    registry.register(WeatherTool);

    let tool_call = ToolCall {
        function: ToolCallInfo {
            id: "call_123".to_string(),
            name: "get_weather".to_string(),
            arguments: r#"{"location": "London"}"#.to_string(),
        },
        tool_type: "function".to_string(),
    };

    let msg = Message::new(Role::Assistant, "").with_tool_calls(vec![tool_call.clone()]);

    assert!(msg.tool_calls().is_some());
    assert_eq!(msg.tool_calls().unwrap().len(), 1);

    let called_tool = registry
        .get(&tool_call.function.name)
        .expect("tool not found");
    let result = called_tool
        .call_dyn(&tool_call.function.arguments)
        .await
        .unwrap();

    assert_eq!(result, "Rainy");

    let tool_msg = Message::new(Role::Tool, result).with_tool_call_id("call_123");

    assert_eq!(tool_msg.role(), &Role::Tool);
    assert_eq!(tool_msg.tool_call_id(), Some("call_123"));
}
