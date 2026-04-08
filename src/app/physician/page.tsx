import type { Metadata } from "next";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { practiceUrl, allProfileUrls } from "@/lib/links";
import { buildPageMetadata } from "@/lib/metadata";
import { SITE, AUTHOR, PRACTICE } from "@/lib/config";

export const metadata: Metadata = buildPageMetadata({
  title: "Physician",
  description:
    "Board-certified plastic surgeon specializing in slow-aging, scar treatment, and natural blepharoplasty. Calibrated for long-term results, not quick fixes.",
  path: "/physician",
});

const clinicalFocus = [
  {
    name: "Slow-aging",
    description:
      "Knowing when a device is enough, when injectables are right, and when surgery is the honest answer. Calibrated for long-term results over quick fixes. I plan treatment as an ongoing relationship, not a one-off session.",
  },
  {
    name: "Scars",
    description:
      "Every scar has its own timeline and its own answer. Lasers, devices, injectables, or surgery — I match the tool to the stage. Covering the full lifecycle from fresh wounds to mature scars.",
  },
  {
    name: "Natural Eyes",
    description:
      "Specializing in nonincisional blepharoplasty at a level few surgeons attempt. Minimal, precise, and designed to look like nothing was done at all. Results over speed, every time.",
  },
];

export default function PhysicianPage() {
  const breadcrumbSchema = buildBreadcrumbSchema([
    { name: "Home", href: "/" },
    { name: "Physician", href: "/physician" },
  ]);

  const physicianSchema = {
    "@context": "https://schema.org",
    "@type": ["Person", "Physician"],
    name: AUTHOR.name,
    alternateName: AUTHOR.korean,
    jobTitle: AUTHOR.jobTitle,
    url: SITE.url,
    worksFor: {
      "@type": "MedicalBusiness",
      name: PRACTICE.fullName,
      url: practiceUrl,
      telephone: PRACTICE.phone,
    },
    medicalSpecialty: "PlasticSurgery",
    sameAs: allProfileUrls,
  };

  const practiceSchema = {
    "@context": "https://schema.org",
    "@type": "MedicalBusiness",
    name: PRACTICE.fullName,
    url: practiceUrl,
    telephone: PRACTICE.phone,
    address: {
      "@type": "PostalAddress",
      addressLocality: "Gangnam-gu",
      addressRegion: "Seoul",
      addressCountry: "KR",
    },
    medicalSpecialty: "PlasticSurgery",
    founder: {
      "@type": "Person",
      name: AUTHOR.name,
      url: SITE.url,
    },
  };

  return (
    <div className="mx-auto max-w-3xl px-6">
      <JsonLd data={breadcrumbSchema} />
      <JsonLd data={physicianSchema} />
      <JsonLd data={practiceSchema} />
      {/* Header */}
      <section className="pt-24 pb-12 sm:pt-32 sm:pb-16">
        <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">
          Physician
        </h1>
        <p className="mt-4 text-lg text-muted leading-relaxed max-w-xl">
          The right treatment at the right time.
        </p>
      </section>

      {/* Clinical Philosophy */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Clinical Philosophy
        </h2>
        <div className="space-y-5 text-foreground/90 leading-relaxed">
          <p>
            I practice both surgery and non-surgical medicine in depth.
            That means the recommendation is always based on what a patient
            actually needs — not limited by what I happen to offer.
          </p>
          <p>
            Better outcomes take more time. I&apos;d rather see someone regularly
            over years, adjusting as they change, than chase a single dramatic
            result. If a treatment is not needed, I say so.
          </p>
        </div>
      </section>

      {/* Clinical Focus */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Clinical Focus
        </h2>
        <div className="space-y-3">
          {clinicalFocus.map((area) => (
            <div
              key={area.name}
              className="rounded-2xl border border-border bg-card p-5"
            >
              <h3 className="text-base font-semibold text-foreground">
                {area.name}
              </h3>
              <p className="mt-2 text-sm text-muted leading-relaxed">
                {area.description}
              </p>
            </div>
          ))}
        </div>
      </section>

      {/* Practice */}
      <section className="pb-24">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Practice
        </h2>
        <div className="rounded-2xl border border-border bg-card p-6">
          <h3 className="text-lg font-semibold">{PRACTICE.name}</h3>
          <p className="mt-1 text-sm text-muted">{PRACTICE.location}</p>
          <p className="mt-1 text-sm text-muted">{PRACTICE.phone}</p>
          <a
            href={practiceUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="mt-4 inline-block text-sm text-accent hover:underline"
          >
            Visit LEAD Plastic Surgery Clinic &rarr;
          </a>
        </div>
      </section>
    </div>
  );
}
