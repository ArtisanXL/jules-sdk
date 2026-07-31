use std::fs;
use std::path::PathBuf;

#[test]
fn test_workspace_members() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_toml_path = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("Cargo.toml");
    let content =
        fs::read_to_string(workspace_toml_path).expect("Failed to read workspace Cargo.toml");

    assert!(content.contains("\"crates/jules-sdk\""));
    assert!(content.contains("\"crates/jules-core\""));
    assert!(content.contains("\"crates/jules-api\""));
    assert!(content.contains("\"crates/jules-macros\""));
    assert!(content.contains("\"crates/jules-cli\""));
}
