# Actual macOS prototype captures — not Windows DPI evidence

Captured 2026-10-16 from the running Rust 0.2.0 native application on macOS
27.0.1 arm64 using Glow's own-window screenshot event after eight UI frames.
These are real framebuffer captures, not mockups or images of other apps.

The `0.2.0` number above identifies that historical capture session, not a
published release or the current candidate. Subsequent owner instructions reserve
initial previews for the `0.1.0` family; see [versioning](../../VERSIONING.md).

| Image | App session zoom | Native backing ratio | Framebuffer size |
| --- | --- | --- | --- |
| [Home 100%](macos-home-zoom100.png) | 1.0 | 2.0 | 2240×1520 px |
| [Home 125%](macos-home-zoom125.png) | 1.25 | 2.0 | 2240×1520 px |
| [Home 150%](macos-home-zoom150.png) | 1.5 | 2.0 | 2240×1520 px |

Window client size before app zoom: 1120×760 platform logical pixels. Theme:
light. UI language: English. Interface font: Arial read from this macOS system.
The native display scale stays unchanged; app zoom changes content density and
may wrap button labels. Cards, navigation, text, transparent logo and empty
recent state were visually inspected. No real editors or document filenames are
invented or installed for these captures.

These images are NOT Windows 100/125/150% OS display-scaling screenshots, and are
NOT reference images of Microsoft Office. Native Windows captures, matched
Office 2024 Start/Backstage references, Narrator/NVDA, native-dialog interaction
and full per-page visual acceptance remain unavailable/unexecuted.

Reproduce a self-capture (feature is excluded from the normal build):

```sh
cargo build --locked --bin breakfree-office --features validation-capture
BREAKFREE_CAPTURE_TO=/absolute/path/macos-home.png BREAKFREE_CAPTURE_ZOOM=1.0 target/debug/breakfree-office
```

Allowed capture zoom values are 1.0, 1.25 and 1.5. The feature captures only this
application's framebuffer and closes its window after saving. Change the file
name/metadata if capturing on another OS; never reuse the macOS labels as proof
of Windows scaling.

## Desktop refinement capture session

See [visual-system 1.1.0 captures](desktop-v1-1/README.md) recorded with
`0.1.0-alpha.1`: all six pages, menu/focus states and small-window checks.
The [current candidate](../../VERSIONING.md) advances independently of these
historical images; their original version labels are preserved.
The images above remain historical evidence of the initial prototype.
