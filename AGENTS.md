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
- Maintain [CHANGELOG.md](CHANGELOG.md) for the current unreleased candidate.
  Do not bump the version for every task or documentation/journal update.
  Propose a justified SemVer change when the release scope requires one.
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

## Журнал и приватное зеркало

This section supersedes any earlier prohibition on duplicating the project log
in `gitwork`. It does not replace the other instructions above.

- Project: `PerchAnton/breakfree-office`, https://github.com/PerchAnton/breakfree-office.
- Primary log: `docs/PROJECT_LOG.md` in this project's own repository.
- Required full private copy: https://github.com/PerchAnton/gitwork, branch `main`,
  exact path `breakfree/breakfree-office/PROJECT_LOG.md`.
- Mirror metadata: `breakfree/breakfree-office/SYNC.md` in `gitwork/main`.
- Read the current `prompts/breakfree-office.md` in `gitwork` before updating the
  log. After every significant stage and whenever a project check is requested,
  update and publish BOTH copies. Publishing only the primary is incomplete.
- Fetch remote updates safely, read existing instructions and both logs, and
  preserve unrelated local changes and other authors' history. If the mirror
  contains entries absent from the primary, reconcile without loss in the primary
  and publish that first. Update an event with the same source URL instead of
  duplicating it. Never use force push.
- Keep public prose in English. Use the sections Current state (Текущее состояние),
  Work and checks (Работа и проверки), Comments and reviews (Комментарии и ревью),
  Offers and requests for help (Предложения и просьбы о помощи), Next actions
  (Следующие действия), and Check history (История проверок). Keep events and
  actions in tables and retain previous history.
- Record date/time with `Europe/Kyiv`, completed work, commit/PR/issue links,
  actual checks and limitations. Distinguish code readiness, local tests, remote
  CI, merge and release. Do not turn fixture results into real-editor evidence.
- Check issues, PR conversations, reviews, inline threads and Discussions when
  enabled. For each event record `@author`, human/bot identity, date, direct source
  URL, request, required response and handling status. An unchecked or unavailable
  source is not evidence that there are no events.
- Record help offers and help requests separately with author, task, message URL
  and whether an owner response is needed. An offer is not an assignment; stars
  and forks are not help offers. Record task priority, concrete next action,
  source, status, blocker and an assignee only when confirmed.
- Document the checked sources, objects, time and covered period. State the full
  coverage on the first check. Later checks cover events since the previous check,
  including new replies on old issues and PRs. No scheduled monitoring is implied.
- Publish journal updates in dedicated commits through this project's branch/PR
  workflow, excluding unrelated files and secrets. Do not publish private
  correspondence or send public replies, invitations or assignments without a
  separate owner instruction. Creating the requested publication PR is allowed.
- After primary publication, fetch fresh `gitwork/main`, re-read this project's
  existing mirror, and copy the entire published primary text byte for byte.
  Keep the metadata outside the mirrored log: source repository, source branch,
  source path, full source commit SHA, immutable source-file URL and synchronization
  time with offset and `Europe/Kyiv`.
- Commit the mirror and metadata separately in `gitwork/main`. Update only this
  project's row in its Breakfree README table with the status, source/mirror
  links, check time and next step. Preserve other projects' rows and files.
- If a push is rejected after concurrent updates, fetch, reconcile all authors'
  entries and retry without force push. Read both published files back from
  GitHub and compare their full text before reporting synchronization success.
- If private publication is unavailable, preserve a ready local copy and report
  explicitly: «приватная копия не отправлена» (the mirror was not sent). Never
  present a successful primary push as proof of a successful mirror push.
- Report links to AGENTS.md, both published logs and commits in both repositories,
  followed by a project summary table: project, changes, commenters, help
  offers/requests, next actions and check date. Distinguish a fresh GitHub check
  from a summary of the saved log.
