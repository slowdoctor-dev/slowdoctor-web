import type { Metadata } from "next";
import Link from "next/link";
import { socialLinks, academicLinks, practiceUrl } from "@/lib/links";

export const metadata: Metadata = {
  title: "Links",
  description:
    "Professional and social links for Joonho Lim -- medical profiles, social media, and practice.",
};

const sections = [
  {
    title: "Medical",
    links: academicLinks.map((l) => ({ label: l.label, url: l.url })),
  },
  {
    title: "Social",
    links: socialLinks.map((l) => ({
      label: `${l.label}${"handle" in l ? ` ${l.handle}` : ""}`,
      url: l.url,
    })),
  },
  {
    title: "Practice",
    links: [{ label: "LEAD Plastic Surgery", url: practiceUrl }],
  },
];

export default function LinksPage() {
  return (
    <div className="mx-auto max-w-3xl px-6">
      {/* Header */}
      <section className="pt-24 pb-12 sm:pt-32 sm:pb-16">
        <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">
          Links
        </h1>
      </section>

      {/* Link Sections */}
      <div className="pb-16 space-y-12">
        {sections.map((section) => (
          <section key={section.title}>
            <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-4">
              {section.title}
            </h2>
            <ul className="space-y-2">
              {section.links.map((link) => (
                <li key={link.label}>
                  <a
                    href={link.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="block py-2 text-foreground hover:text-accent transition-colors"
                  >
                    {link.label}
                    <span className="ml-1 text-muted">↗</span>
                  </a>
                </li>
              ))}
            </ul>
          </section>
        ))}

        {/* Blog */}
        <section>
          <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-4">
            Blog
          </h2>
          <Link
            href="/blog"
            className="block py-2 text-foreground hover:text-accent transition-colors"
          >
            Blog &rarr;
          </Link>
        </section>
      </div>
    </div>
  );
}
