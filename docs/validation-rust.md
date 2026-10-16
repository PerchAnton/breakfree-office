# Rust prototype validation

Date: 2026-10-16. Review branch: `codex/rust-office-shell`. Office version:
0.2.0. Protocol: v1. Visual specification: 1.0.0. This report supersedes the
Python experiment's readiness evidence for the active Rust implementation;
history of the old experiment remains preserved on its separate branch.

## Host and reproducibility

- macOS 27.0.1 (26A434), arm64; no Windows desktop or Linux GUI host available.
- rustc **1.90.0 (1159e78c4 2025-09-14)**; cargo **1.90.0 (840b83a10 2025-07-30)**.
- Toolchain installed from the official Rust distribution into ignored local
  `.tools/`, without changing the user's system PATH. `rust-toolchain.toml`
  pins the same version; Cargo.lock records the resolved dependency graph.
- Native GUI: egui/eframe 0.32.3, Glow, AccessKit enabled; light, English,
  system Arial fallback on this host. No JS/TS/WebView/Electron frontend.

## Executed checks

| Check | Result / scope |
| --- | --- |
| `cargo test --locked --features test-fixtures` | 22 Rust integration tests: 5 contract, 6 headless GUI/accessibility-tree, 9 real fixture-process/registry, 2 settings. No real editor integration inferred |
| `cargo fmt --check` | Passed on the pinned toolchain |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | Passed, including capture instrumentation and test-only helper |
| `cargo build --locked --bin breakfree-office --features validation-capture` | Built and ran the native Rust GUI on macOS |
| Three `--check-manifest` example commands | Live Rust validator accepts fictional Docs/Sheets/Slides examples; no Microsoft format support added |
| `--check-installations` and `--version` | Headless checks available independently of window startup; actual installation state is reported rather than populated with examples |
| Native Home captures | [Actual macOS images and metadata](screenshots/README.md), app zoom 100/125/150%, unchanged native backing ratio 2.0; visual inspection completed for these Home frames |

Transport checks include protocol/type/identity mismatch, duplicate keys, bad
UTF-8 and Unicode escapes, deep/trailing JSON, invalid formats, missing paths,
spawn/exit failure, stdout/stderr size caps, timeout, a POSIX descendant holding
pipes, and exact delivery of Unicode/spaces/quotes/shell punctuation. The Rust
helper lives only in `tests/support/`, behind `test-fixtures`, with no production
demo mode. Settings tests preserve malformed files and unsuccessful replacements;
recents are empty initially and derive only from successful opens, with dedup and
removal that does not delete the document.

The logical-DPI test covers 1.0/1.25/1.5 pixel ratios and 880×600, 1120×760,
1440×900 logical layouts. Pointer input converts AccessKit physical bounds to
egui logical coordinates explicitly: kittest 0.32.3's convenience click otherwise
uses physical bounds unchanged. Menu/navigation, Tab/Enter/Escape, unavailable
editor state, aligned cards, light visuals and transparent approved assets are
checked. These headless checks are not native OS input/accessibility acceptance.

## Unavailable checks and material limitations

| Area | Status |
| --- | --- |
| Windows screenshots at 100%, 125%, 150% OS scaling | NOT executed; no Windows environment. macOS app-zoom images do not substitute |
| Matched Office 2024 Start/Backstage baseline | Exact edition/build, theme setting, UI language, DPI and window proof unavailable. Official 2024 theme illustration verified separately; mixed-version help images rejected as certified baselines |
| Narrator/NVDA, native Windows dialogs, high contrast and monitor transitions | NOT executed; AccessKit enablement is not proof of conformance |
| Full per-page visual/resize/clipping review | Home captures inspected; logical layout/navigation tests run. Native Windows per-page focus/menu/dialog/resize screenshots and glyph clipping measurements remain pending |
| Real Docs/Sheets/Slides integration | NOT executed; no real editor build configured. Create/save/reopen and content preservation belong to the independent editors |
| Windows/Linux native build and server CI | Local host did not run them. The CI matrix is configured; actual server results must be reported separately after publication |
| Windows probe descendants | Direct child is terminated; guaranteed descendant cleanup is not implemented or tested |
| Packaging, signing, installer and release | Not provided or published; no supported-platform claim follows from this source prototype |

The visual tokens are an explicit initial specification based on the owner's
direction, not pixel measurements of an unavailable reference. The project
license is not chosen. System fonts are not redistributed; dependency/font
notices are retained separately. No document-format or Office-parity claim is made.
