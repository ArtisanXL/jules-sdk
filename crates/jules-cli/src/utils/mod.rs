//! Output formatting shared by CLI subcommands.

use clap::ValueEnum;
use serde::{Serialize, Serializer};

/// The output format used to render subcommand results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable plain text output.
    #[default]
    Plain,
    /// Pretty-printed JSON output.
    Json,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plain => write!(f, "plain"),
            Self::Json => write!(f, "json"),
        }
    }
}

impl Serialize for OutputFormat {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

/// A subcommand result that can be rendered as plain text or JSON.
pub trait Render: Serialize {
    /// Renders `self` as a human-readable plain-text string.
    fn render_plain(&self) -> String;

    /// Renders `self` as pretty-printed JSON.
    ///
    /// # Errors
    ///
    /// Returns an error if `self` cannot be serialized to JSON.
    fn render_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Renders `self` using the given [`OutputFormat`].
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialization fails when `format` is
    /// [`OutputFormat::Json`].
    fn render(&self, format: OutputFormat) -> Result<String, serde_json::Error> {
        match format {
            OutputFormat::Plain => Ok(self.render_plain()),
            OutputFormat::Json => self.render_json(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Dummy {
        value: u32,
    }

    impl Render for Dummy {
        fn render_plain(&self) -> String {
            format!("value: {}", self.value)
        }
    }

    #[test]
    fn renders_plain_format() {
        let dummy = Dummy { value: 42 };
        assert_eq!(dummy.render(OutputFormat::Plain).unwrap(), "value: 42");
    }

    #[test]
    fn renders_json_format() {
        let dummy = Dummy { value: 42 };
        let json = dummy.render(OutputFormat::Json).unwrap();
        assert!(json.contains("\"value\": 42"));
    }

    #[test]
    fn output_format_value_enum_round_trips() {
        assert_eq!(
            OutputFormat::from_str("plain", true).unwrap(),
            OutputFormat::Plain
        );
        assert_eq!(
            OutputFormat::from_str("json", true).unwrap(),
            OutputFormat::Json
        );
    }

    #[test]
    fn output_format_display_matches_value_enum_names() {
        assert_eq!(OutputFormat::Plain.to_string(), "plain");
        assert_eq!(OutputFormat::Json.to_string(), "json");
    }
}
