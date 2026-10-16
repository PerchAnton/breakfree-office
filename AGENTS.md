# Working in the BreakFree Office hub

## Repository role

This is the suite integration/brand hub and a lightweight Rust launcher. Editor implementations belong in the separate `PerchAnton/breakfree-docs`, `PerchAnton/breakfree-sheets` and `PerchAnton/breakfree-slides` repositories. Do not turn this repository into a monorepo or add Git submodules without an explicit owner decision.

## Read first

Read README.md, ROADMAP.md, BRAND.md and CONTRIBUTING.md. The suite is at the planning stage. The owner has mandated Rust for the core, application logic, integration and GUI. The GUI toolkit is an implementation choice within Rust; the software license and supported platform matrix remain undecided.

## Rules for changes

- Preserve the approved name, slogan and office-document icon. The musical-note concept was rejected for the office product.
- Keep public documentation in English. Write clearly and distinguish proposed work from tested features.
- Never invent installation commands, test results, releases, platform support or Microsoft Office compatibility.
- Do not select a license, change the brand or introduce a major architecture without owner approval.
- Never copy proprietary Microsoft code, Queen artwork or another project's branding.
- Keep the separate application links correct and use relative links for files in this repository.
- Do not add credentials, personal documents, fonts without redistribution permission, binaries or third-party assets without review.
- Do not add analytics, remote document upload or automatic macro execution as hidden behavior.

## Validation

Use the pinned rust-toolchain.toml and Cargo.lock. Run cargo fmt --check, cargo test --locked --features test-fixtures, and cargo clippy --locked --all-targets --features test-fixtures -- -D warnings. Check Markdown links, image paths and YAML. Report executed checks separately from unexecuted Windows scaling, screen-reader and Office-reference checks. Do not replace missing evidence with fake CI success badges.

## Versioning and owner approval

- Follow [VERSIONING.md](VERSIONING.md). The application version has one source:
  `[package].version` in Cargo.toml; keep Cargo.lock consistent. CLI `--version`
  and About must continue to derive it from `CARGO_PKG_VERSION`.
- Owner-mandated release numbering: all previews before the first ordinary
  release use `0.1.0-alpha.N`, `0.1.0-beta.N` or `0.1.0-rc.N` (positive integer N).
  The first ordinary published release starts at `0.2.0` (the full Cargo/SemVer
  spelling of `0.2`). Keep this startup preview series fixed; advance its suffix
  for a new preview instead of moving to another base version. Publishing source
  commits or a preview does not start the ordinary-release series.
- Increment the application version and update [CHANGELOG.md](CHANGELOG.md)
  for every completed change set, including code, fixes, documentation, rules
  and standalone journal updates. During the current alpha stage, increase N
  by one. Keep Cargo.toml, Cargo.lock and current-version summaries consistent.
- A change set includes its validation, corrections and dedicated journal commits;
  record those under the same version. A later independent change gets the next
  version. Read-only work with no repository changes does not require a bump.
- End every completed-work report to the owner with the current exact version,
  whether it changed and why, and its approval/publication status. Present the
  version for the owner's explicit approval, including when it is unchanged.
- Never infer version approval from silence, tests, CI, a previous version's
  approval or permission to implement. Record an explicit approval with its
  version, reviewed source SHA, scope, date and source; otherwise say pending.
  Reconfirm approval when that reviewed scope changes.
- Version approval, code review/merge and permission to publish a release are
  separate decisions. Do not create release tags, publish binaries or a GitHub
  release automatically. Keep released tags and contents immutable.
- Office, each independent editor, integration protocols and visual specifications
  have separate version histories; an Office bump does not bump the other repos.

## Owner-mandated implementation and visual direction

- Rust is mandatory for the core, logic, integration and GUI, not a language proposal.
- Use Cargo, a verified pinned toolchain and locked dependencies. No JavaScript/
  TypeScript GUI, Electron, or HTML/CSS/JS interface inside a WebView.
- Native OS integration, configuration, resources and shaders are allowed; system
  dependencies do not need to be rewritten in Rust.
- Target the Windows desktop Office 2024 start screens and File/Backstage layout:
  compact left navigation, light workspace, restrained typography and spacing,
  real recent files, create/open actions, app cards and genuinely available templates.
- Keep the BreakFree names, approved transparent logos and identity. Do not copy
  Microsoft's branding, redistribute Microsoft fonts/assets, or substitute a toolkit demo theme.
- Start with a light theme. Rainbow accents belong in the logos, not large UI surfaces.
- Keep shared visual tokens/versioned specifications for Docs, Sheets and Slides.
  Separate shell colors from document-content colors; consume immutable revisions,
  without sibling-folder or mutable-main build dependencies.
- Only verified Office 2024 images count as visual references. Record edition/build,
  Windows version, theme, UI language, DPI and window size; mark unknown fields.
- Acceptance requires real Windows screenshots at 100%, 125%, 150%, clipping,
  resize, focus, menu and accessibility checks. Simulated pixel ratios on another
  OS are additional layout tests, never Windows screenshot evidence.
- No invented recents, templates, accounts or inert controls. Missing editors must
  explicitly appear as not installed. Editors remain independent executables.

## Project journal

- Keep this project's public journal in `docs/PROJECT_LOG.md`. Update it after
  significant stages and requested checks, using dedicated commits and the
  branch/PR workflow. Preserve other authors' work and never force push.
- Keep public prose in English. Retain work/check history and the sections
  Current state, Work and checks, Comments and reviews, Offers and requests for
  help, Next actions and Check history. Use tables for events and actions.
- Record Europe/Kyiv timestamps, source commit/PR/issue links, actual checks and
  limitations. Distinguish code readiness, local tests, CI, merge and release.
- Check issues, PR conversations, reviews, inline threads and Discussions when
  enabled, including new replies on older objects. Record source coverage and
  unavailable sources; absence of a check is not evidence of no events.
- Record each event's @author, human/bot identity, date, direct URL, requested
  action and handling status. Keep help offers and requests separate. Do not
  infer assignments from offers, stars or forks; confirm assignees explicitly.
- Reconcile history without loss and update an event by its source URL rather
  than duplicating it. Do not send public replies or invitations without a
  separate owner instruction. No scheduled monitoring is implied.
