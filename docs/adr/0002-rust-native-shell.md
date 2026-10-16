# ADR 0002: mandatory Rust and a native Rust GUI

- Date: 2026-10-16
- Status: Rust mandated by owner; egui/eframe selected for this review prototype
- Supersedes: the reversible Python/Tk experiment on local `codex/office-launcher-v1`

## Decision

All new core, application logic, executable integration and GUI are Rust. Rust
is no longer a candidate language. Cargo and `rust-toolchain.toml` pin Rust
1.90.0; Cargo.lock pins the resolved dependency graph. No JS/TS frontend,
Electron, WebView, HTML/CSS interface or application server is used. OS APIs,
resources, configuration and shaders are permitted system integration.

The old Python experiment remains in Git history on its separate branch. No
Python source/runtime is needed by this implementation. Its test report remains
historical evidence and is not proof that the Rust port passes the same checks.

## GUI choice

Use native eframe/egui 0.32.3 with the Glow renderer and AccessKit enabled. The UI
is authored in Rust, including navigation, menus, settings, error states and
dialogs. Standard toolkit demo styling is replaced by visual-system v1. Native
file dialogs use rfd; no browser UI is introduced.

| Candidate | Assessment against this prototype |
| --- | --- |
| egui/eframe | Rust UI code, custom density/layout, keyboard focus, DPI-aware logical coordinates, AccessKit integration and a headless accessibility-tree test harness. Selected; native screen-reader acceptance still needs measurement |
| Slint | Strong custom visual and accessibility facilities; introduces a separate UI description language and runtime/licensing choices. Not selected for the owner's Rust-authored GUI requirement |
| GTK/Windows-only widgets | Useful native accessibility, but additional platform/style constraints for closely matching one desktop office layout across candidate platforms |

The toolkit is replaceable if native accessibility or visual acceptance fails;
the Rust language requirement and executable integration protocol are not.
AccessKit enablement alone is not a screen-reader conformance claim.

## Separation and behavior

`contract.rs` validates strict JSON, the existing v1 schema and semantic rules.
`process.rs` handles direct argument vectors, timeout/output limits and process
lifetime. `registry.rs` owns trusted selections and atomic local settings.
`gui.rs` draws the shell and receives worker results; probes run off the UI thread.
`visual.rs` implements versioned shell tokens. Office never edits document content.

Settings accept existing v1 selections and add optional real recent-file records.
Only successfully dispatched opens enter the recent list; this does not prove
that the editor loaded or saved the document. Clearing history never deletes files.
Failed launches add no fake recent entry. Protocol v1 has no template discovery:
the template area truthfully shows an empty state.

Each editor retains its own repository, build, tests, version and executable,
working without Office or the others. Contract/visual specifications may be
consumed at a reviewed immutable commit; no sibling-directory or mutable-main
build dependency is permitted. No shared library or fifth project is created.

## Verification limits

Windows 100/125/150% screenshots, real Office 2024 Start/Backstage measurements,
NVDA/Narrator and Windows native dialogs require a Windows environment. Logical
pixel-ratio tests on macOS cannot substitute. The reference inventory and
validation report record exactly what is available. No licensed Microsoft font
or resource is redistributed. The project license remains the owner's decision.

Primary API references: [eframe 0.32.3](https://docs.rs/eframe/0.32.3/eframe/),
[egui_kittest 0.32.3](https://docs.rs/egui_kittest/0.32.3/egui_kittest/),
[AccessKit](https://accesskit.dev/), and
[Rust 1.90.0](https://blog.rust-lang.org/2025/09/18/Rust-1.90.0/).
