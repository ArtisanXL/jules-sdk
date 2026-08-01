#[test]
fn test_architecture_documentation_exists() {
    let arch_docs = include_str!("../../../ARCHITECTURE.md");
    assert!(arch_docs.contains("Ownership Boundaries"));
}

#[test]
fn test_inter_crate_dependency_graph() {
    // Enforce the dependency rules defined in ARCHITECTURE.md
    let core_toml = include_str!("../../jules-core/Cargo.toml");
    assert!(
        !core_toml.contains("jules-api"),
        "jules-core must not depend on jules-api"
    );
    assert!(
        !core_toml.contains("jules-sdk"),
        "jules-core must not depend on jules-sdk"
    );
    assert!(
        !core_toml.contains("jules-cli"),
        "jules-core must not depend on jules-cli"
    );
    assert!(
        !core_toml.contains("jules-macros"),
        "jules-core must not depend on jules-macros"
    );

    let api_toml = include_str!("../../jules-api/Cargo.toml");
    assert!(
        api_toml.contains("jules-core"),
        "jules-api must depend on jules-core"
    );
    assert!(
        !api_toml.contains("jules-sdk"),
        "jules-api must not depend on jules-sdk"
    );
    assert!(
        !api_toml.contains("jules-cli"),
        "jules-api must not depend on jules-cli"
    );
    assert!(
        !api_toml.contains("jules-macros"),
        "jules-api must not depend on jules-macros"
    );
}
