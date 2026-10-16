use breakfree_office::registry::Registry;
use std::fs;

#[test]
fn corrupt_settings_are_preserved_without_dispatch() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("settings.json");
    for raw in [
        "not json",
        "{}",
        "[]",
        r#"{"settings_version":2,"trusted_executables":{}}"#,
        r#"{"settings_version":1,"trusted_executables":{"unknown":"/tmp/x"}}"#,
        r#"{"settings_version":1,"trusted_executables":{"breakfree.docs":"relative"}}"#,
        r#"{"settings_version":1,"settings_version":1,"trusted_executables":{}}"#,
    ] {
        fs::write(&path, raw).unwrap();
        assert!(Registry::new(path.clone()).is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), raw);
    }
}

#[test]
fn old_settings_without_recent_field_remain_readable() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("settings.json");
    fs::write(&path, r#"{"settings_version":1,"trusted_executables":{}}"#).unwrap();
    assert!(Registry::new(path).unwrap().settings.recent.is_empty());
}
