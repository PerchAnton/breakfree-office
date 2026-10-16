# Development and evidence standard

- Rust is mandatory for the application core, logic, integration and GUI. Use
  the pinned toolchain and Cargo.lock. Do not introduce a JS/TS/WebView frontend.
- Keep the four repositories independent and consume shared specifications at
  reviewed immutable revisions. Editors must work without Office or each other.
- Keep contract validation, settings/registry and process handling out of GUI
  rendering. Use direct argument vectors and bound time/output at the trust boundary.
- Record source commit, runtime/toolchain, build versions, protocol, OS,
  architecture, command, result and limitations for every readiness claim.
- Separate code readiness, local tests, server CI, merge and release. Fixture
  editors cannot prove real editor integration or document-format preservation.
- Visual acceptance uses verified Windows Office 2024 images with edition/build,
  theme, language, scale and window size recorded. Label unknown fields and
  unavailable Windows 100/125/150% checks. macOS captures and logical-DPI tests
  do not substitute for native Windows evidence.
- Keep recent files and templates truthful. Test unavailable editors, keyboard
  focus, menus, cancellation/errors, resizing and clipping. Enabling AccessKit
  does not establish Narrator/NVDA conformance.
- Preserve the approved transparent BreakFree mark and existing instructions.
  Public prose/comments are English. Do not redistribute Microsoft fonts/assets
  or select a project license. Retain dependency notices independently.
- After significant stages, publish the project log and the complete private
  mirror with separate SYNC.md metadata, then read both back and compare bytes.
  Preserve others' history, use dedicated journal commits and never force push.
- Prepare reviewable commits and PRs; no automatic main merge or release.

```sh
cargo fmt --check
cargo test --locked --features test-fixtures
cargo clippy --locked --all-targets --all-features -- -D warnings
```
