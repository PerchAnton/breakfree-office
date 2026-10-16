use breakfree_office::contract;
use serde_json::{Value, json};
use std::path::Path;

fn sample() -> Value {
    serde_json::from_str(include_str!("../contracts/v1/examples/docs.json")).unwrap()
}
fn check(value: &Value) -> breakfree_office::Result<contract::Manifest> {
    contract::validate(&serde_json::to_vec(value).unwrap(), None)
}

#[test]
fn all_examples_conform_and_no_office_format_is_invented() {
    for (source, id) in [
        (
            include_str!("../contracts/v1/examples/docs.json"),
            "breakfree.docs",
        ),
        (
            include_str!("../contracts/v1/examples/sheets.json"),
            "breakfree.sheets",
        ),
        (
            include_str!("../contracts/v1/examples/slides.json"),
            "breakfree.slides",
        ),
    ] {
        let manifest = contract::validate(source.as_bytes(), Some(id)).unwrap();
        assert!(!manifest.can_open(Path::new("example.docx")));
        assert!(manifest.supports("new"));
    }
}

#[test]
fn required_fields_types_values_and_extra_commands_are_rejected() {
    for field in [
        "protocol_version",
        "app_id",
        "app_version",
        "display_name",
        "capabilities",
        "formats",
    ] {
        let mut data = sample();
        data.as_object_mut().unwrap().remove(field);
        assert!(check(&data).is_err(), "missing {field}");
    }
    for (field, invalid) in [
        ("protocol_version", json!(true)),
        ("protocol_version", json!(1.0)),
        ("protocol_version", json!("1")),
        ("protocol_version", json!(2)),
        ("app_id", json!("unknown")),
        ("app_version", json!("")),
        ("display_name", json!(" ")),
        ("display_name", json!("line\nbreak")),
        ("capabilities", json!(["exec"])),
        ("capabilities", json!(["new", "new"])),
        ("capabilities", json!([])),
        ("formats", json!({})),
    ] {
        let mut data = sample();
        data[field] = invalid;
        assert!(check(&data).is_err(), "invalid {field}");
    }
    let mut data = sample();
    data["executable"] = json!("/tmp/arbitrary");
    assert!(check(&data).is_err());
}

#[test]
fn invalid_format_semantics_are_rejected() {
    for (field, value) in [
        ("extensions", json!([".bfdoc"])),
        ("extensions", json!(["BFDOC"])),
        ("extensions", json!(["bfdoc", "bfdoc"])),
        ("extensions", json!([])),
        ("read", json!("true")),
        ("id", json!("../exec")),
    ] {
        let mut data = sample();
        data["formats"][0][field] = value;
        assert!(check(&data).is_err());
    }
    let mut data = sample();
    data["capabilities"] = json!(["new"]);
    assert!(check(&data).is_err());
    data["formats"] = json!([]);
    assert!(check(&data).is_ok());
    let mut data = sample();
    let second = data["formats"][0].clone();
    data["formats"].as_array_mut().unwrap().push(second);
    assert!(check(&data).is_err());
    let mut data = sample();
    data["formats"][0]["read"] = json!(false);
    data["formats"][0]["write"] = json!(false);
    assert!(check(&data).is_err());
}

#[test]
fn strict_transport_rejects_duplicate_keys_invalid_unicode_and_trailing_data() {
    for input in [
        b"[]".as_slice(),
        b"null",
        b"",
        b"{bad}",
        b"\xff",
        br#"{"protocol_version":1,"protocol_version":1}"#,
        br#"{"protocol_version":NaN}"#,
        br#"{"protocol_version":1,"display_name":"\ud800"}"#,
    ] {
        assert!(contract::validate(input, None).is_err());
    }
    let mut repeated = serde_json::to_vec(&sample()).unwrap();
    repeated.extend(repeated.clone());
    assert!(contract::validate(&repeated, None).is_err());
    let deep = format!("{}{}", "[".repeat(1000), "]".repeat(1000));
    assert!(contract::validate(deep.as_bytes(), None).is_err());
    assert!(
        contract::validate(
            &serde_json::to_vec(&sample()).unwrap(),
            Some("breakfree.sheets")
        )
        .is_err()
    );
}

#[test]
fn file_extension_matching_is_case_insensitive_and_requires_read_support() {
    let mut data = sample();
    assert!(check(&data).unwrap().can_open(Path::new("План.BFDOC")));
    data["formats"][0]["read"] = json!(false);
    assert!(!check(&data).unwrap().can_open(Path::new("plan.bfdoc")));
}
