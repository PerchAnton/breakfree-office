# Visual and desktop acceptance gates

| Stage | Dependency | Exit criterion | Current state |
| --- | --- | --- | --- |
| V1: certified baseline | Real Windows Office 2024 | Product/build proof and Start/Backstage captures for Word/Excel/PowerPoint; theme, language, DPI and window measurements logged | Pending; only the official 2024 theme image is verified, not the required screens |
| V2: Rust shell structure | Mandatory Rust decision; v1 visual tokens | Six compact pages, meaningful menus/actions, transparent approved logo, honest absent-editor/template/recent states | Visual v1.1.0 implemented for review; compact icon navigation, real recent search/table and responsive previews/Open pane; actual macOS captures recorded |
| V3: keyboard and semantic access | V2 | Navigation, menus, focus, shortcuts, cancel/error paths and accessible labels tested; no clipped required controls across minimum/default/large layouts | 8 headless GUI tests cover navigation/menu dismissal/focus, logical DPI, session-zoom bounds/reflow and recent search; native Windows acceptance pending |
| V4: Windows scale and visual comparison | V1–V3; real Windows build | Actual prototype screenshots at 100/125/150% OS scaling; resize/text clipping, focus, menus, native dialogs, Narrator/NVDA, no black logo rectangles; compare with matched references | Not executed; no Windows host available |
| V5: editor adoption | Reviewed immutable visual+integration revisions; real editor builds | Docs/Sheets/Slides independently implement common density/states while preserving document-content theme ownership; standalone workflows and Office dispatch verified | Pending; no real-editor integration evidence |

At each scale capture Home/New/Open/Recent(empty and real data)/Settings/About,
File menu open, visible keyboard focus, missing editor, failed launch and ambiguous
file choice. Use only real files created for the run and genuinely installed
editor builds; identify fixture screenshots separately. Record actual glyph
clipping bounds, minimum 880×600, default 1120×760, large 1440×900 logical windows,
monitor transitions and unavailable checks. Screenshots and pixel-ratio tests on
macOS/Linux must be labeled as such, never presented as Windows scaling evidence.
