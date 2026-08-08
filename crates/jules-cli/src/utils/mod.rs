//! Output formatting helpers shared by all subcommands.

use clap::ValueEnum;
use serde::Serialize;

use crate::error::CliError;

/// The output format a subcommand renders its result in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable plain text output.
    Plain,
    /// Pretty-printed JSON output.
    Json,
}

/// A subcommand result that can be rendered as plain text or JSON.
pub trait Render: Serialize {
    /// Renders `self` as human-readable plain text.
    fn render_plain(&self) -> String;

    /// Renders `self` in the given [`OutputFormat`].
    ///
    /// # Errors
    ///
    /// Returns [`CliError::Json`] if JSON serialization fails.
    fn render(&self, format: OutputFormat) -> Result<String, CliError> {
        match format {
            OutputFormat::Plain => Ok(self.render_plain()),
            OutputFormat::Json => Ok(serde_json::to_string_pretty(self)?),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Example {
        name: String,
    }

    impl Render for Example {
        fn render_plain(&self) -> String {
            format!("name: {}", self.name)
        }
    }

    #[test]
    fn renders_plain() {
        let example = Example {
            name: "test".to_string(),
        };
        assert_eq!(example.render(OutputFormat::Plain).unwrap(), "name: test");
    }

    #[test]
    fn renders_json() {
        let example = Example {
            name: "test".to_string(),
        };
        let json = example.render(OutputFormat::Json).unwrap();
        assert!(json.contains("\"name\": \"test\""));
    }
}
