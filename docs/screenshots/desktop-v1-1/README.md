# Desktop refinement 1.1.0 — actual macOS captures

Captured application: **0.1.0-alpha.1**, unreleased. Visual specification: **1.1.0**.
These historical captures retain their actual version; see the
[current candidate and approval status](../../../VERSIONING.md).
Captured 2026-10-16 on macOS 27.0.1 arm64, native backing ratio 2.0, light theme,
English UI, installed system Arial fallback. These are unmodified framebuffer
images from the running Rust/egui/Glow application, not mockups. The approved
BreakFree mark retains its transparency. No editors are configured and the
recent list is actually empty; no sample files, accounts or templates are shown.

| View | Session zoom | Window client size before zoom | Capture |
| --- | --- | --- | --- |
| Home | 100% | 1120×760 | [Image](macos-home-100-default.png) |
| New | 100% | 1120×760 | [Image](macos-new-100-default.png) |
| Open | 100% | 1120×760 | [Image](macos-open-100-default.png) |
| Recent | 100% | 1120×760 | [Image](macos-recent-100-default.png) |
| Settings | 100% | 1120×760 | [Image](macos-settings-100-default.png) |
| About | 100% | 1120×760 | [Image](macos-about-100-default.png) |
| Home | 125% | 1120×760 | [Image](macos-home-125-default.png) |
| Home | 150% | 1120×760 | [Image](macos-home-150-default.png) |
| Home, narrow | 150% | 880×600 | [Image](macos-home-150-minimum.png) |
| Open, stacked | 150% | 880×600 | [Image](macos-open-150-minimum.png) |
| Settings, narrow | 150% | 880×600 | [Image](macos-settings-150-minimum.png) |
| File menu open | 100% | 1120×760 | [Image](macos-home-100-default-menu.png) |
| Keyboard focus on New | 100% | 1120×760 | [Image](macos-home-100-default-focus.png) |

[Capture metadata](captures.json) records each timestamp with Europe/Kyiv offset,
physical dimensions and SHA-256. Default windows produce 2240×1520 pixels;
minimum windows produce 1760×1200. Application zoom changes content density,
not the monitor's OS scale. At larger zoom the main pane deliberately scrolls;
items below the viewport are reached through the visible scrollbar/keyboard
navigation. The headless regression check also scrolls the setup link into view.

Visual inspection covers the six pages, 1/2/3-column reflow, nav/footer and status
visibility, label bounds, transparent mark, file-menu rendering and focus outline.
The actual screenshots have no populated recent files: populated rows/search
are covered by explicitly seeded temporary-file GUI tests, not real-editor
integration evidence. Native Windows dialogs and screen readers were not tested.

These are **not Windows DPI screenshots** and are **not Microsoft Office 2024
references**. Windows 100/125/150% OS scaling, exact Office 2024 Start/Backstage
build/theme/language/window matching, Narrator/NVDA and native monitor transitions
remain pending. The styling is owner-directed while those comparisons are absent.

## Reproduce

Build with the pinned toolchain and locked dependencies:

```sh
cargo build --locked --bin breakfree-office --features validation-capture
BREAKFREE_CAPTURE_TO=/absolute/path/home.png BREAKFREE_CAPTURE_SETTINGS=/absolute/path/empty-settings.json BREAKFREE_CAPTURE_PAGE=home BREAKFREE_CAPTURE_ZOOM=1.0 target/debug/breakfree-office
```

Use a nonexistent settings path for an honest empty session, or an intentionally
prepared existing file; normal user settings are the default if it is omitted.
The executable captures only its own window after 24 frames and closes it.
The feature is excluded from normal builds. Supported capture parameters:

- `BREAKFREE_CAPTURE_PAGE`: home, new, open, recent, settings, about.
- `BREAKFREE_CAPTURE_ZOOM`: 1.0, 1.25, 1.5 (app session zoom only).
- `BREAKFREE_CAPTURE_SIZE`: default, minimum, large (1440×900).
- `BREAKFREE_CAPTURE_STATE`: menu or focus; omitted for a normal page.

The menu/focus options drive real egui state for reproducible rendering; they are
not evidence of human input or native screen-reader behavior. Interaction is
checked separately by the headless keyboard/menu tests. Label all captures with
the actual host and font; never relabel these images as Windows evidence.
