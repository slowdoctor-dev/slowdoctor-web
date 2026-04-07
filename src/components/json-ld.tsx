/**
 * JSON-LD structured data component for SEO.
 *
 * This uses dangerouslySetInnerHTML intentionally -- it is the standard
 * Next.js pattern for embedding JSON-LD structured data in the document head.
 * The data parameter is always a hardcoded constant defined at build time,
 * never derived from user input, so there is no XSS risk.
 *
 * Reference: https://nextjs.org/docs/app/building-your-application/optimizing/metadata#json-ld
 */
export function JsonLd({ data }: { data: Record<string, unknown> }) {
  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: JSON.stringify(data) }}
    />
  );
}
