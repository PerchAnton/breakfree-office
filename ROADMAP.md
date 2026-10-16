# BreakFree Office — Roadmap

**Status: planning.** This roadmap describes proposed work, not delivered features. There are no committed release dates. A milestone is complete only when its acceptance criteria have been demonstrated and reviewed.

## 0 · Agree on the foundation

- [ ] Select the software license and contribution policy with the project owner.
- [ ] Choose the first supported operating system and document why.
- [ ] Evaluate the implementation language, UI toolkit and document model with small prototypes.
- [ ] Define accessibility, keyboard navigation and file-safety requirements.
- [ ] Record the native-file and Microsoft Office interoperability strategies separately.

**Exit criteria:** a reviewed architecture decision, an explicit license choice and a buildable minimal shell on the first target platform. The repositories do not contain that shell yet.

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
