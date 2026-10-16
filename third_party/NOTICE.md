# Rust dependency and font notices

The project owner's software-license decision remains open. The licenses below
apply to third-party dependencies only and do not license BreakFree's original code.
Cargo downloads dependencies with their upstream notices; source is not vendored.

- eframe/egui 0.32.3: MIT OR Apache-2.0; unchanged upstream texts retained in
  [licenses/egui-0.32.3/](licenses/egui-0.32.3/), from source commit
  `af96e0373c18477b77236e2bfc89735af007b1c2`.
- image 0.25.8, serde 1.0.228, serde_json 1.0.145, tempfile 3.23.0 and libc
  0.2.177 retain MIT/Apache notices under `licenses/<name>-<version>/`.
- jsonschema 0.33.0 and rfd 0.15.4 retain their MIT notices in those directories.
- egui's fallback fonts have additional notices preserved unchanged under
  [licenses/epaint_default_fonts-0.32.3/fonts/](licenses/epaint_default_fonts-0.32.3/fonts/):
  Ubuntu Font Licence 1.0, SIL Open Font License 1.1, Hack's notices and the emoji
  icon font MIT notice. The application uses no toolkit sample logo or artwork.

[cargo-packages-macos.json](cargo-packages-macos.json) records declared license
expressions and upstream locations for the 212 third-party packages in Cargo's
locked macOS-target metadata, including test dependencies. This is an inventory,
not a complete binary-distribution license audit or proof for unbuilt targets.
Cargo.lock remains authoritative for all resolved versions. Any future binary
distribution must collect applicable transitive/native notices for that exact
target and preserve these font obligations.

Segoe UI, Arial or DejaVu Sans are read from the user's installed system at
runtime when available and are not copied into this repository. No Microsoft
font, icon, template or screenshot is an application resource. The approved
BreakFree mark is the existing project asset; its distribution license has not
been selected by the agent.
