# Changelog

Only published releases receive a release date and tag link. This file starts
with the existing Rust prototype; no historical release is asserted.
Version selection and owner approval follow [VERSIONING.md](VERSIONING.md).

## Unreleased

**Candidate: `0.1.0-alpha.1`. Owner approval pending; no release published.**
All initial alpha/beta/RC versions use the `0.1.0` base. The first ordinary
published release is reserved as `0.2.0` (short form `0.2`).

### Added

- Rust-native Office launcher with Home, New, Open, Recent, Settings and About;
  trusted standalone-editor configuration and real successfully dispatched recents.
- Strict integration-v1 manifest checks, bounded capability probes and direct
  argument dispatch; unavailable editors are explicitly identified.
- Pinned Rust 1.90.0, locked dependencies, Rust fixture/GUI checks and three-OS CI.
- Shared visual-system v1 and a compact light desktop layout retaining the
  approved transparent BreakFree identity.
- SemVer policy, future release procedure and mandatory end-of-work version
  reporting for explicit owner approval, with Cargo as the version source.

### Changed

- Refine the desktop shell with visual specification 1.1.0: compact icon navigation,
  bottom-aligned Settings/About, white workspace, blank-file previews with explicit
  column reflow, aligned recent rows and a two-pane Open view.
- Add filtering of actual recent filenames/paths without changing their stored
  history. Keep full paths in tooltips/accessibility labels and explicit empty states.
- Restore Escape dismissal of the File menu, show platform-correct shortcut hints,
  and keep navigation reachable at minimum size with 150% session zoom. Add native
  macOS framebuffer evidence; Windows visual acceptance remains pending.

- Replace the never-released internal `0.2.0` designation with
  `0.1.0-alpha.1` in Cargo and runtime version reporting, following the owner's
  preview-numbering decision. Alpha/beta/RC iterations remain in the `0.1.0`
  family until the first ordinary `0.2.0` release.
- Rust is required for the core, logic, integration and GUI. The earlier local
  Python experiment is superseded and is not a published release.

### Known limitations and pending acceptance

- No real editor integration, declared supported-platform matrix, installer or
  verified Microsoft-format compatibility; the project license is undecided.
- Actual Windows 100/125/150% screenshots, verified Office 2024 Start/Backstage
  references and native screen-reader acceptance remain pending.
- Windows probe descendant cleanup is limited; see the
  [validation report](docs/validation-rust.md) for check scope and limitations.
- Intermittent Linux fixture-process start and timeout checks failed in prior
  CI, including [run 38878267706](https://github.com/PerchAnton/breakfree-office/actions/runs/38878267706).
  Other runs passed; the cause remains undiagnosed and is a release-acceptance gate.

[Back to BreakFree Office](README.md)
