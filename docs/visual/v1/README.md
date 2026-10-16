# BreakFree desktop visual system v1

Version: **1.2.0**. This specification applies to Office, Docs, Sheets and Slides.
The required visual target is the Windows desktop Office 2024 Start and
File/Backstage experience, with BreakFree identity. Values in
[tokens.json](tokens.json) are explicit initial implementation values, not falsely
claimed pixel measurements of an unavailable reference capture.

## Layout and density

Use a compact 176-DIP left rail with 40-DIP navigation rows and a light main
workspace. Office's sections are Home, New, Open, Recent, Settings and About;
this is BreakFree navigation, not a claim about an exact Microsoft product menu.
Use 36-DIP content margins and multiples of 4 for spacing, 8 between related
controls and 16–24 between groups. Prefer compact office-sized app/template
176×152-DIP blank-file cards and 60-DIP document rows rather than hero panels, a marketing layout or mobile
navigation. Initial window: 1120×760 logical pixels; minimum: 880×600. Reflow
cards and scroll content when needed; keep navigation and status reachable.

## Typography and color ownership

Page title 28, section 18, app title 15, body/button 14 and captions 12 logical
pixels. Prefer the user's installed Segoe UI on Windows; load it at runtime,
never copy it into the repository. macOS Arial and Linux DejaVu Sans are system
fallbacks with different glyph metrics. egui's licensed fallback is last resort.
Report the actual font used in About and validation evidence. Do not redistribute
Microsoft fonts, icons, templates or screenshots as application resources.

Shell surfaces are white/light gray, text dark neutral and borders subdued.
Selection and focus use a small BreakFree accent. Preserve the approved RGBA
mark's transparent pixels; rainbow stays in the mark. Document page, slide and
cell colors belong to editor/content themes, never shell theme inheritance.

## Components and behavior

| Component | Shared requirement |
| --- | --- |
| Buttons | 30-DIP minimum height; normal, hover, pressed, disabled and visible focus states; no actionable-looking inert control |
| Navigation | Selected state plus text; keyboard reachable; Alt+1…6 selects shell pages |
| Menus | Compact File and Help groups; keyboard focus, Escape dismissal; include only implemented actions |
| Dialogs | Native local file/executable pickers; clear purpose, cancel without mutation; app-choice dialog when formats are ambiguous |
| Panels | Main content scrolls and wraps; stable left rail; no fake recent/template/account rows |
| Status | 26-DIP bar; installed-editor state and busy indication; actionable errors appear as visible text, not only color |
| Accessibility | AccessKit semantic labels, stable keyboard order, visible focus; native Narrator/NVDA and high-contrast checks remain acceptance gates |

Ctrl/Cmd+N opens New, Ctrl/Cmd+O opens the native file picker; Tab/Shift+Tab
move focus and Enter/Space activate controls. A missing editor is explicitly not
installed/unavailable; Create is disabled until the live editor advertises `new`.
No template card is shown without an actual available template and supported
dispatch. Protocol v1 currently supplies no template-discovery action.

Consume these specifications at a reviewed immutable commit. Breaking visual
semantics require a new major specification directory, additive changes a minor
version; fixes increment patch. No editor build may depend on a sibling checkout
or mutable `main`. Sharing a document is not justification for extracting a library.

See [reference inventory](reference.md) and [acceptance plan](acceptance.md).

## 1.1.0 desktop refinement

This additive visual-spec revision keeps the same Rust toolkit and six-page
navigation. Navigation uses original line pictograms, a 3-DIP selection marker,
neutral selected/hover surfaces and a separate keyboard-focus outline. Settings
and About sit at the foot of the rail. Header spacing contracts at short heights
so the full navigation remains reachable at the tested minimum window/session zoom.

The white workspace shows blank-file previews at a fixed 176×152 DIP, with
16-DIP gaps and explicitly calculated 1/2/3-column wrapping. These are illustrations
of the existing `new` actions, not installed templates. Disabled actions retain
readable text and an explicit Not installed/Unavailable state. A working setup
link goes to Settings; creation still requires the editor's validated capability.

Recent files use aligned Name/Last opened columns, full-width actionable rows,
a filename/path search and ellipsis for long text. Full paths remain available
through tooltips and semantic labels. Search never changes stored history;
empty and no-match states remain explicit. Open places This computer beside
recent files at wide widths, stacking below 620 DIP of content. Only local
browsing is offered. Solid scrollbars expose overflowing content at larger zoom.

The File menu keeps working shortcuts (Cmd on macOS, Ctrl elsewhere) and closes
with Escape. Pictograms and paper/grid previews are drawn in Rust, not imported
Microsoft assets. [Actual captures](../../screenshots/desktop-v1-1/README.md)
and regression checks accompany this revision. No verified Office 2024
Start/Backstage pixel measurements or Windows acceptance are asserted.

## 1.2.0 Backstage and desktop controls

Open uses a 204-DIP location list with 48-DIP selected rows for Recent documents
and This computer, plus a working Browse command. The right pane shows actual
recents or local-file browsing; it contains no cloud placeholders. Below 600 DIP
of content width, locations stack above the selected content. Each main page
retains its own scroll position rather than inheriting another page's offset.

Recent rows reserve a separate 38-DIP action column. The visible options button
has a unique full-path accessible label, Tab/Enter access and Escape dismissal.
Removal changes history only; it never deletes the file. Existing right-click
removal is retained. File-menu commands use a separate aligned shortcut column
and a minimum width of 220 DIP.

Settings align editor identity/status and setup actions in two columns at 600 DIP
and above, stacking below that width. Forget location appears only for configured
paths. Appearance groups the implemented session-zoom setting. Original Rust-drawn
computer/file glyphs use the same restrained stroke and shell colors. No new
Microsoft assets, fonts, accounts or unsupported editor actions are introduced.

See [alpha.3 macOS captures](../../screenshots/desktop-v1-2/README.md). Dimensions
remain provisional until matched Windows Office 2024 reference evidence exists.
