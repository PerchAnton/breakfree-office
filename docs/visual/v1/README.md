# BreakFree desktop visual system v1

Version: **1.0.0**. This specification applies to Office, Docs, Sheets and Slides.
The required visual target is the Windows desktop Office 2024 Start and
File/Backstage experience, with BreakFree identity. Values in
[tokens.json](tokens.json) are explicit initial implementation values, not falsely
claimed pixel measurements of an unavailable reference capture.

## Layout and density

Use a compact 184-DIP left rail with 36-DIP navigation rows and a light main
workspace. Office's sections are Home, New, Open, Recent, Settings and About;
this is BreakFree navigation, not a claim about an exact Microsoft product menu.
Use 32-DIP content margins and multiples of 4 for spacing, 8 between related
controls and 16–24 between groups. Prefer compact office-sized app/template
cards and document rows rather than hero panels, a marketing layout or mobile
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
