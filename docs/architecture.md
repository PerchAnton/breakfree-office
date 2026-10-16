# Rust shell architecture

The owner mandates Rust for the core, application logic, integration and GUI.
The previous Python/Tk experiment is superseded, preserved in its separate Git
branch rather than copied into the active application. See [ADR 0002](adr/0002-rust-native-shell.md).

| Component | Responsibility |
| --- | --- |
| `src/contract.rs` | Strict UTF-8 JSON with duplicate-key rejection, checked-in JSON Schema, identity and semantic validation |
| `src/process.rs` | Trusted executable checks, bounded probes, native argument vectors and process reaping |
| `src/registry.rs` | Explicit app paths, atomic user settings, truthful readable-format routing and real recent opens |
| `src/gui.rs` | Native Rust Home/New/Open/Recent/Settings/About, menus, keyboard focus and error display; one background operation at a time |
| `src/visual.rs` | Visual-system v1 shell tokens and runtime system-font selection |
| `contracts/v1/` | Unchanged versioned editor protocol and fictional capability examples |
| `docs/visual/v1/` | Shared visual rules, tokens and reference/acceptance inventory for all three editors |

No document-supplied program, shell command, automatic download or external
manifest can select an executable. The user explicitly chooses a trusted binary;
probing it is code execution, not sandboxing or signature verification. Every
create/open request is preceded by a live capability check. Protocol mismatch,
bad JSON, missing applications, spawn failure, exit failure, timeout and bounded
stdout/stderr are handled outside rendering. POSIX cleanup terminates the probe
process group. Windows currently terminates the direct probe process only;
descendant cleanup still needs a Windows implementation and test evidence.

Recent files are local settings records of actual successful dispatches. They
start empty and are never filled with demo documents. Missing/moved files are
rechecked on open and reported; history removal does not touch document files.
New requests have no file path, so they do not fabricate a recent document.
Settings use the existing per-user Office location and preserve malformed files
instead of overwriting them. Concurrent settings writes across separate Office
instances are not coordinated in this prototype.

The four repositories remain independent; no editor source is copied here.
Office has no editing model, cloud accounts, telemetry, common server, automatic
updater or installer. No DOCX/XLSX/PPTX support is inferred from extensions.
Native dependency code, configuration and shaders do not alter the Rust-only
application/GUI decision. A source-run prototype is not a packaged OS release.

## Desktop presentation refinement

Visual specification [v1.1.0](visual/v1/README.md) retains egui/eframe and the
existing integration/settings contracts. Navigation and document-preview widgets
are native Rust drawing with AccessKit labels and keyboard focus. Recent-file
search filters only the in-memory view of persisted records; it neither adds
history nor changes dispatch. The Open location pane stacks at narrow widths;
card columns are calculated from available logical space. The capture feature
can select page, zoom, window size, menu/focus state and an isolated settings
path for repeatable own-window evidence. These controls are absent from the
normal build. Real Windows scaling and screen-reader acceptance remain open.
