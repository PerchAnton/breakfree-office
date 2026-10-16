# Changelog

Only published releases receive a release date and tag link. This file starts
with the existing Rust prototype; it does not invent a historical `0.1.0` release.
Version selection and owner approval follow [VERSIONING.md](VERSIONING.md).

## Unreleased

**Candidate: `0.2.0`. Owner approval pending; no release published.**

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

- Rust is required for the core, logic, integration and GUI. The earlier local
  Python experiment is superseded and is not a published release.

### Known limitations and pending acceptance

- No real editor integration, declared supported-platform matrix, installer or
  verified Microsoft-format compatibility; the project license is undecided.
- Actual Windows 100/125/150% screenshots, verified Office 2024 Start/Backstage
  references and native screen-reader acceptance remain pending.
- Windows probe descendant cleanup is limited; see the
  [validation report](docs/validation-rust.md) for check scope and limitations.
- A historical [Linux CI run](https://github.com/PerchAnton/breakfree-office/actions/runs/38876088705)
  failed the `hung_probe_is_bounded` assertion. Later runs passed; the cause of
  this intermittent result still needs investigation before release acceptance.

[Back to BreakFree Office](README.md)
