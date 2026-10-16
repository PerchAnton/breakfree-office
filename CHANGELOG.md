# Changelog

Only published releases receive a release date and tag link. This file starts
with the existing Rust prototype; no historical release is asserted.
Version selection and owner approval follow [VERSIONING.md](VERSIONING.md).

## Unreleased

**Candidate: `0.1.0-alpha.3`. Owner approval pending; no release published.**
All initial alpha/beta/RC versions use the `0.1.0` base. The first ordinary
published release is reserved as `0.2.0` (short form `0.2`).

### 0.1.0-alpha.3 — current candidate

#### Changed

- Refine Open into a Backstage-style location list with selected Recent documents
  and This computer views, native Browse actions and responsive stacking.
- Align File-menu shortcut hints in a separate column and keep compact menu sizing.
- Add a visible, keyboard-accessible options menu to each real recent-file row;
  removal changes the list only and preserves the document.
- Compact editor setup into aligned details/actions on wide windows, stacking
  at narrow widths; group Appearance controls and keep per-page scroll positions.
- Advance the shared visual specification to 1.2.0 and the Cargo application
  version to 0.1.0-alpha.3. Existing alpha.1 captures remain historical evidence.
- Record explicit owner approval of alpha.2 at source f4b805e; alpha.3
  remains awaiting approval. No merge, release tag or binary publication.

### 0.1.0-alpha.2 — approved, unreleased

Owner-approved on 2026-10-16 for reviewed source
[f4b805e73549584edf6efdc6e2282f39657f692c](https://github.com/PerchAnton/breakfree-office/commit/f4b805e73549584edf6efdc6e2282f39657f692c);
scope: Rust launcher, visual 1.1.0 desktop refinement and per-change version policy.
Approval was given explicitly in the project conversation; release publication
and PR merge were not authorized.

#### Changed

- Increment the application version and add a version-specific changelog entry
  for every completed change set, including documentation and standalone journal
  updates. Related checks and journal commits complete the same versioned change.
- Advance Cargo.toml/Cargo.lock to `0.1.0-alpha.2`; CLI/About derive the new version
  from Cargo. Update current documentation and retire the documentation-only bump
  exception. Approval was pending at implementation; the later alpha.2 decision
  is recorded above. Release publication remains pending.
- Refine the desktop shell with visual specification 1.1.0: compact icon navigation,
  bottom-aligned Settings/About, white workspace, blank-file previews with explicit
  column reflow, aligned recent rows and a two-pane Open view.
- Add filtering of actual recent filenames/paths without changing their stored
  history. Keep full paths in tooltips/accessibility labels and explicit empty states.
- Restore Escape dismissal of the File menu, show platform-correct shortcut hints,
  and keep navigation reachable at minimum size with 150% session zoom. Add native
  macOS framebuffer evidence; Windows visual acceptance remains pending.
- Attribute the completed desktop refinement to this candidate. Earlier alpha.1
  captures and check results retain their original version and source labels.

### 0.1.0-alpha.1 — previous unreleased candidate

This baseline was not published as a release. The desktop refinement was first
validated under alpha.1 and is tracked in alpha.2 above under the revised policy.

#### Added

- Rust-native Office launcher with Home, New, Open, Recent, Settings and About;
  trusted standalone-editor configuration and real successfully dispatched recents.
- Strict integration-v1 manifest checks, bounded capability probes and direct
  argument dispatch; unavailable editors are explicitly identified.
- Pinned Rust 1.90.0, locked dependencies, Rust fixture/GUI checks and three-OS CI.
- Shared visual-system v1 and a compact light desktop layout retaining the
  approved transparent BreakFree identity.
- SemVer policy, future release procedure and mandatory end-of-work version
  reporting for explicit owner approval, with Cargo as the version source.

#### Changed

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
