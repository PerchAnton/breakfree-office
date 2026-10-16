# BreakFree — Brand guide

## Identity

| Element | Approved direction |
| :--- | :--- |
| Family | **BreakFree** |
| Office suite | **BreakFree Office** |
| Applications | **BreakFree Docs**, **BreakFree Sheets**, **BreakFree Slides** |
| Main slogan | **Your work. Your freedom.** |
| Supporting line | Documents. Numbers. Ideas. Yours. |

Keep the capital B and F in BreakFree. Repository names use lowercase with hyphens: `breakfree-office`, `breakfree-docs`, `breakfree-sheets`, `breakfree-slides`.

## The approved mark

<img src="docs/brand/breakfree-icon.webp" alt="Approved BreakFree Office icon" width="192" height="192">

The mark combines three office documents on a deep aubergine rounded tile. Text, a spreadsheet grid and a chart communicate productivity. A small rainbow ribbon expresses freedom, diversity and LGBTQ+ inclusion without making the product look like a music application.

**Use the office-document mark, not the earlier musical-note concept.** Do not substitute the approved design with a different icon.

The repository includes a **192 × 192 WebP display derivative** of the approved raster image at `docs/brand/breakfree-icon.webp`. It is not a vector master or a complete platform-icon set. Keep the original high-resolution artwork separately for future packaging. Do not upscale this small derivative for app-store artwork.

## Proposed interface palette

These are design tokens for future interface work, not a claim that the raster artwork uses exactly these values.

| Token | Value | Intended use |
| :--- | :--- | :--- |
| Aubergine | `#241337` | Brand backdrop |
| Violet | `#8055C8` | Docs and primary accents |
| Teal | `#248A83` | Sheets accents |
| Coral | `#D86B63` | Slides accents |
| Paper | `#FFFDF8` | Light document surfaces |

The rainbow is a small identity accent, not the default background for controls. Check contrast and never communicate application state through color alone.

## Presentation

Use the shared icon above each repository title. Pair it with the application name so people know which product they are viewing. Prefer generous spacing, concise descriptions and meaningful screenshots once working software exists. Label interface concepts as concepts, not product screenshots.

The musical reference belongs to the name and its idea of freedom. Do not copy Queen's crest, Microsoft product icons or another project's branding, and do not imply endorsement.

## Voice

Clear, welcoming and confident. Explain what a user can do, distinguish plans from tested features and state limitations plainly. Public project documentation is in English. Ukrainian and Russian localization can be planned alongside other languages after the application architecture is decided.

## Ownership and licensing

The project owner has not selected a distribution license for software or brand assets. Do not add a license badge, claim trademark registration or grant third-party reuse rights without the owner's decision.

[Back to BreakFree Office](README.md)

## Owner-mandated desktop UI direction

The family targets Windows desktop Office 2024 Start and File/Backstage density,
layout and typographic hierarchy while retaining all BreakFree names, logos and
identity. Use the light neutral shell palette in [visual system v1](docs/visual/v1/README.md);
the earlier proposed brand palette is not a directive to paint large UI surfaces
violet. Keep rainbow accents in approved transparent marks. Shell colors and
document-content themes are separate. Load system fonts at runtime or use
licensed fallbacks; never distribute Microsoft fonts, artwork or templates.
Exact Office 2024 reference proof and Windows scale acceptance remain required,
not inferred from the appearance of a toolkit demo.
