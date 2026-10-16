# Versioning and release approval

BreakFree Office follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html).
This policy governs future releases; it does not publish or approve one.

## Current candidate and source of truth

The current application version is **`0.1.0-alpha.1`**, an **unreleased alpha
candidate awaiting owner approval**. The earlier internal `0.2.0` designation
was never released and is superseded by the owner's preview-numbering decision.
[CHANGELOG.md](CHANGELOG.md) records the candidate's scope.

[`[package].version` in Cargo.toml](Cargo.toml) is the only application-version
source. Cargo.lock records the same package version. CLI `--version` and About
already use `env!("CARGO_PKG_VERSION")`; do not add a competing VERSION file or
hard-coded runtime number. Documentation status summaries must agree with Cargo.
See the [Cargo version field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field).
Use the Git commit SHA to distinguish work within one unreleased candidate.

## Choosing the next version

The owner has reserved the `0.1.0` base for **all alpha, beta and release-candidate
iterations before the first ordinary release**. Use these exact SemVer forms:

| Stage | Version series | Advancement |
| --- | --- | --- |
| Alpha | `0.1.0-alpha.1`, `0.1.0-alpha.2`, … | Increase N for a new alpha candidate |
| Beta | `0.1.0-beta.1`, `0.1.0-beta.2`, … | Start beta at 1, then increase N |
| Release candidate | `0.1.0-rc.1`, `0.1.0-rc.2`, … | Start RC at 1, then increase N |
| First ordinary published release | **`0.2.0`** | Only after owner approval and completed publication gates |

Here N is a positive integer without leading zeroes. The owner's short form
`0.2` is written `0.2.0` in Cargo, tags and release notes. `0.1.0x` describes the
preview family; it is not a literal Cargo version. Do not publish a bare `0.1.0`
release or move previews to a `0.2.0` base before the first ordinary release.
Source pushes, PRs and preview publications do not constitute that ordinary
release. Version changes and stage transitions still require owner approval;
do not create or publish all the listed example versions automatically.

After the first ordinary `0.2.0` release, apply the increment rules below. This
startup sequence does not imply production stability or Microsoft compatibility.

| Change | Version rule |
| --- | --- |
| Documentation, journal, review or work inside the existing unreleased scope | Keep the candidate version; record meaningful scope changes in the changelog |
| Before the first ordinary release | Keep the `0.1.0` base; advance alpha/beta/RC suffixes using the owner-mandated sequence above |
| Published initial-development series (`0.2.0` onward, before `1.0.0`) | Increment MINOR for a new feature set or incompatible change, resetting PATCH; increment PATCH for compatible fixes to that minor line. Compatibility is provisional and breaking changes must be explicit |
| Stable releases (`1.0.0` onward) | Increment MAJOR for incompatible public behavior/contracts, MINOR for compatible features/deprecations, PATCH for compatible fixes; reset lower fields when a higher field increases |
| Preview distribution | Every published preview gets a unique version; before the first ordinary release, use only the `0.1.0-alpha.N` / `0.1.0-beta.N` / `0.1.0-rc.N` series |

The public compatibility surface includes documented CLI behavior, editor
integration, persisted settings/recent-file data and any supported migration
guarantees. GUI improvements and dependencies must be assessed for their effect
on that surface and the supported platform/toolchain requirements. `1.0.0`
requires an explicit reviewed stability/compatibility commitment.

Choose the increment from changes since the previous published version, not the
number of tasks or commits. A version may remain unchanged across several tasks
while its release scope is under review. Never modify an already released
version's contents or move its tag; publish a new version for a correction.

Office, Docs, Sheets and Slides version independently. Integration protocol v1,
visual-system v1 and the Rust toolchain are separate version domains. Record
the tested application/protocol/visual revisions; matching numbers do not prove
compatibility. Shared dependencies stay pinned to immutable revisions.

## End-of-work report and approval

End **every** completed-work report to the owner with:

- The current exact version read from Cargo.toml, including any preview suffix.
- Whether it changed, with the reason for changing or retaining it.
- Its state: approval pending, explicitly approved for a specified revision, or
  actually published with a release link. A proposal is not an approved release.
- The version presented for the owner's explicit approval.

Record an approval in the project log with version, reviewed full source SHA,
scope, date/time and an owner-authorized source reference. Do not publish private
correspondence. If approval exists only in this chat, record the decision summary
and revision, not the private message text. Changes to the reviewed scope require
renewed approval even when the version number is unchanged. Silence, successful
tests, CI, a merge and approval of a different version do not grant approval.

Version approval does not authorize merging or publishing. No release tag,
GitHub release, installer or binary is created without explicit publication
authorization. There is no automatic release workflow in this repository.

## Preparing a future release

1. Propose the version and scope in a PR. Update Cargo.toml and the root package
   entry in Cargo.lock together when a bump is needed, using the pinned toolchain.
   Update the changelog and all current-version summaries in that same change.
2. Run the required [validation](AGENTS.md#validation), check `--version`, and
   record the exact revision, CI results, tested targets and remaining limitations.
   Complete the applicable [roadmap release gates](ROADMAP.md#5--prepare-verified-releases).
3. Obtain explicit owner version/scope approval and the separate review/merge
   and publication decisions. Keep pending gates visible in both journal copies.
4. Once authorized, finalize dated changelog/release notes and verify the final
   release SHA, Cargo version, `v<version>` tag and artifact names agree. Attach
   only verified artifacts with checksums and platform/format limitations.
5. Record the actual tag, release URL, source SHA, approval and checks in the
   project log; publish its full private mirror and SYNC.md. Start a new
   Unreleased changelog section without inventing the next approved version.

Until publication, keep the changelog under **Unreleased** with no release date.
