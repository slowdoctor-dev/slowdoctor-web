type JsonLdValue =
  | string
  | number
  | boolean
  | null
  | readonly JsonLdValue[]
  | { [key: string]: JsonLdValue };

function stringifyJsonLd(data: JsonLdValue) {
  return JSON.stringify(data)
    .replace(/</g, "\\u003c")
    .replace(/\u2028/g, "\\u2028")
    .replace(/\u2029/g, "\\u2029");
}

/**
 * JSON-LD structured data component for SEO.
 *
 * This uses dangerouslySetInnerHTML intentionally -- it is the standard
 * Next.js pattern for embedding JSON-LD structured data in the document head.
 * The payload is escaped so content cannot terminate the script tag early.
 *
 * Reference: https://nextjs.org/docs/app/building-your-application/optimizing/metadata#json-ld
 */
export function JsonLd({ data }: { data: JsonLdValue }) {
  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: stringifyJsonLd(data) }}
    />
  );
}
