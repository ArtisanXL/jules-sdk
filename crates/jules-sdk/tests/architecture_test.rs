#[test]
fn test_architecture_documentation_exists() {
    let arch_docs = include_str!("../../../ARCHITECTURE.md");
    assert!(arch_docs.contains("Ownership Boundaries"));
}
