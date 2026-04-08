import type { Metadata } from "next";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { socialLinks, medicalLinks, practiceUrl } from "@/lib/links";
import { buildPageMetadata } from "@/lib/metadata";
import { PRACTICE } from "@/lib/config";

export const metadata: Metadata = buildPageMetadata({
  title: "Links",
  description:
    "Professional and social links for Joonho Lim \u2014 medical profiles, social media, and practice.",
  path: "/links",
});

export default function LinksPage() {
  const breadcrumbSchema = buildBreadcrumbSchema([
    { name: "Home", href: "/" },
    { name: "Links", href: "/links" },
  ]);

  return (
    <div className="mx-auto max-w-3xl px-6">
      <JsonLd data={breadcrumbSchema} />
      {/* Header */}
      <section className="pt-24 pb-12 sm:pt-32 sm:pb-16">
        <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">
          Links
        </h1>
        <p className="mt-4 max-w-xl text-lg leading-relaxed text-muted">
          Official profiles, social channels, and practice links for Joonho Lim.
        </p>
      </section>

      <div className="pb-24 space-y-10">
        {/* Practice */}
        <section>
          <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-4">
            Practice
          </h2>
          <div className="rounded-lg border border-border bg-card">
            <a
              href={practiceUrl}
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center justify-between px-5 py-4 hover:bg-accent-muted transition-colors rounded-lg"
            >
              <div>
                <p className="text-foreground font-medium">{PRACTICE.name}</p>
                <p className="text-sm text-muted">{PRACTICE.location}</p>
              </div>
              <span className="text-muted shrink-0 ml-4">&#8599;</span>
            </a>
          </div>
        </section>

        {/* Medical */}
        <section>
          <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-4">
            Medical
          </h2>
          <div className="rounded-lg border border-border bg-card divide-y divide-border">
            {medicalLinks.map((link) => (
              <a
                key={link.label}
                href={link.url}
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center justify-between px-5 py-3.5 hover:bg-accent-muted transition-colors first:rounded-t-lg last:rounded-b-lg"
              >
                <span className="text-foreground font-medium">
                  {link.label}
                </span>
                <span className="text-sm text-muted flex items-center gap-2">
                  {link.detail}
                  <span>&#8599;</span>
                </span>
              </a>
            ))}
          </div>
        </section>

        {/* Social */}
        <section>
          <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-4">
            Social (Korean)
          </h2>
          <div className="rounded-lg border border-border bg-card divide-y divide-border">
            {socialLinks.map((link) => (
              <a
                key={link.label}
                href={link.url}
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center justify-between px-5 py-3.5 hover:bg-accent-muted transition-colors first:rounded-t-lg last:rounded-b-lg"
              >
                <span className="text-foreground font-medium">
                  {link.label}
                </span>
                <span className="text-sm text-muted flex items-center gap-2">
                  {link.handle}
                  <span>&#8599;</span>
                </span>
              </a>
            ))}
          </div>
        </section>
      </div>
    </div>
  );
}
