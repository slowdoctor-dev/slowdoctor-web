import type { Metadata } from "next";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { practiceUrl } from "@/lib/links";
import { buildPageMetadata } from "@/lib/metadata";
import { PRACTICE } from "@/lib/config";

export const metadata: Metadata = buildPageMetadata({
  title: "Physician",
  description:
    "Board-certified plastic surgeon specializing in slow-aging, scar treatment, and blepharoplasty.",
  path: "/physician",
});

const clinicalFocus = [
  {
    name: "Slow-aging",
    description:
      "Non-surgical facial rejuvenation using energy-based devices and injectables, calibrated for long-term results over quick fixes. Management is planned in quarterly cycles over years, not one-off sessions.",
  },
  {
    name: "Scars",
    description:
      "Evidence-based scar treatment combining lasers, microneedling, and subcision. A plastic surgeon's understanding of tissue behavior meets non-surgical precision -- covering the full scar lifecycle from fresh wounds to mature scars.",
  },
  {
    name: "Natural Eyes",
    description:
      "Blepharoplasty that preserves individuality. Minimal, precise, and designed to look like nothing was done at all. Result over speed, every time.",
  },
];

export default function PhysicianPage() {
  const breadcrumbSchema = buildBreadcrumbSchema([
    { name: "Home", href: "/" },
    { name: "Physician", href: "/physician" },
  ]);

  return (
    <div className="mx-auto max-w-3xl px-6">
      <JsonLd data={breadcrumbSchema} />
      {/* Header */}
      <section className="pt-24 pb-12 sm:pt-32 sm:pb-16">
        <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">
          Physician
        </h1>
        <p className="mt-4 text-lg text-muted leading-relaxed max-w-xl">
          Choosing the right way over the fast way.
        </p>
      </section>

      {/* Clinical Philosophy */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Clinical Philosophy
        </h2>
        <div className="space-y-5 text-foreground/90 leading-relaxed">
          <p>
            I practice slow-aging, not anti-aging. The goal is not to reverse
            time but to age well -- gracefully, naturally, and at the right
            pace. Every face has its own timeline, and my work is about
            respecting that timeline while guiding it in a better direction.
          </p>
          <p>
            In a field driven by trends and speed, I choose the slower path.
            Better outcomes take more time. That patience is the foundation of
            everything I do.
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
              className="rounded-lg border border-border bg-card p-5"
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

      {/* Approach */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Approach
        </h2>
        <div className="space-y-5 text-foreground/90 leading-relaxed">
          <p>
            Most plastic surgeons specialize in either surgery or non-surgical
            procedures. I practice both at depth. This means I can assess
            whether a patient genuinely needs an operation or whether a
            well-calibrated device treatment would serve them better -- without
            the bias that comes from only knowing one side.
          </p>
          <p>
            I plan care over quarters and years, not single visits. If a
            treatment is not needed, I say so. The goal is the smallest
            effective intervention at the right time -- not the most expensive
            one, not the most popular one.
          </p>
        </div>
      </section>

      {/* Practice */}
      <section className="pb-24">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Practice
        </h2>
        <div className="rounded-lg border border-border bg-card p-6">
          <h3 className="text-lg font-semibold">{PRACTICE.name}</h3>
          <p className="mt-1 text-sm text-muted">{PRACTICE.location}</p>
          <p className="mt-1 text-sm text-muted">{PRACTICE.phone}</p>
          <a
            href={practiceUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="mt-4 inline-block text-sm text-accent hover:underline"
          >
            leadps.co.kr &rarr;
          </a>
        </div>
      </section>
    </div>
  );
}
