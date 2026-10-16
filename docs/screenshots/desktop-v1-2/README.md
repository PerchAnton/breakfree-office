# Backstage refinement 1.2.0 — actual macOS captures

Application **0.1.0-alpha.3**, unreleased and awaiting owner approval. Visual
specification **1.2.0**. Captured 2026-10-16 on macOS 27.0.1 arm64 with native
backing ratio 2.0, light theme, English UI and installed system Arial fallback.
These are unmodified native Glow framebuffer captures from the running Rust app.
No editors are configured and recents are genuinely empty; no invented files,
templates or accounts have been inserted. The approved mark keeps transparency.

| View / state | Session zoom | Actual client size before app zoom | Capture |
| --- | --- | --- | --- |
| Home / normal | 100% | 1120×760 | [Image](macos-home-100-default.png) |
| New / normal | 100% | 1120×760 | [Image](macos-new-100-default.png) |
| Open / normal | 100% | 1120×760 | [Image](macos-open-100-default.png) |
| Recent / normal | 100% | 1120×760 | [Image](macos-recent-100-default.png) |
| Settings / normal | 100% | 1120×760 | [Image](macos-settings-100-default.png) |
| About / normal | 100% | 1120×760 | [Image](macos-about-100-default.png) |
| Home / normal | 125% | 1120×760 | [Image](macos-home-125-default.png) |
| Home / normal | 150% | 1120×760 | [Image](macos-home-150-default.png) |
| Home / normal | 150% | 880×600 | [Image](macos-home-150-minimum.png) |
| Open / normal | 150% | 880×600 | [Image](macos-open-150-minimum.png) |
| Settings / normal | 150% | 880×600 | [Image](macos-settings-150-minimum.png) |
| Home / menu | 100% | 1120×760 | [Image](macos-home-100-default-menu.png) |
| Home / focus | 100% | 1120×760 | [Image](macos-home-100-default-focus.png) |
| Open / computer | 100% | 1120×760 | [Image](macos-open-100-default-computer.png) |
| Open / computer | 150% | 880×600 | [Image](macos-open-150-minimum-computer.png) |
| Open / normal | 125% | 1120×760 | [Image](macos-open-125-default.png) |
| Open / normal | 100% | 1440×815 | [Image](macos-open-100-large.png) |

The large window requested 1440×900 logical pixels; macOS constrained its height
to the available display. The table records the actual framebuffer-derived size,
not the requested size. Physical dimensions, requested size, actual size, timestamps
and SHA-256 hashes are in [captures.json](captures.json).

Inspected: selected locations, two-column Open at default size/125% zoom,
compact settings columns, menu shortcut alignment, keyboard focus, transparent
logo, wrapping and visible overflow scrolling. Minimum-size/150% content extends
below the viewport and remains scrollable; the rail stays reachable. Populated
recent-file options are covered by temporary-file regression tests, not represented
as production-editor screenshot evidence.

These are macOS session-zoom tests, **not Windows 100/125/150% OS-scaling proof**.
Native Windows dialogs, Narrator/NVDA, real-editor integration and certified
Office 2024 Start/Backstage comparison remain unexecuted. No measured similarity
percentage or exact-reference match is claimed.

## Reproduce

Build with the pinned toolchain and use isolated settings:

```sh
cargo build --locked --bin breakfree-office --features validation-capture
BREAKFREE_CAPTURE_TO=/absolute/path/open.png BREAKFREE_CAPTURE_PAGE=open BREAKFREE_CAPTURE_ZOOM=1.25 BREAKFREE_CAPTURE_SIZE=default BREAKFREE_CAPTURE_SETTINGS=/absolute/path/empty-settings.json target/debug/breakfree-office
```

Use an absent settings file for the no-editor state. The capture build reads
`BREAKFREE_CAPTURE_STATE=computer` for the This computer location, `menu` for
File, and `focus` for keyboard focus on New. These validation hooks are excluded
from normal builds. Do not replace the running capture binary during a series.

[Visual acceptance](../../visual/v1/acceptance.md) · [Validation](../../validation-rust.md)
