# BreakFree Office — Roadmap

**Status: Rust shell prototype under review; editor milestones pending.** Rust is owner-mandated for the core, application logic, integration and GUI. The items below distinguish implemented prototype work from unverified acceptance. There are no committed release dates. A milestone is complete only when its acceptance criteria have been demonstrated and reviewed.

## 0 · Agree on the foundation

- [ ] Select the software license and contribution policy with the project owner.
- [ ] Choose the first supported operating system and document why.
- [x] Record the owner-mandated Rust language decision; no language selection remains open.
- [x] Select native egui/eframe for the Rust-authored shell prototype; keep screen-reader and visual acceptance open.
- [ ] Review the document model independently in each editor.
- [ ] Define accessibility, keyboard navigation and file-safety requirements.
- [ ] Record the native-file and Microsoft Office interoperability strategies separately.

**Exit criteria:** a reviewed architecture decision, an explicit license choice and a buildable minimal shell on the first target platform. A source-run Rust shell is now under review; packaging, license selection and platform acceptance are not complete.

## Office integration and visual delivery

| Stage | Dependencies | Acceptance criterion | Status |
| --- | --- | --- | --- |
| Contract v1 | Independent editor boundaries | Schema, strict JSON/identity/semantic checks, examples and rejection tests | Existing v1 preserved and ported to Rust; see validation evidence |
| Rust shell | Mandatory Rust; pinned Cargo/toolchain | Real create/open/configuration; no-editor state; bounded background probes; direct arguments; real recents | Implemented for review; no production release |
| Certified visual baseline (V1) | Real Office 2024 on Windows | Start/Backstage screenshots with edition/build, theme, language, scale and window dimensions | Pending; only official Office 2024 theme illustration verified |
| Common visual system (V2) | V1 direction; own identity | Versioned density, fonts, colors, component states; compact six-page light shell; transparent mark | v1 specified/implemented; exact reference measurements pending |
| Keyboard/layout (V3) | V2 | Focus/menu/shortcut checks and no clipped required controls at minimum/default/large layouts | Headless logical-DPI checks planned/executed per validation report; native acceptance pending |
| Windows scale/accessibility (V4) | Certified baseline; real Windows prototype | Screenshots at 100%, 125%, 150%; resize, clipping, focus, menus, native dialogs, Narrator/NVDA and matched-reference comparison | Not executed; Windows environment unavailable |
| Independent editor adoption (V5) | Reviewed immutable contract/visual revision; real editor builds | Docs first, then Sheets/Slides; standalone save/reopen plus Office dispatch; real-build/version matrix | Pending; fixtures do not establish editor integration |
| Content/common settings | Validated editor/version matrix | Separate versioned exchange/settings contract, preservation tests and demonstrated reuse | Not started |
| Suite distribution | Verified OS builds; owner-selected license | Independent apps preserved; packaging/signing/notices/install checks and release approval | Not started; no common installer/auto-release |

Detailed gates: [visual acceptance](docs/visual/v1/acceptance.md). A matching
headless pixel ratio on macOS is not a Windows scaling screenshot. Shared Rust
components require demonstrated reuse and pinned versions; no sibling-directory
or mutable-main dependency is permitted.

## 1 · Make writing dependable

Start in [BreakFree Docs](https://github.com/PerchAnton/breakfree-docs).

- [ ] Create and edit a simple document.
- [ ] Add basic character and paragraph formatting.
- [ ] Support undo/redo, save, reopen and unsaved-change prompts.
- [ ] Define a versioned local persistence format and round-trip tests.
- [ ] Test recovery from interrupted writes without destroying a good file.

**Exit criteria:** the create → edit → save → reopen → continue workflow preserves all supported content. A reproducible build and a documented test command exist.

## 2 · Expand documents carefully

- [ ] Add styles, lists, tables and images in small, tested increments.
- [ ] Investigate pagination and print/PDF export separately from editing.
- [ ] Implement and document an explicitly scoped DOCX subset.
- [ ] Collect redistributable test fixtures for Unicode, layout and unsupported content.

**Exit criteria:** a compatibility matrix describes exactly what survives import/export; unsupported features are reported rather than silently discarded.

## 3 · Build the other applications

### BreakFree Sheets

Follow the [Sheets roadmap](https://github.com/PerchAnton/breakfree-sheets/blob/main/ROADMAP.md): a grid, typed values, formulas, recalculation and reliable persistence before advanced Excel compatibility.

### BreakFree Slides

Follow the [Slides roadmap](https://github.com/PerchAnton/breakfree-slides/blob/main/ROADMAP.md): a slide model, basic objects, editing, persistence and presentation mode before advanced PowerPoint compatibility.

**Exit criteria:** each application has its own working minimum release, test suite and documented limitations. Parallel work is possible when maintainers and infrastructure are ready.

## 4 · Share what has proved useful

Extract genuinely reusable components only after more than one application needs them. Candidates include design tokens, file dialogs, recovery, accessibility patterns and export services. Use versioned dependencies; a shared-library repository is optional and does not exist yet.

## 5 · Prepare verified releases

- [ ] Test installation, upgrade, uninstallation and file associations on declared targets.
- [ ] Publish real build artifacts, checksums, release notes and known issues.
- [ ] Document how files, preferences and recovery data are stored.
- [ ] Establish a private vulnerability-reporting route before public binaries.
- [ ] Review third-party licenses and redistribution notices.
- [ ] Add supported platforms only after their builds and workflows have been tested.

## Not promised for the first release

Full Microsoft Office parity, VBA/macros, enterprise administration, cloud collaboration, mobile clients and perfect round-trip support for every document format are outside the initial commitment. They need separate investigation and prioritization.

[Back to BreakFree Office](README.md)
