# REVIEW-CODEX

Date: 2026-04-11

## Critical

None.

## High

None.

## Medium

### Fixed

- `src/components/json-ld.tsx:9`
  - `JsonLd` injected raw `JSON.stringify(data)` into a `<script type="application/ld+json">`.
  - If any future title, description, or schema field contains `<`-based script-terminating content such as `</script>`, the JSON-LD block can break early and create an injection/XSS footgun.
  - Fixed by escaping `<`, U+2028, and U+2029 before writing the script payload.

## Low

### Fixed

- `scripts/convert-md.cts:167`
  - Frontmatter parsing relied on loose casts and `Record<string, unknown>`-style handling, which made `date`, `title`, `description`, `tags`, and `axes` access less defensible than necessary.
  - Fixed by adding explicit frontmatter interfaces and a dedicated `parseAxesFrontmatter()` validator.

- `src/components/axis-bar.tsx:11`
  - The filled bar segment was decorative but exposed to assistive tech alongside the textual label and numeric value.
  - Fixed by hiding the decorative meter from the accessibility tree while keeping the human-readable labels and numbers intact.

### Need Human Decision

None.

## Verification

- `npx tsc --noEmit`
- `npm run build`
- `node --experimental-strip-types scripts/validate.cts`

## Notes

- No critical or high-severity issues were found in the current codebase after the full source and build-path review.
