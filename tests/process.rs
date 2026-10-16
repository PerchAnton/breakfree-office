#![cfg(feature = "test-fixtures")]
use breakfree_office::{process, registry::Registry};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

fn fixture(dir: &Path, mode: &str) -> PathBuf {
    let name = if cfg!(windows) {
        format!("{mode} Пример.exe")
    } else {
        format!("{mode} Пример")
    };
    let file = dir.join(name);
    fs::copy(env!("CARGO_BIN_EXE_breakfree-test-editor"), &file).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&file, fs::Permissions::from_mode(0o700)).unwrap();
    }
    file
}

#[test]
fn real_rust_fixture_probe_and_version() {
    let dir = tempfile::tempdir().unwrap();
    let file = fixture(dir.path(), "docs");
    assert_eq!(
        process::probe(&file, "breakfree.docs").unwrap().app_version,
        env!("CARGO_PKG_VERSION")
    );
    let out = std::process::Command::new(file)
        .arg("--version")
        .output()
        .unwrap();
    assert_eq!(
        String::from_utf8(out.stdout).unwrap().trim(),
        env!("CARGO_PKG_VERSION")
    );
}

#[test]
fn absent_relative_and_directory_paths_are_rejected() {
    let dir = tempfile::tempdir().unwrap();
    for file in [
        dir.path().to_path_buf(),
        dir.path().join("absent"),
        PathBuf::from("relative"),
    ] {
        assert!(process::executable(&file).is_err());
    }
}

#[test]
fn bad_protocol_identity_json_exit_and_output_limits_are_handled() {
    let dir = tempfile::tempdir().unwrap();
    for mode in [
        "docs-protocol2",
        "sheets",
        "docs-malformed",
        "docs-probe-exit",
        "docs-stdout-limit",
        "docs-stderr-limit",
    ] {
        let file = fixture(dir.path(), mode);
        assert!(process::probe(&file, "breakfree.docs").is_err(), "{mode}");
    }
}

#[test]
fn hung_probe_is_bounded() {
    let dir = tempfile::tempdir().unwrap();
    let file = fixture(dir.path(), "docs-timeout");
    let start = Instant::now();
    assert!(
        process::probe_with_timeout(&file, "breakfree.docs", Duration::from_millis(350))
            .unwrap_err()
            .contains("timed out")
    );
    assert!(start.elapsed() < Duration::from_secs(2));
}

#[cfg(unix)]
#[test]
fn inherited_pipe_cannot_hold_probe_open() {
    let dir = tempfile::tempdir().unwrap();
    let file = fixture(dir.path(), "docs-holding-child");
    let start = Instant::now();
    assert!(
        process::probe_with_timeout(&file, "breakfree.docs", Duration::from_millis(350)).is_err()
    );
    assert!(start.elapsed() < Duration::from_secs(2));
}

#[test]
fn new_and_open_deliver_unicode_space_and_shell_punctuation_as_single_arguments() {
    let dir = tempfile::tempdir().unwrap();
    let file = fixture(dir.path(), "docs");
    let manifest = process::probe(&file, "breakfree.docs").unwrap();
    process::dispatch(&file, &manifest, "new", None).unwrap();
    let read_args = || {
        serde_json::from_slice::<Vec<String>>(&fs::read(file.with_extension("args.json")).unwrap())
            .unwrap()
    };
    assert_eq!(read_args(), ["--new"]);
    let document = dir
        .path()
        .join("План with spaces & 'quoted' $(echo marker).BFDOC");
    fs::write(&document, b"test-only").unwrap();
    process::dispatch(&file, &manifest, "open", Some(&document)).unwrap();
    assert_eq!(
        read_args(),
        [
            "--open".to_owned(),
            fs::canonicalize(document)
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        ]
    );
}

#[test]
fn unsupported_file_and_failed_dispatch_are_reported() {
    let dir = tempfile::tempdir().unwrap();
    let file = fixture(dir.path(), "docs-dispatch-exit");
    let manifest = process::probe(&file, "breakfree.docs").unwrap();
    assert!(process::dispatch(&file, &manifest, "new", None).is_err());
    let docx = dir.path().join("test.docx");
    fs::write(&docx, b"test-only").unwrap();
    assert!(process::dispatch(&file, &manifest, "open", Some(&docx)).is_err());
    assert!(process::dispatch(&file, &manifest, "convert", None).is_err());
}

#[test]
fn settings_persist_only_verified_paths_and_real_successful_recent_opens() {
    let dir = tempfile::tempdir().unwrap();
    let settings = dir.path().join("settings.json");
    let mut registry = Registry::new(settings.clone()).unwrap();
    assert!(registry.settings.recent.is_empty());
    assert!(registry.launch(0, "new", None).is_err());
    let file = fixture(dir.path(), "docs");
    registry.configure(0, &file).unwrap();
    let before = fs::read(&settings).unwrap();
    assert!(
        registry
            .configure(0, &fixture(dir.path(), "docs-protocol2"))
            .is_err()
    );
    assert_eq!(before, fs::read(&settings).unwrap());
    let document = dir.path().join("Реальный файл.bfdoc");
    fs::write(&document, b"fixture-only").unwrap();
    registry.launch(0, "open", Some(&document)).unwrap();
    registry.launch(0, "open", Some(&document)).unwrap();
    assert_eq!(registry.settings.recent.len(), 1);
    assert_eq!(Registry::new(settings).unwrap().settings.recent.len(), 1);
    registry
        .clear_recent(Some(&fs::canonicalize(&document).unwrap()))
        .unwrap();
    assert!(registry.settings.recent.is_empty());
    assert!(document.exists());
    registry.forget(0).unwrap();
    assert!(file.exists());
    assert!(registry.settings.trusted_executables.is_empty());
}

#[test]
fn changed_protocol_is_rechecked_before_launch_and_never_adds_recent() {
    let dir = tempfile::tempdir().unwrap();
    let settings = dir.path().join("settings.json");
    let mut registry = Registry::new(settings).unwrap();
    let file = fixture(dir.path(), "docs");
    registry.configure(0, &file).unwrap();
    // Replacing the binary with the incompatible fixture changes its filename-based response.
    let alternate = fixture(dir.path(), "docs-protocol2");
    registry
        .settings
        .trusted_executables
        .insert("breakfree.docs".into(), alternate);
    assert!(registry.launch(0, "new", None).is_err());
    assert!(registry.settings.recent.is_empty());
}
