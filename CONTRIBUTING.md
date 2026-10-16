# Contributing to BreakFree Office

Thank you for helping shape BreakFree. The project is at the planning stage; workflow examples, accessibility feedback, documentation improvements and focused technical proposals are especially useful now.

## Choose the right repository

| Topic | Where it belongs |
| :--- | :--- |
| Suite identity, common principles and cross-application decisions | [breakfree-office](https://github.com/PerchAnton/breakfree-office/issues) |
| Writing, document models and DOCX workflows | [breakfree-docs](https://github.com/PerchAnton/breakfree-docs/issues) |
| Grids, formulas, data and XLSX workflows | [breakfree-sheets](https://github.com/PerchAnton/breakfree-sheets/issues) |
| Slides, presentation mode and PPTX workflows | [breakfree-slides](https://github.com/PerchAnton/breakfree-slides/issues) |

## Propose a change

Search existing issues, describe the user problem and give one concrete example. Explain the smallest useful outcome, alternatives and how completion could be tested. Discuss major architecture, dependencies, licensing and changes to the approved brand before implementing them.

A feature idea is not an implementation commitment. Maintainer review determines scope and priority.

## Submit a pull request

Keep changes focused. Link the relevant issue, explain what changed and describe the checks you actually ran. For documentation, verify relative links and distinguish planned features from existing ones. Once code exists, include regression tests for fixes and reproducible build/test instructions.

Do not claim tests passed when they were not run. Do not introduce fake download links, release badges, screenshots of nonexistent features or platform support without evidence.

## Versions and releases

Follow [VERSIONING.md](VERSIONING.md) and update [CHANGELOG.md](CHANGELOG.md)
for every completed change set, including documentation and standalone journal
updates. Increment the application version (currently the alpha suffix by one),
keep Cargo.toml/Cargo.lock consistent, and record the reason and scope. Validation
and journal commits completing the same change set share that version. Report the exact
current version and approval status at the end of every completed task. Only an
explicit owner decision approves a version; approval to publish a release is
separate. PRs, successful CI and merges do not publish a release automatically.

## Licensing first

A software license has not been selected. Please discuss licensing with the owner before submitting substantive implementation code or third-party assets. Do not copy proprietary code, restricted fonts, logos or unlicensed test documents into the project. List the origin and license of any proposed dependency or fixture.

## Privacy and security

Use synthetic or safely anonymized examples. Never attach real customer files, private correspondence, credentials, access tokens or personal information. Do not execute imported macros, scripts or external data links without an explicit reviewed security design.

A private vulnerability-reporting route must be established before public binaries. Do not publish exploitable private information in an ordinary issue.

## Working together

Follow the [Code of Conduct](CODE_OF_CONDUCT.md). People of every background, including LGBTQ+ people, are welcome. Prefer patient explanations, specific feedback and respectful disagreement.

There is no service-level or response-time promise. The owner is [Anton Perchenko](https://github.com/PerchAnton).
